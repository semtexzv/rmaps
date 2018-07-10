use prelude::*;

use map::render;
use map::style;
use map::tiles::data;

macro_rules! rmaps_program {
    ($f:expr,$name:expr) => {
        {
            let vert_prelude = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../shaders/_prelude.vert.glsl"));
            let frag_prelude = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../shaders/_prelude.frag.glsl"));

            let vert_src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../shaders/",$name,".vert.glsl"));
            let frag_src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../shaders/",$name,".frag.glsl"));

            let vert_src = format!("{}\n{}\n",vert_prelude,vert_src);
            let frag_src = format!("{}\n{}\n",frag_prelude,frag_src);

            //panic!("VERT: {}\n FRAG: {}\n",vert_src,frag_src);


            program!($f,
                100 es => {
                    vertex: &vert_src,
                    fragment: &frag_src,
                }
            )
        }
    };
}

pub mod property;
pub mod background;
pub mod raster;
pub mod fill;

#[repr(C)]
#[derive(Debug, Clone, Copy, Vertex)]
pub struct Vertex {
    #[glium(attr = "pos")]
    pos: [f32; 2]
}

pub trait Layer: Debug {
    fn render_begin(&mut self, params: &mut render::RenderParams);

    fn render_tile(&mut self, params: &mut render::RenderParams,
                   tile: TileCoords,
                   bucket: &render::RenderBucket) -> Result<()>;

    fn render_end(&mut self, params: &mut render::RenderParams);

    fn uses_source(&mut self, source: &str) -> bool;

    fn create_bucket(&mut self, display: &Display, data: &data::TileData) -> Result<render::RenderBucket>;
}

pub fn parse_style_layers(facade: &Display, style: &style::Style) -> Vec<Box<Layer>> {
    let mut res: Vec<Box<Layer>> = vec![];
    for l in style.layers.iter() {
        match l {
            style::BaseStyleLayer::Background(l) => {
                res.push(Box::new(background::BackgroundLayer::parse(l.clone())))
            }
            style::BaseStyleLayer::Fill(l) => {
                res.push(Box::new(fill::FillLayer::parse(facade, l.clone())))
            }
            style::BaseStyleLayer::Raster(l) => {
                res.push(Box::new(raster::RasterLayer::parse(facade, l.clone())))
            }
            _ => {}
        }
    }
    res
}