use prelude::*;

pub mod render;

pub mod style;
pub mod storage;
pub mod tiles;


pub struct MapView {
    addr: Addr<Unsync, MapViewImpl>,
    sys: SystemRunner,
}

impl MapView {
    pub fn new(f: &Display) -> Self {
        let sys = System::new("Map");
        let _impl = MapViewImpl::new(f);

        let addr = _impl.start();

        return MapView {
            sys,
            addr,
        };
    }

    pub fn do_run<R>(&mut self, f: impl FnOnce(Addr<Unsync, MapViewImpl>) -> R) -> R {
        let addr = self.addr.clone();
        let res = self.sys.run_until_complete(::common::futures::future::lazy(|| {
            Ok::<R, !>(f(addr))
        }));
        self.sys.pulse();
        res.unwrap()
    }

    pub fn render(&mut self, surface: glium::Frame) {
        self.do_run(|add| {
            add.do_send(MapMethodArgs::Render(surface));
        });
    }

    pub fn set_style_url(&mut self, url: &str) {
        self.do_run(|add| {
            add.do_send(MapMethodArgs::SetStyleUrl(url.into()));
        });
    }

    pub fn get_camera(&mut self) -> Camera {
        self.do_run(|add| {
            add.send(Invoke::new(|i: &mut MapViewImpl| {
                i.camera.clone()
            }))
        }).wait().unwrap().unwrap()
    }

    pub fn set_camera(&mut self, camera: Camera) {
        self.do_run(|add| {
            add.send(Invoke::new(|i: &mut MapViewImpl| {
                i.camera = camera;
            }))
        }).wait().unwrap().unwrap()
    }
    // TODO, what coordinate system ?
    pub fn clicked(&mut self, point: PixelPoint) {
        self.do_run(|add| {
            add.send(Invoke::new(move |i: &mut MapViewImpl| {
                i.clicked(point);
            }))
        }).wait().unwrap().unwrap()
    }
}

pub struct MapViewImpl {
    addr: Option<Addr<Unsync, MapViewImpl>>,
    sync_addr: Option<Addr<Syn, MapViewImpl>>,

    camera: Camera,
    renderer: render::Renderer,

    source: Addr<Syn, storage::DefaultFileSource>,
    tile_worker: Addr<Syn, tiles::data::TileDataWorker>,
    facade: Box<glium::Display>,
    style: Option<style::Style>,
    tile_storage: tiles::TileStorage,
}

impl Actor for MapViewImpl {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut <Self as Actor>::Context) {
        self.addr = Some(ctx.address());
        self.sync_addr = Some(ctx.address());
    }
}

use self::tiles::data;

impl Handler<storage::ResourceCallback> for MapViewImpl {
    type Result = ();

    fn handle(&mut self, msg: storage::ResourceCallback, _ctx: &mut Context<Self>) {
        match msg.0 {
            Ok(res) => {
                match res.req.data {
                    storage::RequestData::StyleJson { .. } => {
                        let parsed = json::from_slice(&res.data).unwrap();
                        self.set_style(parsed);
                    }
                    storage::RequestData::Tile(storage::TileRequestData { coords, ref source, .. }) => {
                        self.tile_storage.finished_tile(&coords);
                        let source_data = self.style.as_ref().unwrap().sources.get(source).unwrap().clone();

                        let rq = tiles::data::DecodeTile {
                            source: source_data,
                            source_name: source.clone(),
                            res: res.clone(),
                            cb: self.sync_addr.as_ref().unwrap().clone().recipient(),
                        };
                        self.tile_worker.do_send(rq);
                    }
                    _ => {
                        panic!("Resource {:?}", res);
                    }
                }
            }
            Err(_e) => {
                //   panic!("Resource request failed : {:?}", e)
            }
        }
    }
}

impl Handler<tiles::data::TileReady> for MapViewImpl {
    type Result = ();

    fn handle(&mut self, msg: tiles::data::TileReady, ctx: &mut Context<Self>) {
        let data = Rc::new(msg.data);
        self.renderer.tile_ready(data);
    }
}

impl MapViewImpl {
    pub fn new(f: &Display) -> Self {
        let src_add = storage::DefaultFileSource::spawn();
        let tile_worker_add = tiles::data::TileDataWorker::spawn();

        let mut camera: Camera = Default::default();
        camera.pos = Mercator::latlng_to_world(LatLng::new(49, 16));
        // camera.pos = Mercator::latlng_to_point(LatLng::new(-26,137));

        let m = MapViewImpl {
            addr: None,
            sync_addr: None,

            camera,
            renderer: render::Renderer::new(&f),

            source: src_add,
            tile_worker: tile_worker_add,

            facade: Box::new((*f).clone()),
            style: None,
            tile_storage: tiles::TileStorage::new(),
        };


        return m;
    }

    pub fn set_style(&mut self, style: style::Style) {
        //trace!("Style changed {:#?}", style);
        self.renderer.style_changed(&style).unwrap();
        self.style = Some(style);
    }

    pub fn set_style_url(&mut self, url: &str) {
        println!("Setting style url : {:?}", url);

        let req = storage::Request::style(url.into());
        let addr: Addr<Syn, MapViewImpl> = self.sync_addr.clone().unwrap().into();
        self.source.do_send(storage::ResourceRequest(req, addr.recipient()));
    }

    pub fn clicked(&mut self, pixel: PixelPoint) {
        println!("PIXEL : {:?}", pixel);
        let dev = self.camera.window_to_device(pixel);
        println!("DEVICE : {:?}", dev);
        let world = self.camera.device_to_world(dev);
        println!("WORLD : {:?}", world);
    }
    pub fn render(&mut self, target: &mut glium::Frame) {
        let projection = self.camera.projection();
        let view = self.camera.view();
        let params = self::render::RenderParams {
            disp: self.facade.deref(),
            frame: target,
            projection,
            view,
            camera: &self.camera,

            frame_start: PreciseTime::now(),
        };
        self.renderer.render(params).unwrap();

        if let Some(ref style) = self.style {
            for (src_id, src) in style.sources.iter() {
                let needed = self.tile_storage.needed_tiles();
                //  println!("Needed : {:?}\n in flight : {:?}", needed, self.tile_storage.in_flight);
                for coord in needed {
                    self.tile_storage.requested_tile(&coord);
                    let req = storage::Request::tile(src_id.clone(), src.url_template(), coord);
                    let addr: Addr<Syn, MapViewImpl> = self.sync_addr.clone().unwrap().into();
                    self.source.do_send(storage::ResourceRequest(req, addr.recipient()));
                }
            }
        }
    }
}


pub enum MapMethodArgs {
    Render(glium::Frame),
    SetStyleUrl(String),
}

impl Message for MapMethodArgs {
    type Result = ();
}

impl Handler<MapMethodArgs> for MapViewImpl {
    type Result = ();

    fn handle(&mut self, msg: MapMethodArgs, _ctx: &mut Self::Context) -> () {
        match msg {
            MapMethodArgs::Render(mut frame) => {
                self.render(&mut frame);
                frame.finish().unwrap();
            }
            MapMethodArgs::SetStyleUrl(url) => {
                self.set_style_url(&url)
            }
        };
    }
}

pub struct Invoke<A, F, R>
    where A: Actor,
          F: FnOnce(&mut A) -> R,
          R: 'static

{
    f: F,
    _a: ::std::marker::PhantomData<A>,
    _r: ::std::marker::PhantomData<R>,
}

impl<A, F, R> Invoke<A, F, R>
    where A: Actor,
          F: FnOnce(&mut A) -> R,
          R: 'static

{
    fn new(f: F) -> Self {
        Invoke {
            f: f,
            _a: ::std::marker::PhantomData,
            _r: ::std::marker::PhantomData,
        }
    }
}

impl<A, F, R> Message for Invoke<A, F, R>
    where A: Actor,
          F: FnOnce(&mut A) -> R,
          R: 'static
{
    type Result = Result<R>;
}

impl<F, R> Handler<Invoke<MapViewImpl, F, R>> for MapViewImpl
    where F: FnOnce(&mut MapViewImpl) -> R,
          R: 'static
{
    type Result = Result<R>;

    fn handle(&mut self, msg: Invoke<MapViewImpl, F, R>, _ctx: &mut Context<Self>) -> Result<R> {
        Ok((msg.f)(self))
    }
}