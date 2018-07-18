use prelude::*;


use super::{
    LayerCommon,
    BaseLayout,
    Visibility,
    StyleLayer,
    StyleLayerExt,
    StyleProp,
};


#[derive(Deserialize, Debug, Clone)]
pub struct LineLayer {
    #[serde(flatten)]
    pub common: LayerCommon,
    #[serde(default)]
    pub layout: LineLayout,
    #[serde(default)]
    pub paint: LinePaint,
}

impl StyleLayer for LineLayer {
    type PaintType = LinePaint;
    type LayoutType = LineLayout;

    fn get_paint(&self) -> &Self::PaintType { &self.paint }
    fn get_layout(&self) -> &Self::LayoutType { &self.layout }
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct LineLayout {
    #[serde(rename = "line-cap")]
    cap: Option<StyleProp<String>>,
    #[serde(rename = "line-join")]
    join: Option<StyleProp<String>>,
    #[serde(rename = "line-miter-limit")]
    miter_limit: Option<StyleProp<f32>>,
    #[serde(rename = "line-round-limit")]
    round_limit: Option<StyleProp<f32>>,

    visibility: Option<Visibility>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct LinePaint {
    #[serde(rename = "line-opacity")]
    opacity: Option<StyleProp<f32>>,

    #[serde(rename = "line-color")]
    color: Option<StyleProp<Color>>,

    #[serde(rename = "line-translate")]
    translate: Option<StyleProp<[f32; 2]>>,

    #[serde(rename = "line-translate-anchor")]
    translate_anchor: Option<String>,

    #[serde(rename = "line-width")]
    width: Option<StyleProp<f32>>,

    #[serde(rename = "line-gap_width")]
    gap_width: Option<StyleProp<f32>>,

    #[serde(rename = "line-offset")]
    offset: Option<StyleProp<f32>>,

    #[serde(rename = "line-blur")]
    blur: Option<StyleProp<f32>>,

    #[serde(rename = "line-dasharray")]
    dash_array: Option<StyleProp<Vec<f32>>>,

    #[serde(rename = "line-pattern")]
    pattern: Option<StyleProp<String>>,
}