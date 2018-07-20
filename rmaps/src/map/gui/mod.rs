use ::prelude::*;

use ::imgui;
use imgui::*;

use ::imgui_glium_renderer::Renderer;


use common::glium::{
    self,
    glutin::{
        self,
        dpi::{
            LogicalPosition,
            LogicalSize,
        },
    },
    *,
};

#[derive(Default)]
pub struct InputData {
    wheel: f32,
}

pub struct State {}

pub struct Gui {
    imgui: ImGui,
    display: Box<glium::Display>,
    renderer: Renderer,
    last_frame: PreciseTime,
    state: State,
    input_data: InputData,
}

impl Gui {
    pub fn new(display: &glium::Display) -> Result<Self> {
        let mut imgui = ImGui::init();
        imgui.set_ini_filename(None);

        let config = ImFontConfig::new()
            .oversample_h(4)
            .pixel_snap_h(true)
            .size_pixels(15.0);
        config.rasterizer_multiply(1.75).add_font(
            &mut imgui.fonts(), include_bytes!("../../../../Roboto-Medium.ttf"), &FontGlyphRange::japanese());
        config.merge_mode(true).add_default_font(&mut imgui.fonts());

        let mut renderer = Renderer::init(&mut imgui, display).expect("Failed to initialize renderer");

        Ok(Gui {
            imgui,
            display: Box::new((*display).clone()),
            renderer,
            last_frame: PreciseTime::now(),
            state: State {},
            input_data: Default::default(),
        })
    }
    pub fn render(&mut self, surface: &mut glium::Frame) {
        let now = PreciseTime::now();
        let delta = self.last_frame.to(now);
        let delta_s = delta.num_microseconds().unwrap() as f32 / 1_000_000.0;
        self.last_frame = now;

        let size_pt = surface.get_dimensions();
        let ui = self.imgui.frame(size_pt, size_pt, delta_s);
        render(&ui, &mut self.state);
        self.renderer.render(surface, ui).expect("Rendering failed");
    }
    pub fn resized(&mut self, size: LogicalSize) {}
}

use super::{MapViewImpl, input::*};

impl InputHandler for Gui {
    fn mouse_moved(&mut self, pos: PixelPoint) {
        self.imgui.set_mouse_pos(pos.x as _, pos.y as _);
        trace!("Moved: {:?}", self.imgui.mouse_pos());
    }

    fn mouse_button(&mut self, pressed: bool) -> bool {
        self.imgui.set_mouse_down(&[pressed, false, false, false, false, ]);
        return true;
        //return self.imgui
    }

    fn mouse_scroll(&mut self, scroll: f64) {
        self.input_data.wheel += scroll as f32;
        self.imgui.set_mouse_wheel(self.input_data.wheel);
        self.input_data.wheel = 0.0;
    }
}

/*
fn hello_world<'a>(ui: &Ui<'a>) -> bool {
    ui.window(im_str!("Hello world"))
        .size((300.0, 100.0), ImGuiCond::FirstUseEver)
        .build(|| {
            ui.text(im_str!("Hello world!"));
            ui.text(im_str!("こんにちは世界！"));
            ui.text(im_str!("This...is...imgui-rs!"));
            ui.separator();
            let mouse_pos = ui.imgui().mouse_pos();
            ui.text(im_str!(
                "Mouse Position: ({:.1},{:.1})",
                mouse_pos.0,
                mouse_pos.1
            ));
        });

    true
}
*/

fn render<'a>(ui: &Ui<'a>, state: &'a mut State) {
    ui.window(im_str!("Profiler"))
        //.position((400., 400.), ImGuiCond::FirstUseEver)
        .size((400., 400.), ImGuiCond::FirstUseEver)
        .movable(true)
        .resizable(true)
        .build(|| {
            ui.text(im_str!("Hello world!"));
            let mouse_pos = ui.imgui().mouse_pos();
            ui.text(im_str!(
                "Mouse Position: ({:.1},{:.1})",
                mouse_pos.0,
                mouse_pos.1
            ));
        })
}