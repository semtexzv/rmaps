use prelude::*;

pub mod render;

pub mod style;
pub mod storage;
pub mod tiles;

use std::ptr;
use std::sync::mpsc::{channel, Sender, Receiver};

pub struct MapView {
    addr: *mut MapViewImpl,
    sys: SystemRunner,
}

fn pulse(sys: &mut SystemRunner) {
    sys.block_on(::common::futures::future::lazy(|| {
        ::tokio_timer::sleep(::std::time::Duration::from_millis(1))
    })).unwrap();
}

impl MapView {
    pub fn new(f: &Display) -> Self {
        let mut sys = System::new("Map");
        let (tx, rx) = channel();
        let mut _impl = MapViewImpl::new(f, tx);
        let addr = _impl.start();
        pulse(&mut sys);
        pulse(&mut sys);
        println!("Receiving");
        let ptr = rx.recv().unwrap();
        println!("Received");

        return MapView {
            sys,
            addr: ptr,
        };
    }


    pub fn do_run<R>(&mut self, f: impl FnOnce(&mut MapViewImpl) -> R) -> R {
        use ::common::futures::future::*;

        let res = unsafe {
            let addr = self.addr;
            self.sys.block_on(lazy(|| {
                ok::<_, ()>(f(&mut (*addr)))
            }))
        }.unwrap();

        res
    }

    pub fn pulse(&mut self) {
        self.sys.block_on(::common::futures::future::lazy(|| {
            ::tokio_timer::sleep(::std::time::Duration::from_millis(3))
        })).unwrap();
    }

    pub fn render(&mut self, mut surface: glium::Frame) {
        self.do_run(|map: &mut MapViewImpl| {
            map.render(&mut surface);
        });
        surface.finish().unwrap();
        self.pulse();
        //info!("Render")
    }

    pub fn set_style_url(&mut self, url: &str) {
        self.do_run(|map: &mut MapViewImpl| {
            map.set_style_url(url);
        });
    }

    pub fn get_camera(&mut self) -> Camera {
        self.do_run(|map: &mut MapViewImpl| {
            map.camera.clone()
        })
    }

    pub fn set_camera(&mut self, camera: Camera) {
        self.do_run(|map: &mut MapViewImpl| {
            map.camera = camera;
        });
    }

    pub fn clicked(&mut self, point: PixelPoint) {
        self.do_run(|map: &mut MapViewImpl| {
            map.clicked(point)
        });
    }
}

pub struct MapViewImpl {
    tx: Sender<*mut MapViewImpl>,

    addr: Option<Addr<MapViewImpl>>,

    camera: Camera,
    renderer: Option<render::Renderer>,

    source: Addr<storage::DefaultFileSource>,
    tile_loader: Addr<tiles::TileLoader>,

    facade: Box<glium::Display>,
    style: Option<Rc<style::Style>>,

}

impl MapViewImpl {
    pub fn new(f: &Display, tx: Sender<*mut Self>) -> Self {
        let src_add = storage::DefaultFileSource::spawn();
        let tile_loader = tiles::TileLoader::spawn(src_add.clone());

        let mut camera: Camera = Default::default();
        camera.pos = Mercator::latlng_to_world(LatLng::new(49, 16));

        let m = MapViewImpl {
            tx,
            addr: None,

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
        println!("Style uRL");
        let req = storage::resource::Request::style(url.into());
        let addr: Addr<MapViewImpl> = self.addr.clone().unwrap().into();
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
        unsafe {
            let ptr = self as *mut _;
            self.tx.send(ptr).unwrap();
        }
        self.addr = Some(ctx.address());
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