use prelude::*;
use map::util::profiler;
use map::{
    style,
    render::{
        self,
        layers::{
            self, Layer, LayerNew,
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
    color: Property<Color, True, False>,
    #[property(paint = "opacity")]
    opacity: Property<f32, True, False>,
    #[property(paint = "pattern", nofeature, nozoom)]
    pattern: Property<Option<String>>,
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

impl LayerNew for BackgroundLayer {
    type StyleLayer = style::BackgroundLayer;

    fn new(facade: &Display, style_layer: &<Self as LayerNew>::StyleLayer) -> Self {
        return BackgroundLayer {
            style_layer: style_layer.clone(),
            properties: Default::default(),
        };
    }
}
