use prelude::*;


use super::{
    LayerCommon,
    StyleLayer,
    BaseLayout,
    Visibility,
    StyleProp,
};

#[derive(Deserialize, Debug, Clone)]
pub struct FillLayer {
    #[serde(flatten)]
    pub common: LayerCommon,
    #[serde(default = "BaseLayout::default")]
    pub layout: BaseLayout,
    #[serde(default = "FillPaint::default")]
    pub paint: FillPaint,
}

impl StyleLayer for FillLayer {
    type PaintType = FillPaint;
    type LayoutType = BaseLayout;

    fn get_paint(&self) -> &Self::PaintType { &self.paint }
    fn get_layout(&self) -> &Self::LayoutType { &self.layout }
}

#[derive(Deserialize, Default, Debug, Clone)]
pub struct FillPaint {
    #[serde(rename = "fill-antialias")]
    #[serde(default)]
    pub antialias: StyleProp<bool>,

    #[serde(rename = "fill-opacity")]
    #[serde(default = "Default::default")]
    pub  opacity: StyleProp<f32>,

    #[serde(rename = "fill-color")]
    #[serde(default = "Default::default")]
    pub   color: StyleProp<Color>,

    #[serde(rename = "fill-outline-color")]
    #[serde(default = "Default::default")]
    pub  outline_color: StyleProp<Color>,

    #[serde(rename = "fill-translate")]
    #[serde(default = "Default::default")]
    pub   translate: StyleProp<[f32; 2]>,

    #[serde(rename = "fill-translate-anchor")]
    #[serde(default = "Default::default")]
    pub translate_anchor: StyleProp<Option<String>>,

    #[serde(rename = "fill-pattern")]
    #[serde(default = "Default::default")]
    pub pattern: StyleProp<Option<String>>,
}

