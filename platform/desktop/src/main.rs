pub extern crate rmaps;

pub mod prelude;

use prelude::*;

use ::prelude::common::glium::{
    self,
    glutin,
};


fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Hello, world!")
        .with_dimensions(800, 800);
    let context = glutin::ContextBuilder::new()
        .with_pixel_format(8, 8)
        .with_vsync(true);

    ::rmaps::init();

    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut map = rmaps::map::MapView::new(&display.clone());//.unwrap();
    map.set_style_url("file://style.json");

    let mut running = true;
    while running {
        let surface = display.draw();
        map.render(surface);

        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed => {
                        running = false;
                        return;
                    }
                    glium::glutin::WindowEvent::MouseWheel { delta, phase, modifiers, .. } => {
                        let px = match delta {
                            glutin::MouseScrollDelta::PixelDelta(_x, y) => {
                                y
                            }
                            glutin::MouseScrollDelta::LineDelta(_x, y) => {
                                y
                            }
                        };
                        let mut c = map.get_camera();

                        c.zoom += px;
                        map.set_camera(c);
                        //map.came
                    }
                    glutin::WindowEvent::Resized(w, h) => display.gl_window().resize(w, h),
                    _ => ()
                },
                _ => ()
            }
        });
    }
}
