use prelude::*;

use ::map::style;
use map::render::*;
use map::tiles::data::*;
use super::property::*;


#[derive(Debug, Clone, Default, Properties)]
#[properties(BackgroundLayer)]
pub struct BackgroundLayerProperties {
    #[property(name = "color", nofeature)]
    color: BaseProperty<Color>,

    opacity: BaseProperty<f32>,
}

#[derive(Debug)]
pub struct BackgroundLayer {
    style_layer: style::BackgroundLayer,
    properties: BackgroundLayerProperties,
}

impl super::Layer for BackgroundLayer {
    fn render_begin(&mut self, params: &mut RenderParams) {
        let evaluator = PropertiesEvaluator::only_zoom(params.zoom);
        self.properties.eval(&self.style_layer, &evaluator);

        panic!("{:?}", self.properties);
        let mut c = self.properties.color.get().to_rgba();
        params.frame.clear_color(c[0], c[1], c[2], c[3]);
    }

    fn render_tile(&mut self, params: &mut RenderParams, tile: TileCoords, bucket: &RenderBucket) -> Result<()> {
        Ok(())
    }

    fn render_end(&mut self, params: &mut RenderParams) {}

    fn uses_source(&mut self, source: &str) -> bool {
        false
    }


    fn create_bucket(&mut self, display: &Display, data: &TileData) -> Result<RenderBucket> {
        return Ok(RenderBucket::NoOp);
    }
}

impl BackgroundLayer {
    pub fn parse(layer: style::BackgroundLayer) -> Self {
        return BackgroundLayer {
            style_layer: layer,
            properties: Default::default(),
        };
    }
}