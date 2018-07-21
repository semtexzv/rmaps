use ::prelude::*;

use ::imgui::{
    self, *,
};
use imgui::{
    Ui, StyleVar,
};

use ::imgui_glium_renderer::Renderer;
use ::common::arraydeque::{
    ArrayDeque,
    behavior::Wrapping,
};


use map::util::profiler::{
    Section,
    finish_frame,
};

pub type RingBuf<T> = ArrayDeque<T, Wrapping>;

pub const MAX_FRAMES: usize = 128;

#[derive(Default)]
pub struct ProfilerState {
    paused: bool,

    selection_start: usize,
    selection_end: usize,

    section_area_zoom_start: f32,
    frames: RingBuf<[Section; MAX_FRAMES]>,
}


impl ProfilerState {
    pub fn new() -> Self {
        ProfilerState {
            selection_end: 2,
            ..Default::default()
        }
    }
    fn set_pause(&mut self, paused: bool) {
        self.paused = paused;
    }
    pub fn render<'a>(&mut self, ui: &Ui<'a>) {
        if !self.paused {
            if let Some(f) = finish_frame() {
                self.frames.push_back(f);
            }
        }
        if self.frames.len() < self.selection_end {
            return;
        }

        let control_height = 20.;
        let control_width = 60.;

        let frame_height = 60.;

        let control_size: ImVec2 = (control_width, control_height).into();

        let back_color: u32 = 0x88000000;
        let section_color: u32 = 0xFF808080;

        let area_size: ImVec2 = (ui.get_content_region_avail().0, control_height).into();
        let area_min: ImVec2 = ui.get_cursor_screen_pos().into();
        let area_max = (area_size.x + area_min.y, area_size.y + area_min.y);
        ui.get_window_draw_list().add_rect(area_min, area_max, back_color).build();

        if ui.button(
            if self.paused {
                im_str!("Resume")
            } else {
                im_str!("Pause")
            }, control_size) {
            self.set_pause(!self.paused)
        }

        let bars_area = (ui.get_content_region_avail().0, frame_height).into();
        let bars_min: ImVec2 = ui.get_cursor_screen_pos().into();
        let bars_max = bars_min + bars_area;

        /*
        ui.button(im_str!("Frame section"), bars_area);
        if ui.is_item_active() {}
            */

        let values = self.frames
            .iter()
            .map(|p| p.total().num_milliseconds() as f32)
            .collect::<Vec<_>>();

        ui.plot_histogram(im_str!(""), &values)
            .graph_size(bars_area)
            .scale_max(20.)
            .build();


        let section_area = (ui.get_content_region_avail().0, 200.).into();
        let section_min: ImVec2 = ui.get_cursor_screen_pos().into();
        let section_max = section_min + section_area;

        let count = self.selection_end - self.selection_start;
        let width_per_frame = section_area.x / count as f32;

        let row_height = control_height;
        let row_space = 4.;

        for (i, frame) in self.frames.iter().skip(self.selection_start).take(self.selection_end - self.selection_start).enumerate() {
            let start_time = frame.start;
            let end_time = frame.end;
            let total_time = start_time.to(end_time).num_microseconds().unwrap();

            let win_start = i as f32 * width_per_frame;
            let win_end = (i + 1) as f32 * width_per_frame;
            let win_available = win_end - win_start;

            struct DP {
                section_min: ImVec2,
                start_time: PreciseTime,
                end_time: PreciseTime,

                win_start: f32,
                win_end: f32,
                row_height: f32,
                row_space: f32,
                section_color: u32,
            }

            fn draw<'a>(ui: &Ui<'a>, level: usize, ev: &Section, p: &DP) {
                let total_time = p.start_time.to(p.end_time).num_microseconds().unwrap();
                let win_available = p.win_end - p.win_start;


                let offset_t = p.start_time.to(ev.start).num_microseconds().unwrap();
                let width_t = ev.start.to(ev.end).num_microseconds().unwrap();


                let offset = p.win_start + (offset_t as f32 / total_time as f32 * win_available);
                let width = width_t as f32 / total_time as f32 * win_available;

                let p1: ImVec2 = p.section_min + (offset + p.row_space / 2., level as f32 * p.row_height + p.row_space / 2.).into();
                let p2: ImVec2 = p.section_min + (offset + width - p.row_space / 2., (level + 1) as f32 * p.row_height).into();

                {
                    let draw_list = ui.get_window_draw_list();

                    draw_list.add_rect(p1, p2, p.section_color)
                        .filled(true)
                        .build();
                    draw_list.with_clip_rect(p1, p2, || {
                        draw_list.add_text((p1.x, p1.y ), 0xFFFFFFFF, format!("{}", ev.name));
                    });
                }
                for e in ev.frames.iter() {
                    draw(ui, level + 1, e, p);
                }
            }

            draw(ui, 0, frame, &DP {
                section_min,
                start_time,
                end_time,
                win_start,
                win_end,
                row_height,
                row_space,
                section_color,
            });
        }
    }
}


/*
pub fn render_profiler<'a>(ui: &Ui<'a>, state: &'a mut ProfilerState) {


    let values = state.screens
        .iter()
        .map(|p| p.total().num_milliseconds() as f32)
        .collect::<Vec<_>>();

    ui.child_frame(im_str!("scroll"), (ui.get_content_region_avail().0, 100.))
        .scrollbar_horizontal(true)
        .always_show_horizontal_scroll_bar(true)

        .show_borders(true)
        .build(|| {
            if state.locked {
                ui.scroll_to_x(ui.scroll_max_x());
            }

            let mut w = state.scale * values.len() as f32;
            if w < ui.get_content_region_avail().0 {
                w = ui.get_content_region_avail().0;
            }
            ui.plot_histogram(im_str!(""), &values)
                .graph_size((w, ui.get_content_region_avail().1 - 5.))
                .build();


            /*
            ui.plot_histogram(im_str!("Profiler"), &values)
                .graph_size((5. * values.len() as f32, ui.get_content_region_avail().1))
                .build();
                */
        });

    let w = ui.get_content_region_avail().0 / 4.;
    ui.child_frame(im_str!("controls"), (ui.get_content_region_avail().0, 25.)).build(|| {
        ui.push_item_width(w);
        ui.drag_float(im_str!("Scale"), &mut state.scale)
            .min(1.)
            .max(10.)
            .speed(0.01)
            .build();


        ui.same_line(0.);
        if state.recording && ui.button(im_str!("stop"), (w, -1.)) {
            state.recording = false;
        }
        if !state.recording && ui.button(im_str!("start"), (w, -1.)) {
            state.screens.clear();
            state.recording = true;
        }

        ui.same_line(0.);
        if state.locked {
            if ui.button(im_str!("unlock"), (w, -1.)) {
                state.locked = false;
            }
        } else {
            if ui.button(im_str!("lock"), (w, -1.)) {
                state.locked = true;
            }
        }


        ui.pop_item_width();
    });
    const row_height: f32 = 20.;
    const FACTOR: f32 = 400.;
    ui.child_frame(im_str!("frames"), (ui.get_content_region_avail().0, 150.)).build(|| {
        use map::util::profiler::Frame;


        fn draw(ui: &Ui, level: usize, start: PreciseTime, end: PreciseTime, ev: &Frame) {
            let total_t = start.to(end).num_microseconds().unwrap();
            let offset_t = start.to(ev.start).num_microseconds().unwrap();
            let width_t = ev.start.to(ev.end).num_microseconds().unwrap();

            let avail = ui.get_content_region_avail().0;

            let offset = offset_t as f32 / total_t as f32;
            let width = width_t as f32 / total_t as f32;

            ui.set_cursor_pos((offset * avail, level as f32 * row_height as f32));

            ui.with_id(im_str!("level-{}",level), || {
                ui.button(im_str!("-"), (width * avail, row_height));
            });
            for e in ev.frames.iter() {
                draw(ui, level + 1, start, end, e);
            }
        }

        let len = state.screens.len();
        if len > 4 {
            let start = state.screens[0].clone().start;
            let end = state.screens[3].clone().end;
            for f in state.screens.iter().take(3) {
                draw(ui, 0, start, end, &f);
            }
        }
    })
}

*/