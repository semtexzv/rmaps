use prelude::*;
pub use map::util::profiler;


use map::{
    style,
    tiles,
    render::{
        self, property,
    },
};

macro_rules! layer_program {
    ($facade:expr, $name:expr, $uniforms:expr, $features:expr) => { {
            let vert_src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../shaders/",$name,".vert.glsl"));
            let frag_src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../shaders/",$name,".frag.glsl"));
            ::map::render::shaders::ShaderProcessor::get_shader($facade,$name, vert_src,frag_src,$uniforms,$features)
        }
    };
}

pub mod background;
pub mod raster;

pub mod fill;
pub mod line;

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
    fn new_tile(&mut self, display: &Display, data: &Rc<tiles::TileData>) -> Result<()>;

    /// Called before start of rendering each frame, layer should request needed resources
    fn prepare(&mut self, params: render::PrepareParams) -> Result<()>;

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

pub trait LayerNew {
    type StyleLayer: style::StyleLayer;

    fn new(facade: &Display, style_layer: &Self::StyleLayer) -> Self;
}

pub trait WithSource {
    type Source: Debug = ();
    fn source_name(&self) -> Option<&str>;
}

pub trait Bucket: Debug {
    fn needs_explicit_eval(&self) -> bool {
        false
    }
    fn upload(&mut self, display: &Display) -> Result<()> {
        Ok(())
    }
}

pub trait BucketLayer: Debug + WithSource {
    type Bucket: Bucket;

    fn begin_pass(&mut self, params: &mut render::RenderParams, pass: RenderPass) -> Result<()> {
        Ok(())
    }

    fn end_pass(&mut self, params: &mut render::RenderParams, pass: RenderPass) -> Result<()> {
        Ok(())
    }

    fn new_tile(&mut self, display: &Display, tile: &Rc<tiles::TileData>) -> Result<Option<Self::Bucket>>;


    fn eval_layer(&mut self, params: &render::EvaluationParams) -> Result<()> {
        Ok(())
    }
    fn eval_bucket(&mut self, params: &render::EvaluationParams, bucket: &mut Self::Bucket) -> Result<()>;

    fn render_bucket(&mut self, params: &mut render::RenderParams, coords: UnwrappedTileCoords, bucket: &Self::Bucket) -> Result<()>;
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
    pub tiles: BTreeSet<UnwrappedTileCoords>,
}

impl<L: BucketLayer> BucketLayerHolder<L> {
    pub fn new(l: L) -> Self {
        BucketLayerHolder {
            layer: l,
            buckets: BTreeMap::new(),
            tiles: BTreeSet::new(),
        }
    }
}

impl<L: BucketLayer> Layer for BucketLayerHolder<L> {
    fn new_tile(&mut self, display: &Display, data: &Rc<tiles::TileData>) -> Result<()> {
        let coords = data.coord;
        if let Some(bucket) = self.layer.new_tile(display, data)? {
            self.buckets.insert(coords, BucketState {
                bucket,
                evaluated: None,
            });
        };

        Ok(())
    }

    fn prepare(&mut self, mut params: render::PrepareParams) -> Result<()> {
        let (next, missing) = self.get_renderable_tiles(&params.cover);
        self.tiles = next;
        if let Some(name) = self.layer.source_name() {
            for m in missing {
                (params.requestor)(name, m.wrap());
            }
        }

        Ok(())
    }


    /// TODO, better system for re-evaluating and uploading  modified data,
    /// Only re-evaluate on zoom change of integer coordinates ?
    fn evaluate(&mut self, params: &render::EvaluationParams) -> Result<()> {
        self.layer.eval_layer(params)?;
        let zoom = params.zoom;
        for t in self.tiles.iter() {
            if let Some(mut v) = self.buckets.get_mut(&t.wrap()) {
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
        }
        Ok(())
    }

    fn render(&mut self, params: &mut render::RenderParams) -> Result<()> {
        self.layer.begin_pass(params, RenderPass::Opaque)?;


        for t in self.tiles.iter() {
            if let Some(mut v) = self.buckets.get_mut(&t.wrap()) {
                v.bucket.upload(params.display)?;
                self.layer.render_bucket(params, *t, &mut v.bucket)?;
            }
        }

        self.layer.end_pass(params, RenderPass::Opaque)?;
        Ok(())
    }
}

impl<L: BucketLayer> BucketLayerHolder<L> {
    fn get_renderable_tiles(&self, cover: &TileCover) -> (BTreeSet<UnwrappedTileCoords>, BTreeSet<UnwrappedTileCoords>) {
        let mut expected_tiles = cover.tiles().clone();
        let mut missing = BTreeSet::new();
        for i in 0..10 {
            let mut to_add = BTreeSet::new();
            let mut to_remove = BTreeSet::new();


            for t in expected_tiles.iter() {
                if !self.buckets.contains_key(&t.wrap()) {
                    if let Some(p) = t.parent() {
                        to_add.insert(p);
                        missing.insert(*t);
                        to_remove.insert(*t);
                    }
                }
            }

            expected_tiles.extend(to_add.iter());
            for r in to_remove {
                expected_tiles.remove(&r);
            }
        }
        (expected_tiles, missing)
    }
}


pub fn parse_style_layers(facade: &Display, style: &style::Style) -> Vec<Box<dyn Layer>> {
    let mut res: Vec<Box<Layer>> = vec![];
    for l in style.layers.iter() {
        match l {
            style::BaseStyleLayer::Background(l) => {
                res.push(box background::BackgroundLayer::new(facade, l))
            }
            style::BaseStyleLayer::Fill(l) => {
                res.push(box BucketLayerHolder::new(fill::FillLayer::new(facade, l)))
            }
            style::BaseStyleLayer::Line(l) => {
                //res.push(box BucketLayerHolder::new(line::LineLayer::new(facade, l)))
            }
            style::BaseStyleLayer::Raster(l) => {
                res.push(box BucketLayerHolder::new(raster::RasterLayer::new(facade, l)))
            }
            _ => {}
        }
    }
    res
}