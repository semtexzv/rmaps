use prelude::*;


pub mod layers;

pub mod property;

pub mod shaders;


use map::style;
use map::tiles::{
    TileLoader,
    data,
};
use std::hash;


pub struct RendererParams<'a> {
    pub display: &'a Display,
    pub frame: &'a mut glium::Frame,
    pub camera: &'a Camera,

    pub loader: Addr<Syn, TileLoader>,

    pub frame_start: PreciseTime,

}

pub struct PrepareParams<'a> {
    pub camera: &'a Camera,
    pub cover: &'a TileCover,

    pub requestor: &'a mut dyn FnMut(&str, TileCoords),
}

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
    pub style: Rc<style::Style>,
    pub layers: Vec<LayerData>,
}


impl Renderer {
    pub fn new(display: &Display, style: Rc<style::Style>) -> Self {
        Renderer {
            display: Box::new(display.clone()),
            layers: layers::parse_style_layers(&display, &style).into_iter().map(|l| {
                LayerData {
                    layer: l,
                    evaluated: None,
                }
            }).collect(),
            style,

        }
    }
    pub fn tile_ready(&mut self, tile: Rc<data::TileData>) {
        for l in self.layers.iter_mut() {
            l.layer.new_tile(&self.display, &tile).unwrap();
        }
    }
    pub fn render(&mut self, mut params: RendererParams) -> Result<()> {
        let camera = params.camera;
        let cover = TileCover::from_camera(camera);

        let eval_params = EvaluationParams::new(params.camera.zoom);

        let requests: Result<BTreeSet<(String, TileCoords)>> = self.layers
            .deref_mut()
            .par_iter_mut()
            .map(|l| {
                let mut req = vec![];
                l.layer.prepare(PrepareParams {
                    camera,
                    cover: &cover,
                    requestor: &mut |source, tile| {
                        req.push((source.to_string(), tile));
                    },
                })?;

                Ok(req)
            })
            .collect::<Result<Vec<_>>>()
            .map(|outer| {
                outer.into_iter().flat_map(|inner| inner.into_iter())
            })
            .map(|r| r.collect());


        //trace!("Needed tiles: {:#?}", requests);
        for t in requests.unwrap() {
            if let Some(source) = self.style.sources.get(&t.0) {
                let name: String = t.0.into();
                let coord = t.1;
                let source = source.clone();
                let fut = params.loader.send(Invoke::new(move |loader: &mut TileLoader|  {
                    loader.request_tile(name, &source, coord);
                }))
                    .map(|_|());
                spawn(fut);
            }
        }

        self.layers.deref_mut().par_iter_mut().for_each(|l| {
            let (should_eval, really) = match l.evaluated {
                None => (true, true),
                Some(ref e) if e.zoom != eval_params.zoom => (true, false),
                _ => (false, false),
            };

            if should_eval {
                l.layer.evaluate(&eval_params).unwrap();
            }
        });

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
