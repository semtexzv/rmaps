use prelude::*;

use super::{
    LayerCommon,
    BaseLayout,
    Visibility,
    StyleLayer,
    StyleProp,
};

#[derive(Deserialize, Debug, Clone)]
pub struct SymbolLayer {
    #[serde(flatten)]
    pub  common: LayerCommon,
    #[serde(default)]
    pub  layout: SymbolLayout,
    #[serde(default)]
    pub paint: SymbolPaint,
}

impl StyleLayer for SymbolLayer {
    type PaintType = SymbolPaint;
    type LayoutType = SymbolLayout;

    fn get_paint(&self) -> &Self::PaintType { &self.paint }
    fn get_layout(&self) -> &Self::LayoutType { &self.layout }
}

#[derive(Deserialize,Default, Debug, Clone)]
pub struct SymbolLayout {
    #[serde(default, rename = "symbol-placement")]
    placement: StyleProp<Option<String>>,

    #[serde(default, rename = "symbol-spacing")]
    spacing: StyleProp<Option<f32>>,

    #[serde(rename = "symbol-avoid-edges")]
    avoid_edges: Option<bool>,
}

#[derive(Deserialize,Default, Debug, Clone)]
pub struct SymbolPaint {}