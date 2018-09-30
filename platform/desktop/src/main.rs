#![allow(unused_variables, unused_imports)]
#![feature(nll)]
#![feature(unboxed_closures)]

#![feature(allocator_api)]

use std::alloc::System;

#[global_allocator]
static A: System = System;

pub extern crate rmaps;
pub extern crate common;
pub extern crate actix_web;

pub extern crate rusqlite;

pub mod prelude;
pub mod types;


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
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (4, 0)))
        //.with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGlEs, (3, 0)))
        .with_gl_profile(glutin::GlProfile::Core)
        .with_pixel_format(8, 8)
        .with_stencil_buffer(8)
        .with_srgb(true)
        .with_vsync(true);

    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut map = rmaps::map::MapView::<types::DesktopTypes>::new(&display.clone());//.unwrap();
    //map.set_style_url("file://simple.json");
    map.set_style_url("mapbox://styles/semtexzv/cjjjv418k6m0b2rok0oiejd4i");
    // North star
    //map.set_style_url("mapbox://styles/semtexzv/cjm699hdycl2y2snx6os4bo9t");
    // Streets
    // map.set_style_url("mapbox://styles/semtexzv/cjmdb386z7hm22rmlunomo8w0");


    //map.set_style_url("mapbox://styles/semtexzv/cjmlvlcf3qyqu2rniv6m2oxlu");
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
                    WindowEvent::MouseWheel { delta, modifiers, .. } => {
                        match delta {
                            glutin::MouseScrollDelta::PixelDelta(LogicalPosition { x, y }) => {
                                map.mouse_scroll(y as _);
                            }
                            glutin::MouseScrollDelta::LineDelta(_x, y) => {
                                map.mouse_scroll(y as f64 / 16.);
                                ;
                            }
                        };
                    }
                    _ => ()
                },
                _ => ()
            }
        });
    }
}
