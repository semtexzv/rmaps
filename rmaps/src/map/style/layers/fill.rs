use prelude::*;

use super::{
    LayerCommon,
    BaseLayout,
    Visibility,
};

use super::super::function::Function;
use super::super::color::Color;

#[derive(Deserialize, Debug, Clone)]
pub struct FillPaint {
    #[serde(rename = "fill-antialias")]
    pub antialias: Option<Function<bool>>,

    #[serde(rename = "fill-opacity")]
    pub  opacity: Option<Function<f32>>,

    #[serde(rename = "fill-color")]
    pub   color: Option<Function<Color>>,

    #[serde(rename = "fill-outline-color")]
    pub    outline_color: Option<Function<Color>>,

    #[serde(rename = "fill-translate")]
    pub   translate: Option<Function<[f32; 2]>>,

    #[serde(rename = "fill-translate-anchor")]
    pub translate_anchor: Option<String>,

    #[serde(rename = "fill-pattern")]
    pub pattern: Option<Function<String>>,
}


#[derive(Deserialize, Debug, Clone)]
pub struct FillLayer {
    #[serde(flatten)]
    pub common: LayerCommon,
    #[serde(default = "BaseLayout::default")]
    pub layout: BaseLayout,
    pub paint: Option<FillPaint>,
}