use prelude::*;


pub mod layers;

pub mod property;

pub mod shaders;


use map::style;
use map::tiles::data;
use std::hash;


pub struct RendererParams<'a> {
    pub display: &'a Display,
    pub frame: &'a mut glium::Frame,
    pub camera: &'a Camera,

    pub frame_start: PreciseTime,
}

pub struct PrepareParams<'a> {
    pub camera: &'a Camera,
    pub cover: &'a TileCover,
}

/// Idea:
/// Stencil test for each tile, by utilizing encoding tile ids into 8 or 16 bits
/// and using Equality tests for stencil clipping, therefore, no overlaps will be rendered
///

pub struct RenderParams<'a> {
    pub display: &'a Display,
    pub frame: &'a mut glium::Frame,
    pub camera: &'a Camera,

    pub frame_start: PreciseTime,
}


#[derive(Debug)]
pub struct EvaluationParams {
    pub zoom: f32,
    pub time: u64,

}

impl EvaluationParams {
    fn new(zoom: f32) -> Self {
        EvaluationParams {
            zoom,
            time: 0,
        }
    }
}

#[derive(Debug)]
pub struct LayerData {
    pub layer: Box<layers::Layer>,
    pub evaluated: Option<EvaluationParams>,
}

unsafe impl Send for LayerData {}

unsafe impl Sync for LayerData {}

pub struct Renderer {
    pub display: Box<Display>,
    pub layers: Vec<LayerData>,
}


impl Renderer {
    pub fn new(display: &Display) -> Self {
        Renderer {
            display: Box::new(display.clone()),
            layers: vec![],
        }
    }

    pub fn style_changed(&mut self, style: &style::Style) -> Result<()> {
        self.layers = layers::parse_style_layers(&self.display, style).into_iter().map(|l| {
            LayerData {
                layer: l,
                evaluated: None,
            }
        }).collect();
        Ok(())
    }
    pub fn tile_ready(&mut self, tile: Rc<data::TileData>) {
        for l in self.layers.iter_mut() {
            l.layer.new_tile(&self.display, &tile).unwrap();
        }
    }
    pub fn render(&mut self, mut params: RendererParams) -> Result<()> {
        let camera = params.camera;
        let cover = TileCover::from_camera(camera);
        self.layers.deref_mut().par_iter_mut().for_each(|l| {
            l.layer.prepare(PrepareParams {
                camera,
                cover : &cover,
            }).unwrap();
        });

        let eval_params = EvaluationParams::new(params.camera.zoom);

        for l in self.layers.iter_mut() {
            let (should_eval, really) = match l.evaluated {
                None => (true, true),
                Some(ref e) if e.zoom != eval_params.zoom => (true, false),
                _ => (false, false),
            };

            if should_eval {
                l.layer.evaluate(&eval_params)?;
            }
        }

        let cover = TileCover::from_camera(&params.camera);
        for l in self.layers.iter_mut() {
            l.layer.render(&mut RenderParams {
                display: &params.display,
                frame: &mut params.frame,
                camera: &params.camera,
                frame_start: params.frame_start,

            }).unwrap();
        }

        Ok(())
    }
}
