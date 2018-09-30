#![allow(unused_variables, unused_imports)]
#![feature(nll)]
#![feature(unboxed_closures)]

extern crate futures;

extern crate rt_wasm;
#[macro_use]
extern crate stdweb;

pub extern crate rmaps;
pub extern crate common;

pub mod types;
pub mod prelude;

use prelude::*;

use common::glium::{
    self,
    glutin::{
        self,
        WindowEvent,
        MouseButton, ElementState,
        dpi::{
            LogicalSize, LogicalPosition,
            PhysicalPosition, PhysicalSize,
        },
    },
};


use glium::glutin::*;


use stdweb::unstable::TryInto;
use stdweb::web::{
    self,
    IEventTarget,
    INonElementParentNode,
    CanvasRenderingContext2d,
    html_element::CanvasElement,
};

fn async_main() -> impl Future<Item=(), Error=()> {
    futures::lazy(move || {
        return futures::finished(());
    })
}


use glium::index::PrimitiveType;

fn main() {
    stdweb::initialize();
    common::init_log();

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Maps test");

    let context = glutin::ContextBuilder::new()
        .with_gl(glutin::GlRequest::Specific(glutin::Api::WebGl, (2, 0)))
        .with_pixel_format(8, 8)
        .with_stencil_buffer(8);

    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut map = rmaps::map::MapView::<types::WebTypes>::new(&display.clone());
    //map.set_style_url("mapbox://styles/semtexzv/cjjjv418k6m0b2rok0oiejd4i");
    //map.set_style_url("mapbox://styles/semtexzv/cjmlvlcf3qyqu2rniv6m2oxlu");
    map.set_style_url("mapbox://styles/semtexzv/cjmmc76cxrxs72ss4k8u81ikd");

    let size = display.get_framebuffer_dimensions();
    map.window_resized(PixelSize::new(size.0, size.1));

    events_loop.run_forever(|event| {
        let surface = display.draw();
        map.render(surface);

        match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => {
                    return glium::glutin::ControlFlow::Break;
                }
                WindowEvent::Resized(s) => {
                    map.window_resized(PixelSize::new(s.width, s.height));
                }
                WindowEvent::CursorMoved { position, .. } => {
                    map.mouse_moved(PixelPoint::new(position.x, position.y));
                }
                WindowEvent::MouseInput { state: glium::glutin::ElementState::Pressed, button: MouseButton::Left, .. } => {
                    map.mouse_button(true);
                }
                WindowEvent::MouseInput { state: ElementState::Released, button: MouseButton::Left, .. } => {
                    map.mouse_button(false);
                }
                WindowEvent::MouseWheel { delta,  .. } => {
                    println!("MouseWheel : {:?} ", delta);
                    match delta {
                        glutin::MouseScrollDelta::PixelDelta(LogicalPosition { x, y }) => {
                            map.mouse_scroll(y as f64/ 300. );
                        }
                        glutin::MouseScrollDelta::LineDelta(_x, y) => {
                            map.mouse_scroll(y as f64/ 10. );
                        }
                    };
                }
                _ => ()
            },
            _ => ()
        }
        stdweb::webcore::executor::EventLoop.wake();
        glium::glutin::ControlFlow::Continue
    });
}
