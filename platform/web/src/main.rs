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

fn main2() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Maps test")

        .with_dimensions(LogicalSize::new(600., 600.));

    let context = glutin::ContextBuilder::new()
        .with_gl(glutin::GlRequest::Specific(glutin::Api::WebGl, (2, 0)))
        .with_pixel_format(8, 8)
        .with_stencil_buffer(8);


    let display = glium::Display::new(window, context, &events_loop).unwrap();
    /*
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    */

    // building the vertex buffer, which contains all the vertices that we will draw
    let vertex_buffer = {
        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 2],
            color: [f32; 3],
        }

        implement_vertex!(Vertex, position, color);

        glium::VertexBuffer::new(&display,
                                 &[
                                     Vertex { position: [-0.5, -0.5], color: [0.0, 1.0, 0.0] },
                                     Vertex { position: [0.0, 0.5], color: [0.0, 0.0, 1.0] },
                                     Vertex { position: [0.5, -0.5], color: [1.0, 0.0, 0.0] },
                                 ],
        ).unwrap()
    };

    // building the index buffer
    let index_buffer = glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList,
                                               &[0u16, 1, 2]).unwrap();

    // compiling shaders and linking them together
    let program = program!(&display,
        140 => {
            vertex: "
                #version 140
                uniform mat4 matrix;
                in vec2 position;
                in vec3 color;
                out vec3 vColor;
                void main() {
                    gl_Position = vec4(position, 0.0, 1.0) * matrix;
                    vColor = color;
                }
            ",

            fragment: "
                #version 140
                in vec3 vColor;
                out vec4 f_color;
                void main() {
                    f_color = vec4(vColor, 1.0);
                }
            "
        },

        110 => {
            vertex: "
                #version 110
                uniform mat4 matrix;
                attribute vec2 position;
                attribute vec3 color;
                varying vec3 vColor;
                void main() {
                    gl_Position = vec4(position, 0.0, 1.0) * matrix;
                    vColor = color;
                }
            ",

            fragment: "
                #version 110
                varying vec3 vColor;
                void main() {
                    gl_FragColor = vec4(vColor, 1.0);
                }
            ",
        },

        100 => {
            vertex: "
                #version 100
                uniform lowp mat4 matrix;
                attribute lowp vec2 position;
                attribute lowp vec3 color;
                varying lowp vec3 vColor;
                void main() {
                    gl_Position = vec4(position, 0.0, 1.0) * matrix;
                    vColor = color;
                }
            ",

            fragment: "
                #version 100
                varying lowp vec3 vColor;
                void main() {
                    gl_FragColor = vec4(vColor, 1.0);
                }
            ",
        },
    ).unwrap();

    // Here we draw the black background and triangle to the screen using the previously
    // initialised resources.
    //
    // In this case we use a closure for simplicity, however keep in mind that most serious
    // applications should probably use a function that takes the resources as an argument.
    let draw = || {
        // building the uniforms
        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ]
        };

        // drawing a frame
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.clear_stencil(0xFF);
        target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();
    };

    // Draw the triangle to the screen.
    draw();

    // the main loop
    events_loop.run_forever(|event| {
        match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                // Break from the main loop when the window is closed.
                glutin::WindowEvent::CloseRequested => return glutin::ControlFlow::Break,
                // Redraw the triangle when the window is resized.
                glutin::WindowEvent::Resized(..) => draw(),
                _ => (),
            },
            _ => (),
        }
        glutin::ControlFlow::Continue
    });
}

fn main() {
    stdweb::initialize();
    common::init_log();

   // main2();


    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Maps test")

        .with_dimensions(LogicalSize::new(600., 600.));

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
        println!("Event loop");
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
                WindowEvent::MouseWheel { delta, modifiers, .. } => {
                    match delta {
                        glutin::MouseScrollDelta::PixelDelta(LogicalPosition { x, y }) => {
                            map.mouse_scroll(y as _);
                        }
                        glutin::MouseScrollDelta::LineDelta(_x, y) => {
                            map.mouse_scroll(y as f64 / 16.);
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
