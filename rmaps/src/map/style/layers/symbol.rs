use prelude::*;

use super::{
    LayerCommon,
    BaseLayout,
    Visibility,
    StyleProp,
};

#[derive(Deserialize, Debug, Clone)]
pub struct SymbolLayer {
    #[serde(flatten)]
    pub  common: LayerCommon,
    pub  layout: Option<SymbolLayout>,
    pub paint: Option<SymbolPaint>,
}


#[derive(Deserialize, Debug, Clone)]
pub struct SymbolLayout {
    #[serde(default, rename = "symbol-placement")]
    placement: StyleProp<Option<String>>,

    #[serde(default, rename = "symbol-spacing")]
    spacing: StyleProp<Option<f32>>,

    #[serde(rename = "symbol-avoid-edges")]
    avoid_edges: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SymbolPaint {}