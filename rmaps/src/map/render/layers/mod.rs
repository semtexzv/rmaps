use prelude::*;

use map::{
    style,
    tiles::data,
    render::{
        self, property,
    },
};


macro_rules! layer_program {
    ($facade:expr, $name:expr, $uniforms:expr, $features:expr) => { {
            let vert_src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../shaders/",$name,".vert.glsl"));
            let frag_src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../shaders/",$name,".frag.glsl"));
            ::map::render::shaders::ShaderProcessor::get_shader($facade, vert_src,frag_src,$uniforms,$features)
        }
    };
}

pub mod background;
pub mod raster;
pub mod fill;

#[repr(C)]
#[derive(Debug, Clone, Copy, Vertex)]
pub struct Vertex {
    #[glium(attr = "pos")]
    pos: [f32; 2],
    feature: u16,
}

pub enum RenderPass {
    Opaque,
    Translucent,
    Extrusion,
}

pub trait Layer: Debug {
    /// Called when new tile data arrives, individual layers will need to copy the Rc, if they need to keep the data around
    fn new_tile(&mut self, display: &Display, data: &Rc<data::TileData>) -> Result<()>;

    /// Called for new layers, or layers that have been explicitly changed
    /// Also called when zoom level changes
    fn evaluate(&mut self, params: &render::EvaluationParams) -> Result<()>;

    /// Called for each frame, GPU uploads and rendering happens here
    fn render(&mut self, params: &mut render::RenderParams) -> Result<()>;


    fn begin_pass(&mut self, params: &mut render::RenderParams, pass: RenderPass) -> Result<()> {
        Ok(())
    }

    fn has_render_pass(&self, pass: RenderPass) -> bool {
        match pass {
            RenderPass::Opaque => true,
            _ => false
        }
    }

    fn end_pass(&mut self, params: &mut render::RenderParams, pass: RenderPass) -> Result<()> {
        Ok(())
    }
}


pub trait Bucket: Debug {
    fn needs_explicit_eval(&self) -> bool {
        false
    }
    fn upload(&mut self, display: &Display) -> Result<()>;
}

pub trait BucketLayer: Debug {
    type Bucket: Bucket;

    fn begin_pass(&mut self, params: &mut render::RenderParams, pass: RenderPass) -> Result<()> {
        Ok(())
    }

    fn end_pass(&mut self, params: &mut render::RenderParams, pass: RenderPass) -> Result<()> {
        Ok(())
    }

    fn new_tile(&mut self, display: &Display, data: &Rc<data::TileData>) -> Result<Option<Self::Bucket>>;


    fn eval_layer(&mut self, params: &render::EvaluationParams) -> Result<()> {
        Ok(())
    }
    fn eval_bucket(&mut self, params: &render::EvaluationParams, bucket: &mut Self::Bucket) -> Result<()>;

    fn render_bucket(&mut self, params: &mut render::RenderParams, bucket: &Self::Bucket) -> Result<()>;
}

#[derive(Debug)]
pub struct BucketState<B: Bucket> {
    pub bucket: B,
    pub evaluated: Option<render::EvaluationParams>,
}

#[derive(Debug)]
pub struct BucketLayerHolder<L: BucketLayer> {
    pub layer: L,
    pub buckets: BTreeMap<TileCoords, BucketState<L::Bucket>>,
}

impl<L: BucketLayer> BucketLayerHolder<L> {
    pub fn new(l: L) -> Self {
        BucketLayerHolder {
            layer: l,
            buckets: BTreeMap::new(),
        }
    }
}

impl<L: BucketLayer> Layer for BucketLayerHolder<L> {
    fn new_tile(&mut self, display: &Display, data: &Rc<data::TileData>) -> Result<()> {
        let coords = data.coord;
        if let Some(bucket) = self.layer.new_tile(display, data)? {
            self.buckets.insert(coords, BucketState {
                bucket,
                evaluated: None,
            });
        };

        Ok(())
    }

    /// TODO, better system for re-evaluating and uploading  modified data,
    /// Only re-evaluate on zoom change of integer coordinates ?
    fn evaluate(&mut self, params: &render::EvaluationParams) -> Result<()> {
        let zoom = params.zoom as _;
        let pred = |(k, _): &(&TileCoords, &mut BucketState<L::Bucket>)| {
            k.z < zoom as i32 + 1
        };

        for (k, mut v) in self.buckets.iter_mut().filter(pred) {
            let should_eval = match v.evaluated {
                None => true,
                Some(ref e) if e.zoom != zoom => true,
                _ => false,
            };
            if should_eval {
                self.layer.eval_bucket(params, &mut v.bucket)?;
            }

            v.evaluated = Some(render::EvaluationParams::new(zoom));
        }

        Ok(())
    }

    // TODO: beter render picking system, checkout mapbox tile cover
    fn render(&mut self, params: &mut render::RenderParams) -> Result<()> {
        self.layer.begin_pass(params, RenderPass::Opaque)?;
        let zoom = params.camera.zoom;

        let pred = |(k, _): &(&TileCoords, &mut BucketState<L::Bucket>)| {
            k.z < zoom as i32 + 1
        };
        for (k, mut v) in self.buckets.iter_mut().filter(pred) {
            v.bucket.upload(params.disp)?;
            self.layer.render_bucket(params, &mut v.bucket)?;
        }
        self.layer.end_pass(params, RenderPass::Opaque)?;
        Ok(())
    }
}

pub fn parse_style_layers(facade: &Display, style: &style::Style) -> Vec<Box<dyn Layer>> {
    let mut res: Vec<Box<Layer>> = vec![];
    for l in style.layers.iter() {
        match l {
            style::BaseStyleLayer::Background(l) => {
                res.push(Box::new(background::BackgroundLayer::parse(l.clone())))
            }
            style::BaseStyleLayer::Fill(l) => {
                res.push(Box::new(BucketLayerHolder::new(fill::FillLayer::parse(facade, l.clone()))))
            }
            style::BaseStyleLayer::Raster(l) => {
                res.push(Box::new(raster::RasterLayer::parse(facade, l.clone())))
            }
            _ => {}
        }
    }
    res
}