use prelude::*;


use super::{
    LayerCommon,
    BaseLayout,
    Visibility,
    Function,
};


#[derive(Deserialize, Debug, Clone)]
pub struct LineLayer {
    #[serde(flatten)]
    pub common: LayerCommon,
    pub layout: Option<LineLayout>,
    pub paint: Option<LinePaint>,
}


#[derive(Deserialize, Debug, Clone)]
pub struct LineLayout {
    #[serde(rename = "line-cap")]
    cap: Option<Function<String>>,
    #[serde(rename = "line-join")]
    join: Option<Function<String>>,
    #[serde(rename = "line-miter-limit")]
    miter_limit: Option<Function<f32>>,
    #[serde(rename = "line-round-limit")]
    round_limit: Option<Function<f32>>,

    visibility: Option<Visibility>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LinePaint {
    #[serde(rename = "line-opacity")]
    opacity: Option<Function<f32>>,

    #[serde(rename = "line-color")]
    color: Option<Function<Color>>,

    #[serde(rename = "line-translate")]
    translate: Option<Function<[f32; 2]>>,

    #[serde(rename = "line-translate-anchor")]
    translate_anchor: Option<String>,

    #[serde(rename = "line-width")]
    width: Option<Function<f32>>,

    #[serde(rename = "line-gap_width")]
    gap_width: Option<Function<f32>>,

    #[serde(rename = "line-offset")]
    offset: Option<Function<f32>>,

    #[serde(rename = "line-blur")]
    blur: Option<Function<f32>>,

    #[serde(rename = "line-dasharray")]
    dash_array: Option<Function<Vec<f32>>>,

    #[serde(rename = "line-pattern")]
    pattern: Option<Function<String>>,
}