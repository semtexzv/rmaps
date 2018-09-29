use prelude::*;

//pub mod gui;
pub mod input;
pub mod render;

pub mod style;
pub mod storage;
pub mod tiles;

pub mod util;
pub mod hal;

use std::ptr;
use std::sync::mpsc::{channel, Sender, Receiver};
use common::actix::fut::*;

use self::util::profiler;

pub struct MapView<I: hal::Platform> {
    addr: *mut MapViewImpl<I>,
    sys: SystemRunner,
}

fn pulse(sys: &mut SystemRunner) {
    sys.block_on(::common::futures::future::lazy(|| {
        ::tokio_timer::sleep(::std::time::Duration::from_micros(1))
    }));
}


impl<I: hal::Platform> MapView<I> {
    pub fn new(f: &Display) -> Self {
        let mut sys = System::new("Map");
        let (tx, rx) = channel();
        let mut _impl = MapViewImpl::new(f, tx);
        let addr: Addr<MapViewImpl<I>> = _impl.start();
        pulse(&mut sys);
        pulse(&mut sys);
        let ptr = rx.recv().unwrap();

        return MapView {
            sys,
            addr: ptr,
        };
    }

    pub fn do_run<R: Send + 'static, F: FnOnce(&mut MapViewImpl<I>, &mut Context<MapViewImpl<I>>) -> R + 'static>(&mut self, f: F) -> R {
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
        self.do_run(move |map: &mut MapViewImpl<I>, ctx| {
            map.window_resized(PixelSize::new(surface.get_dimensions().0, surface.get_dimensions().1));
            map.render(&mut surface, ctx);
            surface.finish().unwrap();
        });
        self.pulse();
    }

    pub fn set_style_url(&mut self, url: &str) {
        let u = url.to_string();
        self.do_run(move |map: &mut MapViewImpl<I>, ctx| {
            map.set_style_url(&u, ctx);
        });
    }

    pub fn window_resized(&mut self, dims: PixelSize) {
        self.do_run(move |map: &mut MapViewImpl<I>, _| {
            map.window_resized(dims)
        });
    }

    pub fn mouse_moved(&mut self, pixel: PixelPoint) {
        println!("Self ptr : {:?}", self as *mut _);
        self.do_run(move |map: &mut MapViewImpl<I>, _| {
            map.handle_mouse_moved(pixel)
        });
    }

    pub fn mouse_button(&mut self, down: bool) {
        self.do_run(move |map: &mut MapViewImpl<I>, _| {
            map.handle_mouse_button(down)
        });
    }

    pub fn mouse_scroll(&mut self, scroll: f64) {
        self.do_run(move |map: &mut MapViewImpl<I>, _| {
            map.handle_mouse_scroll(scroll)
        });
    }
}

use self::input::InputHandler;

#[derive(Default)]
pub struct InputStatus {
    last_pos: PixelPoint,
    captured: bool,
}


impl<'a, I: hal::Platform> InputHandler for MapViewImpl<I> {
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

pub struct MapViewImpl<I: hal::Platform> {
    tx: Sender<*mut MapViewImpl<I>>,
    addr: Option<Addr<MapViewImpl<I>>>,
    camera: Camera,
    renderer: Option<render::Renderer>,
    file_source: Addr<storage::DefaultFileSource<I>>,

    facade: Box<glium::Display>,
    style: Option<Rc<style::Style>>,
    //gui: Gui,
    input: InputStatus,

}


impl<I: hal::Platform> Actor for MapViewImpl<I> {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let ptr = self as *mut _;
        self.tx.send(ptr).unwrap();

        self.addr = Some(ctx.address());
    }
}


impl<I: hal::Platform> MapViewImpl<I> {
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
            //gui: Gui::new(f).unwrap(),
            facade: Box::new((*f).clone()),
            style: None,
            input: Default::default(),
        };


        return m;
    }

    pub fn set_style(&mut self, style: style::Style, ctx: &mut Context<MapViewImpl<I>>) {
        trace!("MapViewImpl: Setting style ..");
        let style = Rc::new(style);
        self.renderer = Some(render::Renderer::new::<I>(&self.facade, style.clone(), self.file_source.clone().recipient()));
        if let Some(ref sprite) = style.as_ref().sprite {
            let image = storage::Request::SpriteImage(format!("{}", sprite));
            let json = storage::Request::SpriteJson(format!("{}", sprite));


            let img =
                wrap_future(self.file_source.send(image))
                    .from_err::<Error>()
                    .map(|res, this: &mut MapViewImpl<I>, ctx| {
                        //trace!("MapViewImpl: Retrieved sprite image .. : {:?}", res);
                        this.renderer.as_mut().unwrap().sprite_png_ready(res.unwrap().data);
                    });

            let js =
                wrap_future(self.file_source.send(json))
                    .from_err::<Error>()
                    .map(|res, this: &mut MapViewImpl<I>, ctx| {
                        //trace!("MapViewImpl: Retrieved sprite json .. : {:?}", res);

                        let parsed: Result<style::sprite::SpriteAtlas> = res
                            .map_err(|e| e.into())
                            .and_then(|x| {
                                json::from_slice(&x.data[..]).map_err(|e| e.into())
                            });

                        //trace!("MapViewImpl: Parsed sprite JSON : {:?}", parsed);
                        this.renderer.as_mut().unwrap().sprite_json_ready(parsed.unwrap());
                    });

            ctx.spawn(img.drop_err());
            ctx.spawn(js.drop_err());
        }
        self.style = Some(style);
    }


    pub fn set_style_url(&mut self, url: &str, ctx: &mut Context<MapViewImpl<I>>) {
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

        self.mouse_moved(pixel);
    }
    pub fn handle_mouse_button(&mut self, down: bool) {
        self.mouse_button(down);
    }

    pub fn handle_mouse_scroll(&mut self, scroll: f64) {

        self.mouse_scroll(scroll);
    }


    pub fn render(&mut self, target: &mut glium::Frame, ctx: &mut Context<Self>) {
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
    }

    pub fn new_tile(&mut self, tile: tiles::TileData, ctx: &mut Context<MapViewImpl<I>>) {
        if let Some(ref mut r) = self.renderer {
            r.tile_ready(Rc::new(tile));
        }
    }
}

/*
trace_macros!(true);

impl_invoke_handler!(MapViewImpl);

*/

impl<I: hal::Platform, F, R> Handler<Invoke<MapViewImpl<I>, F, R>> for MapViewImpl<I> where
    F: FnOnce(&mut MapViewImpl<I>, &mut <MapViewImpl<I> as Actor>::Context)
        -> R, R: 'static {
    type Result = Result<R>;
    fn handle(
        &mut self, msg: Invoke<MapViewImpl<I>, F, R>, _ctx: &mut Context<
            Self>) -> Result<R> { Ok((msg.f)(self, _ctx)) }
}