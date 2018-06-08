pub extern crate rmaps;

pub mod prelude;

use prelude::*;


fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Hello, world!")
        .with_dimensions(1024, 768);
    let context = glutin::ContextBuilder::new()
        .with_vsync(true);

    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut map = rmaps::map::MapView::new(&display.clone()).unwrap();
    map.set_style(common::json::from_str(include_str!("../../../bright.json")).unwrap());

    let mut running = true;
    while running {

        let mut surface = display.draw();
        map.render(&mut surface);
        surface.finish().unwrap();


        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed => {
                        running = false;
                        return;
                    },
                    glutin::WindowEvent::Resized(w, h) => display.gl_window().resize(w, h),
                    _ => ()
                },
                _ => ()
            }
        });

    }
}
