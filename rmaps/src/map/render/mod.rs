use prelude::*;

#[macro_export]
macro_rules! layer_program {
    ($facade:expr, $name:expr, $uniforms:expr, $features:expr) => { {
            let vert_src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../shaders/",$name,".vert.glsl"));
            let frag_src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../shaders/",$name,".frag.glsl"));
            ::map::render::shaders::ShaderProcessor::get_shader($facade,$name, vert_src,frag_src,$uniforms,$features)
        }
    };
}


pub mod images;
pub mod layers;
pub mod source;

pub mod property;

pub mod shaders;

pub mod clip;


use map::style;

use std::hash;

use map::{
    tiles,
    util::profiler,
};


use self::images::ImageAtlas;

pub struct RendererParams<'a, I: ::map::hal::Platform> {
    pub display: &'a Display,
    pub frame: &'a mut glium::Frame,
    pub camera: &'a Camera,

    pub ctx: &'a mut Context<super::MapViewImpl<I>>,

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
    pub atlas: &'a ImageAtlas,
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
    pub clipper: clip::Clipper,
    pub sources: BTreeMap<String, Addr<source::BaseSource>>,
    pub image_atlas: images::ImageAtlas,
}


impl Renderer {
    pub fn new<P: hal::Platform>(display: &Display, style: Rc<style::Style>, file_source: Recipient<::map::storage::Request>) -> Self {
        Renderer {
            display: Box::new(display.clone()),
            layers: layers::parse_style_layers(&display, &style).into_iter().map(|l| {
                LayerData {
                    layer: l,
                    evaluated: None,
                }
            }).collect(),
            clipper: clip::Clipper::new(display).unwrap(),
            sources: source::parse_sources::<P>(&style, file_source),
            image_atlas: images::ImageAtlas::new(&display).unwrap(),
            style,

        }
    }
    pub fn sprite_json_ready(&mut self, data: ::map::style::sprite::SpriteAtlas) {
        self.image_atlas.set_sprite_atlas(data);
    }
    pub fn sprite_png_ready(&mut self, data: Vec<u8>) {
        self.image_atlas.set_sprite_texture(data);
    }
    pub fn tile_ready(&mut self, tile: Rc<tiles::TileData>) {
        for l in self.layers.iter_mut() {
            l.layer.new_tile(&self.display, &tile).unwrap();
        }
    }
    pub fn render<I: ::map::hal::Platform>(&mut self, mut params: RendererParams<I>) -> Result<()> {
        params.frame.clear_color(0., 0., 1., 1.);
        params.frame.clear_stencil(0xFF);

        let camera = params.camera;
        let cover = TileCover::from_camera(camera);

        let eval_params = EvaluationParams::new(params.camera.zoom);


        let requests: Vec<Vec<(String, TileCoords)>> = self.layers
            .deref_mut()
            .iter_mut()
            .map(|l| {
                let mut req = vec![];
                l.layer.prepare(PrepareParams {
                    camera,
                    cover: &cover,
                    requestor: &mut |source, tile| {
                        req.push((source.to_string(), tile));
                    },
                }).unwrap();

                req
            }).collect();

        //println!("Requests for tiles : {:?}", requests);

        let requests: Vec<(String, TileCoords, _)> = requests
            .into_iter()
            .fold(BTreeSet::new(), |mut acc, v| {
                acc.extend(v);
                acc
            }).into_iter().map(|t| {
            if let Some(source) = self.style.sources.get(&t.0) {
                let name: String = t.0.into();
                let coord = t.1;
                let source = source.clone();
                (name, coord, source)
            } else {
                panic!()
            }
        }).collect();

        for (name, coord, source) in requests.into_iter() {
            let source = self.sources.get(&name).expect("Source missing");
            use common::actix::fut::*;

            //println!("Requesting : {:?} from {:?}", coord, name);
            let req = self::source::TileRequest {
                coords: coord,
            };

            let req = wrap_future(source.send(req));
            use self::source::TileError;

            let fut = req.from_err::<Error>()
                .and_then(|res, this: &mut super::MapViewImpl<I>, ctx| {
                    match res {
                        Ok(data) => {
                            println!("Got tile");
                            this.new_tile(data, ctx);
                        }
                        Err(TileError::Error(e)) => {
                            debug!("Tile error occured : {:?}", e);
                        }
                        _ => {}
                    }
                    ok(())
                });

            params.ctx.spawn(fut.drop_err());
        }

        self.layers.deref_mut().iter_mut().for_each(|l| {
            let (should_eval, really) = match l.evaluated {
                None => (true, true),
                Some(ref e) if e.zoom != eval_params.zoom => (true, false),
                _ => (false, false),
            };

            if should_eval {
                l.layer.evaluate(&eval_params).unwrap();
            }
        });

        let mut params = RenderParams {
            display: &params.display,
            frame: &mut params.frame,
            camera: &params.camera,
            atlas: &self.image_atlas,
            frame_start: params.frame_start,

        };
        self.clipper.apply_mask(&cover, &mut params)?;

        for l in self.layers.iter_mut() {
            l.layer.render(&mut params).unwrap();
        }

        Ok(())
    }
}
