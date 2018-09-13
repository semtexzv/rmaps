use prelude::*;
use map::util::profiler;
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

#[derive(Debug, Clone, Default, Properties)]
#[properties(BackgroundLayer)]
pub struct BackgroundLayerProperties {
    #[property(paint = "color", nofeature)]
    color: PaintProperty<Color>,
    #[property(paint = "opacity")]
    opacity: PaintProperty<f32>,
    #[property(paint = "pattern", nofeature, nozoom)]
    pattern: PaintProperty<Option<String>>,
}

#[derive(Debug)]
pub struct BackgroundLayer {
    style_layer: style::BackgroundLayer,
    properties: BackgroundLayerProperties,
}

impl Layer for BackgroundLayer {
    fn new_tile(&mut self, display: &Display, data: &Rc<tiles::TileData>) -> Result<()> {
        Ok(())
    }

    fn prepare(&mut self, params: render::PrepareParams) -> Result<()> {
        Ok(())
    }


    fn evaluate(&mut self, params: &render::EvaluationParams) -> Result<()> {
        let evaluator = PropertiesEvaluator::from(params);
        self.properties.eval(&self.style_layer, &evaluator).unwrap();

        Ok(())
    }
    fn render(&mut self, params: &mut render::RenderParams) -> Result<()> {
        let mut c = self.properties.color.get().to_rgba();
        params.frame.clear_color(c[0], c[1], c[2], c[3] * self.properties.opacity.get());
        Ok(())
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