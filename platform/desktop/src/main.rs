#![allow(unused_variables,unused_imports)]
pub extern crate rmaps;

pub mod prelude;

use prelude::*;

use ::prelude::common::glium::{
    self,
    glutin::{
        self,
        WindowEvent,
        dpi::{
            LogicalSize, LogicalPosition,
            PhysicalPosition, PhysicalSize,
        },
    },
};


fn main() {
    common::init_log();

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Maps test")

        .with_dimensions(LogicalSize::new(800., 800.));

    let context = glutin::ContextBuilder::new()
        .with_pixel_format(8, 8)
        .with_stencil_buffer(8)
        .with_vsync(true);

    ::rmaps::init();

    let display = glium::Display::new(window, context, &events_loop).unwrap();


    let mut map = rmaps::map::MapView::new(&display.clone());//.unwrap();
    map.set_style_url("file://std.json");
    let mut running = true;
    while running {
        let surface = display.draw();
        map.render(surface);


        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => {
                        running = false;
                        return;
                    }
                    glium::glutin::WindowEvent::MouseWheel { delta, modifiers, .. } => {
                        let px = match delta {
                            glutin::MouseScrollDelta::PixelDelta(LogicalPosition { x, y }) => {
                                y as f32
                            }
                            glutin::MouseScrollDelta::LineDelta(_x, y) => {
                                y
                            }
                        };
                        let mut c = map.get_camera();

                        c.zoom += px;
                        map.set_camera(c);
                    }
                    //glutin::WindowEvent::Resized(s) => display.gl_window().resize(PhysicalSize::from_logical(s)),
                    _ => ()
                },
                _ => ()
            }
        });
    }
}
