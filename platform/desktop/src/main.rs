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

    ::rmaps::init();

    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut map = rmaps::map::MapView::new(&display.clone());//.unwrap();
    map.set_style_url("file://../../../bright.json");

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
                    },
                    glutin::WindowEvent::Resized(w, h) => display.gl_window().resize(w, h),
                    _ => ()
                },
                _ => ()
            }
        });

    }
}
