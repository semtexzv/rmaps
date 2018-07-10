use prelude::*;

use map::layers;
use map::style;
use map::tiles::data;
use std::hash;

pub struct CameraProperties {
    zoom: f64
}

pub struct FeatureProperties {
    feature: ::mapbox_tiles::Feature,
}

pub struct RenderParams<'a> {
    pub disp: &'a Display,
    pub frame: &'a mut glium::Frame,
    pub projection: cgmath::Matrix4<f32>,
    pub view: cgmath::Matrix4<f32>,
    pub zoom: f64,
}

#[derive(Debug)]
pub struct LayerData {
    pub layer: Box<layers::Layer>,
    pub buckets: BTreeMap<TileCoords, RenderBucket>,
}

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
                buckets: BTreeMap::new(),
            }
        }).collect();
        Ok(())
    }
    pub fn tile_ready(&mut self, tile: data::TileData) {
        for l in self.layers.iter_mut() {
            if l.layer.uses_source(&tile.source) {
                let bucket = l.layer.create_bucket(&self.display, &tile).unwrap();
                l.buckets.insert(tile.coord, bucket);
            }
        }
    }
    pub fn render(&mut self, mut params: RenderParams) -> Result<()> {
        for l in self.layers.iter_mut() {
            l.layer.render_begin(&mut params);
            for (c, b) in l.buckets.iter() {
                l.layer.render_tile(&mut params, *c, &b).unwrap();
            }
            l.layer.render_end(&mut params);
        }
        Ok(())
    }
}


#[derive(Debug, Clone)]
pub struct LineBucket {}

#[derive(Debug)]
pub enum RenderBucket {
    NoOp,
    Raster(layers::raster::RasterBucket),
    Fill(layers::fill::FillBucket),
    Line(LineBucket),
}