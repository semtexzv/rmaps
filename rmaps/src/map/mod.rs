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
    renderer: Option<render::Renderer>,

    source: Addr<Syn, storage::DefaultFileSource>,
    tile_loader: Addr<Syn, tiles::TileLoader>,

    facade: Box<glium::Display>,
    style: Option<Rc<style::Style>>,

}

impl MapViewImpl {
    pub fn new(f: &Display) -> Self {
        let src_add = storage::DefaultFileSource::spawn();
        let tile_loader = tiles::TileLoader::spawn(src_add.clone());

        let mut camera: Camera = Default::default();
        camera.pos = Mercator::latlng_to_world(LatLng::new(49, 16));

        let m = MapViewImpl {
            addr: None,
            sync_addr: None,

            camera,
            renderer: None,

            source: src_add.clone(),

            facade: Box::new((*f).clone()),
            style: None,
            tile_loader: tile_loader,
        };


        return m;
    }

    pub fn set_style(&mut self, style: style::Style) {
        let style = Rc::new(style);
        self.renderer = Some(render::Renderer::new(&self.facade, style.clone()));
        self.style = Some(style);
    }

    pub fn set_style_url(&mut self, url: &str) {
        let req = storage::Request::style(url.into());
        let addr: Addr<Syn, MapViewImpl> = self.sync_addr.clone().unwrap().into();
        spawn(self.source.send(storage::ResourceRequest {
            request: req,
            callback: addr.recipient(),
        }))
    }

    pub fn clicked(&mut self, pixel: PixelPoint) {
        println!("PIXEL : {:?}", pixel);
        let dev = self.camera.window_to_device(pixel);
        println!("DEVICE : {:?}", dev);
        let world = self.camera.device_to_world(dev);
        println!("WORLD : {:?}", world);
        let tile = world.tile_at_zoom(self.camera.zoom_int());
        println!("TILE : {:?}", tile);
    }

    pub fn render(&mut self, target: &mut glium::Frame) {
        let params = self::render::RendererParams {
            display: self.facade.deref(),
            frame: target,
            camera: &self.camera,
            loader: self.tile_loader.clone(),

            frame_start: PreciseTime::now(),
        };

        if let Some(ref mut render) = self.renderer {
            render.render(params).unwrap();
        }
    }
}


impl Actor for MapViewImpl {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut <Self as Actor>::Context) {
        self.addr = Some(ctx.address());
        self.sync_addr = Some(ctx.address());
        let add = ctx.address();
        self.tile_loader.do_send(Invoke::new(|x: &mut tiles::TileLoader| x.map = Some(add)));
    }
}

use self::tiles::data;

impl Handler<storage::ResourceResponse> for MapViewImpl {
    type Result = ();

    fn handle(&mut self, msg: storage::ResourceResponse, _ctx: &mut Context<Self>) {
        match msg.request {
            storage::Request::Tile(req) => {
                //self.tile_loader.tile_arrived(req, msg.result)
            }
            storage::Request::SourceJson(req) => {}
            storage::Request::StyleJson(req) => {
                let parsed: Result<style::Style> = msg.result
                    .map_err(|e| e.into())
                    .and_then(|x| {
                        json::from_slice(&x.data[..]).map_err(|e| e.into())
                    });
                self.set_style(parsed.unwrap());
            }
        }
    }
}

impl Handler<tiles::data::TileReady> for MapViewImpl {
    type Result = ();

    fn handle(&mut self, msg: tiles::data::TileReady, ctx: &mut Context<Self>) {
        let data = Rc::new(msg.data);
        if let Some(ref mut r) = self.renderer {
            r.tile_ready(data);
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


impl<F, R> Handler<Invoke<MapViewImpl, F, R>> for MapViewImpl
    where F: FnOnce(&mut MapViewImpl) -> R,
          R: 'static
{
    type Result = Result<R>;

    fn handle(&mut self, msg: Invoke<MapViewImpl, F, R>, _ctx: &mut Context<Self>) -> Result<R> {
        Ok((msg.f)(self))
    }
}