use prelude::*;

use super::{
    LayerCommon,
    BaseLayout,
    Visibility,
    StyleLayer,
    StyleProp,
    defaults::*,
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

#[derive(Deserialize, Debug, Clone)]
pub struct LinePaint {
    #[serde(rename = "line-opacity")]
    #[serde(default = "default_opacity")]
    pub opacity: StyleProp<f32>,

    #[serde(rename = "line-color")]
    #[serde(default = "default_color")]
    pub color: StyleProp<Color>,

    #[serde(rename = "line-translate")]
    #[serde(default = "default_translate")]
    pub translate: StyleProp<[f32; 2]>,

    #[serde(rename = "line-translate-anchor")]
    pub translate_anchor: Option<String>,

    #[serde(rename = "line-width")]
    #[serde(default = "default_line_width")]
    pub width: StyleProp<f32>,

    #[serde(rename = "line-gap_width")]
    #[serde(default = "default_gap_width")]
    pub gap_width: StyleProp<f32>,

    #[serde(rename = "line-offset")]
    pub offset: Option<StyleProp<f32>>,

    #[serde(rename = "line-blur")]
    pub blur: Option<StyleProp<f32>>,

    #[serde(rename = "line-dasharray")]
    pub  dash_array: Option<StyleProp<Vec<f32>>>,

    #[serde(rename = "line-pattern")]
    pub pattern: Option<StyleProp<String>>,

}

impl Default for LinePaint {
    fn default() -> Self {
        LinePaint {
            opacity: default_opacity(),
            color: default_color(),
            translate: default_translate(),
            translate_anchor: None,
            width: default_line_width(),
            gap_width: default_gap_width(),
            offset: None,
            blur: None,
            dash_array: None,
            pattern: None,
        }
    }
}


