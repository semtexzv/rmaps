use prelude::*;

use super::{
    LayerCommon,
    BaseLayout,
    Visibility,
};

use super::super::function::Function;
use super::super::color::Color;

#[derive(Deserialize, Debug, Clone)]
pub struct SymbolLayer {
    #[serde(flatten)]
    pub  common: LayerCommon,
    pub  layout: Option<SymbolLayout>,
    pub paint: Option<SymbolPaint>,
}


#[derive(Deserialize, Debug, Clone)]
pub struct SymbolLayout {
    #[serde(rename = "symbol-placement")]
    placement: Option<Function<String>>,

    #[serde(rename = "symbol-spacing")]
    spacing: Option<Function<f32>>,

    #[serde(rename = "symbol-avoid-edges")]
    avoid_edges: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SymbolPaint {}