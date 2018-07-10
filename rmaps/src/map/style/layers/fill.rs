use prelude::*;


use super::{
    LayerCommon,
    StyleLayer,
    BaseLayout,
    Visibility,
    Function,
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
    pub antialias: Function<bool>,

    #[serde(rename = "fill-opacity")]
    #[serde(default = "Default::default")]
    pub  opacity: Function<f32>,

    #[serde(rename = "fill-color")]
    #[serde(default = "Default::default")]
    pub   color: Function<Color>,

    #[serde(rename = "fill-outline-color")]
    #[serde(default = "Default::default")]
    pub  outline_color: Function<Color>,

    #[serde(rename = "fill-translate")]
    #[serde(default = "Default::default")]
    pub   translate: Function<[f32; 2]>,

    #[serde(rename = "fill-translate-anchor")]
    #[serde(default = "Default::default")]
    pub translate_anchor: Option<String>,

    #[serde(rename = "fill-pattern")]
    #[serde(default = "Default::default")]
    pub pattern: Option<Function<String>>,
}

