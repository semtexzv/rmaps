use prelude::*;
use super::Vertex;
use map::{
    style,
    render::{
        self,
        layers::{
            self, Layer,
        },
        property::*,
    },
    tiles::{
        self
    },
};


use map::render::shaders::{
    UniformPropertyLayout,
    FeaturePropertyLayout,
    PropertyItemLayout,
};

pub mod bucket;

#[derive(Debug)]
pub struct LineLayer {
    style_layer: style::FillLayer,
    pub shader_program: Rc<glium::Program>,
    //pub properties: FillLayerProperties,
    pub layout: (UniformPropertyLayout, FeaturePropertyLayout),
}

impl layers::LayerNew for LineLayer {
    type StyleLayer = style::LineLayer;

    fn new(facade: &Display, style_layer: &<Self as layers::LayerNew>::StyleLayer) -> Self {
        unimplemented!()
    }
}