use prelude::*;

use super::{
    LayerCommon,
    StyleLayer,
    BaseLayout,
    Visibility,
    Function,
};

#[derive(Debug, Deserialize, Clone)]
pub struct BackgroundLayer {
    #[serde(flatten)]
    pub common: LayerCommon,
    #[serde(default = "Default::default")]
    pub layout: BaseLayout,
    #[serde(default = "Default::default")]
    pub paint: BackgroundPaint,
}

impl StyleLayer for BackgroundLayer {
    type PaintType = BackgroundPaint;
    type LayoutType = BaseLayout;

    fn get_paint(&self) -> &<Self as StyleLayer>::PaintType {
        &self.paint
    }

    fn get_layout(&self) -> &<Self as StyleLayer>::LayoutType {
        &self.layout
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct BackgroundPaint {
    #[serde(rename = "background-color")]
    #[serde(default = "default_background_color")]
    pub color: Function<Color>,

    #[serde(rename = "background-opacity")]
    #[serde(default = "default_backround_opacity")]
    pub opacity: Function<f32>,

    #[serde(rename = "background-pattern")]
    #[serde(default)]
    pub pattern: Function<Option<String>>,
}

fn default_background_color() -> Function<Color> {
    return Function::Value(Color([0., 0., 0., 1.]));
}

fn default_backround_opacity() -> Function<f32> {
    return Function::Value(1.0);
}

impl Default for BackgroundPaint {
    fn default() -> Self {
        BackgroundPaint {
            color: default_background_color(),
            opacity: default_backround_opacity(),
            pattern: Function::Value(None),
        }
    }
}