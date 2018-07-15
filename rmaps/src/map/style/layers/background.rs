use prelude::*;

use super::{
    LayerCommon,
    StyleLayer,
    BaseLayout,
    Visibility,
    StyleProp,
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
    pub color: StyleProp<Color>,

    #[serde(rename = "background-opacity")]
    #[serde(default = "default_backround_opacity")]
    pub opacity: StyleProp<f32>,

    #[serde(rename = "background-pattern")]
    #[serde(default)]
    pub pattern: StyleProp<Option<String>>,
}

fn default_background_color() -> StyleProp<Color> {
    return StyleProp::Value(Color([0., 0., 0., 1.]));
}

fn default_backround_opacity() -> StyleProp<f32> {
    return StyleProp::Value(1.0);
}

impl Default for BackgroundPaint {
    fn default() -> Self {
        BackgroundPaint {
            color: default_background_color(),
            opacity: default_backround_opacity(),
            pattern: StyleProp::Value(None),
        }
    }
}