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
    sys.block_on(::common::futures::future::lazy(|| {
        ::tokio_timer::sleep(::std::time::Duration::from_millis(1))
    })).unwrap();
}


impl MapView {
    /// Initialization
    pub fn new(f: &Display) -> Self {
        let mut sys = System::new("Map");
        let (tx, rx) = channel();
        let mut _impl = MapViewImpl::new(f, tx);
        let addr: Addr<MapViewImpl> = _impl.start();
        pulse(&mut sys);
        pulse(&mut sys);
        let ptr = rx.recv().unwrap();

        return MapView {
            sys,
            addr: ptr,
        };
    }

    pub fn do_run<R: Send + 'static, F: FnOnce(&mut MapViewImpl, &mut Context<MapViewImpl>) -> R + 'static>(&mut self, f: F) -> R {
        use ::common::futures::future::*;
        use std::sync::Arc;
        use std::cell::RefCell;

        unsafe {
            let addr = self.addr;
            let add = (*self.addr).addr();
            let invoke: ForceSend<F> = ForceSend(f);

            let res = self.sys.block_on(::common::futures::future::lazy(|| {
                add.send(Invoke::new(move |m, c| {
                    (invoke.0)(m, c)
                }))
            })).unwrap().unwrap();
            res
        }
    }

    pub fn pulse(&mut self) {
        pulse(&mut self.sys)
    }

    pub fn render(&mut self, mut surface: glium::Frame) {
        self.do_run(move |map: &mut MapViewImpl, ctx| {
            map.render(&mut surface, ctx);
            surface.finish().unwrap();
        });
        self.pulse();
    }

    pub fn set_style_url(&mut self, url: &str) {
        let u = url.to_string();
        self.do_run(move |map: &mut MapViewImpl, ctx| {
            map.set_style_url(&u, ctx);
        });
    }

    pub fn window_resized(&mut self, dims: PixelSize) {
        self.do_run(move |map: &mut MapViewImpl, _| {
            map.window_resized(dims)
        });
    }

    pub fn mouse_moved(&mut self, pixel: PixelPoint) {
        self.do_run(move |map: &mut MapViewImpl, _| {
            map.handle_mouse_moved(pixel)
        });
    }

    pub fn mouse_button(&mut self, down: bool) {
        self.do_run(move |map: &mut MapViewImpl, _| {
            map.handle_mouse_button(down)
        });
    }

    pub fn mouse_scroll(&mut self, scroll: f64) {
        self.do_run(move |map: &mut MapViewImpl, _| {
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
    file_source: Addr<storage::DefaultFileSource>,

    facade: Box<glium::Display>,
    style: Option<Rc<style::Style>>,
    gui: Gui,
    input: InputStatus,

}

use common::actix_web::actix::fut::*;


impl Actor for MapViewImpl {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let ptr = self as *mut _;
        self.tx.send(ptr).unwrap();

        self.addr = Some(ctx.address());
    }
}


impl MapViewImpl {
    pub fn addr(&self) -> Addr<Self> {
        self.addr.as_ref().map(|x| x.clone()).unwrap()
    }
    pub fn new(f: &Display, tx: Sender<*mut Self>) -> Self {
        let src_add = storage::DefaultFileSource::spawn();

        let mut camera: Camera = Default::default();
        camera.pos = Mercator::latlng_to_world(LatLng::new(49, 16));

        let m = MapViewImpl {
            tx,
            addr: None,
            camera,
            renderer: None,
            file_source: src_add.clone(),
            gui: Gui::new(f).unwrap(),
            facade: Box::new((*f).clone()),
            style: None,
            input: Default::default(),
        };


        return m;
    }

    pub fn set_style(&mut self, style: style::Style, ctx: &mut Context<MapViewImpl>) {
        trace!("MapViewImpl: Setting style ..");
        let style = Rc::new(style);
        self.renderer = Some(render::Renderer::new(&self.facade, style.clone(), self.file_source.clone()));
        if let Some(ref sprite) = style.as_ref().sprite {
            let image = storage::Request::SpriteImage(format!("{}", sprite));
            let json = storage::Request::SpriteJson(format!("{}", sprite));


            let img =
                wrap_future(self.file_source.send(image))
                    .from_err::<Error>()
                    .map(|res, this: &mut MapViewImpl, ctx| {
                        trace!("MapViewImpl: Retrieved sprite image .. : {:?}", res);
                        this.renderer.as_mut().unwrap().sprite_png_ready(res.unwrap().data);
                    });

            let js =
                wrap_future(self.file_source.send(json))
                    .from_err::<Error>()
                    .map(|res, this: &mut MapViewImpl, ctx| {
                        trace!("MapViewImpl: Retrieved sprite json .. : {:?}", res);

                        let parsed: Result<style::sprite::SpriteAtlas> = res
                            .map_err(|e| e.into())
                            .and_then(|x| {
                                json::from_slice(&x.data[..]).map_err(|e| e.into())
                            });

                        trace!("MapViewImpl: Parsed sprite JSON : {:?}", parsed);
                        this.renderer.as_mut().unwrap().sprite_json_ready(parsed.unwrap());
                    });

            ctx.spawn(img.drop_err());
            ctx.spawn(js.drop_err());
        }
        self.style = Some(style);
    }


    pub fn set_style_url(&mut self, url: &str, ctx: &mut Context<MapViewImpl>) {
        trace!("Setting style URL");
        let req = storage::resource::Request::style(url.into());

        let send_fut = wrap_future(self.file_source.send(req));
        let data_fut = send_fut.from_err::<Error>();
        let work_fut = data_fut
            .map(|res, this: &mut Self, ctx| {
                match res {
                    Ok(resource) => {
                        let parsed = json::from_slice(&resource.data).unwrap();
                        this.set_style(parsed, ctx);
                    }
                    Err(e) => {
                        panic!("Could not retrieve style data : {:?}", e);
                    }
                }
            });

        ctx.spawn(work_fut.drop_err());
    }

    pub fn window_resized(&mut self, dims: PixelSize) {
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


    pub fn render(&mut self, target: &mut glium::Frame, ctx: &mut Context<Self>) {
        profiler::begin("Render");
        let params = self::render::RendererParams {
            display: self.facade.deref(),
            frame: target,
            camera: &self.camera,
            ctx: ctx,
            frame_start: PreciseTime::now(),
        };

        if let Some(ref mut render) = self.renderer {
            render.render(params).unwrap();
        }
        profiler::end();
    }

    pub fn new_tile(&mut self, tile: tiles::TileData, ctx: &mut Context<MapViewImpl>) {
        if let Some(ref mut r) = self.renderer {
            r.tile_ready(Rc::new(tile));
        }
    }
}

impl_invoke_handler!(MapViewImpl);