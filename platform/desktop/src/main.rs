#![allow(unused_variables, unused_imports)]
#![feature(nll)]
#![feature(unboxed_closures)]

pub extern crate rmaps;
pub extern crate common;

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


use glium::glutin::{
    *,
};

fn main() {
    common::init_log();

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Maps test")

        .with_dimensions(LogicalSize::new(600., 600.));

    let context = glutin::ContextBuilder::new()
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (4,0)))
        //.with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGlEs, (3, 0)))
        .with_gl_profile(glutin::GlProfile::Core)
        .with_pixel_format(8, 8)
        .with_stencil_buffer(8)
        .with_vsync(true);

    let display = glium::Display::new(window, context, &events_loop).unwrap();


    let mut map = rmaps::map::MapView::new(&display.clone());//.unwrap();
    //map.set_style_url("file://simple.json");
    //map.set_style_url("mapbox://styles/semtexzv/cjjjv418k6m0b2rok0oiejd4i");
    // North star
    //map.set_style_url("mapbox://styles/semtexzv/cjm699hdycl2y2snx6os4bo9t");
    // Streets
    map.set_style_url("mapbox://styles/semtexzv/cjmdb386z7hm22rmlunomo8w0");
    let mut running = true;



    while running {
        let surface = display.draw();
        map.render(surface);


        events_loop.poll_events(|event| {
            //let mut camera: Camera = map.get_camera();
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => {
                        running = false;
                        return;
                    }

                    WindowEvent::Resized(s) => {
                        map.window_resized(PixelSize::new(s.width, s.height));
                        //size = s;
                        //camera.set_size(PixelSize::new(s.width, s.height));
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        //pos = PixelPoint::new(position.x, position.y);
                        map.mouse_moved(PixelPoint::new(position.x, position.y));

                        /*
                        if let Some(last) = last_pos {
                            let last = camera.window_to_world(last);
                            let now = camera.window_to_world(pos);

                            let diff = now - last;
                            camera.set_pos(camera.pos() - diff);
                            last_pos = Some(pos);
                        }
                        */
                    }
                    WindowEvent::MouseInput { state: glium::glutin::ElementState::Pressed, button: MouseButton::Left, .. } => {
                        map.mouse_button(true);

                        //down_pos = Some(pos.clone());
                        //last_pos = Some(pos.clone());
                    }
                    WindowEvent::MouseInput { state: ElementState::Released, button: MouseButton::Left, .. } => {
                        map.mouse_button(false);
                        /*
                        down_pos = None;
                        last_pos = None;
                        map.clicked(PixelPoint::new(pos.x, pos.y));
                        */
                    }
                    WindowEvent::MouseWheel { delta, modifiers, .. } => {
                        match delta {
                            glutin::MouseScrollDelta::PixelDelta(LogicalPosition { x, y }) => {
                                map.mouse_scroll(y as _);
                            }
                            glutin::MouseScrollDelta::LineDelta(_x, y) => {
                                map.mouse_scroll( y as f64 / 16.);;

                            }
                        };
                    }
                    _ => ()
                },
                _ => ()
            }
        });
        // let t2 = PreciseTime::now();
        // info!("Time: {:?}",t1.to(t2).num_milliseconds());
    }
}
