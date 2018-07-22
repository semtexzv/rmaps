use prelude::*;

pub mod gui;
pub mod input;
pub mod render;

pub mod style;
pub mod storage;
pub mod tiles;

pub mod util;

use std::ptr;
use std::sync::mpsc::{channel, Sender, Receiver};

use self::util::profiler;

pub struct MapView {
    addr: *mut MapViewImpl,
    sys: SystemRunner,
}

fn pulse(sys: &mut SystemRunner) {
    sys.run_until_complete(::common::futures::future::lazy(|| {
        ::tokio_timer::sleep(::std::time::Duration::from_millis(1))
    })).unwrap();
}

impl MapView {
    pub fn new(f: &Display) -> Self {
        let mut sys = System::new("Map");
        let (tx, rx) = channel();
        let mut _impl = MapViewImpl::new(f, tx);
        let addr: Addr<MapViewImpl> = _impl.start();
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

        let addr = self.addr;
        let res = unsafe {
            self.sys.run_until_complete(::common::futures::future::lazy(|| {
                ok::<_, ()>(f(&mut (*addr)))
            })).unwrap()
        };

        res
    }

    pub fn pulse(&mut self) {
        pulse(&mut self.sys)
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

    pub fn window_resized(&mut self, dims: PixelSize) {
        self.do_run(|map: &mut MapViewImpl| {
            map.window_resized(dims)
        });
    }

    pub fn mouse_moved(&mut self, pixel: PixelPoint) {
        self.do_run(|map: &mut MapViewImpl| {
            map.handle_mouse_moved(pixel)
        });
    }

    pub fn mouse_button(&mut self, down: bool) {
        self.do_run(|map: &mut MapViewImpl| {
            map.handle_mouse_button(down)
        });
    }

    pub fn mouse_scroll(&mut self, scroll: f64) {
        self.do_run(|map: &mut MapViewImpl| {
            map.handle_mouse_scroll(scroll)
        });
    }
}

use self::input::InputHandler;
use self::gui::Gui;

#[derive(Default)]
pub struct InputStatus {
    last_pos: PixelPoint,
    captured: bool,
}


impl<'a> InputHandler for MapViewImpl {
    fn has_captured(&mut self) -> bool {
        return self.input.captured;
    }

    fn mouse_moved(&mut self, pixel: PixelPoint) -> bool {
        if self.input.captured {
            let last = self.input.last_pos;
            let last = self.camera.window_to_world(last);
            let now = self.camera.window_to_world(pixel);

            let diff = now - last;
            self.camera.set_pos(self.camera.pos() - diff);
        }

        self.input.last_pos = pixel;
        self.has_captured()
    }

    fn mouse_button(&mut self, pressed: bool) -> bool {
        self.input.captured = pressed;
        self.has_captured()
    }

    fn mouse_scroll(&mut self, scroll: f64) -> bool {
        self.camera.set_zoom(self.camera.zoom + scroll as f32);
        self.has_captured()
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

    gui: Gui,
    input: InputStatus,

}

impl MapViewImpl {
    pub fn addr(&self) -> Addr<Self> {
        self.addr.as_ref().map(|x| x.clone()).unwrap()
    }
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
            gui: Gui::new(f).unwrap(),
            facade: Box::new((*f).clone()),
            style: None,
            tile_loader: tile_loader,
            input: Default::default(),
        };


        return m;
    }

    pub fn set_style(&mut self, style: style::Style) {
        let style = Rc::new(style);
        self.renderer = Some(render::Renderer::new(&self.facade, style.clone()));
        if let Some(ref sprite) = style.as_ref().sprite {
            let image = storage::Request::SpriteImage(format!("{}", sprite));
            let json = storage::Request::SpriteJson(format!("{}", sprite));

            let cb = self.addr().recipient();
            spawn(self.source.send(storage::ResourceRequest::new(image, cb.clone())));
            spawn(self.source.send(storage::ResourceRequest::new(json, cb)))
        }
        for (n, v) in style.sources.iter() {
            if let Some(ref url) = v.deref().url {
                let cb = self.addr().recipient();
                let rq = storage::Request::SourceJson(n.to_string(), url.to_string());
                spawn(self.source.send(storage::ResourceRequest::new(rq, cb)));
            }
        }
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

    pub fn window_resized(&mut self, dims: PixelSize) {
        trace!("Resized: {:?}", dims);
        self.camera.set_size(dims);
    }


    pub fn handle_mouse_moved(&mut self, pixel: PixelPoint) {
        if self.gui.has_captured() {
            self.gui.mouse_moved(pixel);
        } else if self.has_captured() {
            self.mouse_moved(pixel);
        } else {
            self.gui.mouse_moved(pixel);
            self.mouse_moved(pixel);
        }
    }
    pub fn handle_mouse_button(&mut self, down: bool) {
        self.gui.mouse_button(down);
        self.mouse_button(down);
    }

    pub fn handle_mouse_scroll(&mut self, scroll: f64) {
        if self.gui.has_captured() {
            self.gui.mouse_scroll(scroll);
        } else if self.has_captured() {
            self.mouse_scroll(scroll);
        }

        if !self.gui.mouse_scroll(scroll) {
            self.mouse_scroll(scroll);
        }
    }


    pub fn render(&mut self, target: &mut glium::Frame) {
        profiler::begin("Render");
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
        profiler::end();
        //  self.gui.render(target)
    }
}


impl Actor for MapViewImpl {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let ptr = self as *mut _;
        self.tx.send(ptr).unwrap();

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
            storage::Request::SourceJson(name, url) => {
                let parsed: Result<style::TileJson> = msg.result
                    .map_err(|e| e.into())
                    .and_then(|x| {
                        json::from_slice(&x.data[..]).map_err(|e| e.into())
                    });
                // TODO, somehow propagate this info to appropriate style source
            }
            storage::Request::SpriteJson(req) => {
                if let Some(ref mut r) = self.renderer {
                    let parsed: Result<style::sprite::SpriteAtlas> = msg.result
                        .map_err(|e| e.into())
                        .and_then(|x| {
                            json::from_slice(&x.data[..]).map_err(|e| e.into())
                        });

                    r.sprite_json_ready(parsed.unwrap());
                }
            }
            storage::Request::SpriteImage(req) => {
                if let Some(ref mut r) = self.renderer {
                    r.sprite_png_ready(msg.result.unwrap().data);
                }
            }
            storage::Request::StyleJson(req) => {
                let parsed: Result<style::Style> = msg.result
                    .map_err(|e| e.into())
                    .and_then(|x| {
                        json::from_slice(&x.data[..]).map_err(|e| e.into())
                    });
                self.set_style(parsed.unwrap());
            }
            _ => {
                panic!("Shouldnt happen");
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

impl_invoke_handler!(MapViewImpl);