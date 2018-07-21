use ::prelude::*;

use ::imgui::{
    self,*
};

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

pub mod prof;

#[derive(Default)]
pub struct InputData {
    wheel: f32,
}

#[derive(Default)]
pub struct State {
    profiler: prof::ProfilerState,
}

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
            state: State {
                profiler: prof::ProfilerState::new(),
                ..Default::default()
            },

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
    fn has_captured(&mut self) -> bool {
        return self.imgui.input_state().want_capture_mouse();
    }

    fn mouse_moved(&mut self, pos: PixelPoint) -> bool {
        self.imgui.set_mouse_pos(pos.x as _, pos.y as _);
        self.has_captured()
    }

    fn mouse_button(&mut self, pressed: bool) -> bool {
        self.imgui.set_mouse_down(&[pressed, false, false, false, false, ]);
        self.has_captured()
    }

    fn mouse_scroll(&mut self, scroll: f64) -> bool {
        self.input_data.wheel += scroll as f32;
        self.imgui.set_mouse_wheel(self.input_data.wheel);
        self.input_data.wheel = 0.0;
        self.has_captured()
    }
}

fn render<'a>(ui: &Ui<'a>, state: &'a mut State) {
    let win_size = ui.imgui().display_size();
    ui.window(im_str!("Profiler"))
        .position((0., 0.), ImGuiCond::Always)
        .size((win_size.0, 300.), ImGuiCond::Always)
        .movable(true)
        .resizable(true)
        .build(|| {
            state.profiler.render(ui)
        })
}
