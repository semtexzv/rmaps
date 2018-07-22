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

#[derive(Deserialize, Debug, Clone)]
pub struct FillPaint {
    #[serde(rename = "fill-antialias")]
    #[serde(default)]
    pub antialias: StyleProp<bool>,

    #[serde(rename = "fill-opacity")]
    #[serde(default = "default_opacity")]
    pub  opacity: StyleProp<f32>,

    #[serde(rename = "fill-color")]
    #[serde(default = "default_color")]
    pub   color: StyleProp<Color>,

    #[serde(rename = "fill-outline-color")]
    #[serde(default = "default_outline_color")]
    pub  outline_color: Option<StyleProp<Color>>,

    #[serde(rename = "fill-translate")]
    #[serde(default = "default_translate")]
    pub   translate: StyleProp<[f32; 2]>,

    #[serde(rename = "fill-translate-anchor")]
    #[serde(default = "Default::default")]
    pub translate_anchor: StyleProp<Option<String>>,

    #[serde(rename = "fill-pattern")]
    #[serde(default = "Default::default")]
    pub pattern: Option<StyleProp<String>>,
}

pub fn default_antialiass() -> StyleProp<bool> {
    true.into()
}

pub fn default_color() -> StyleProp<Color> {
    StyleProp::Value(Color::default())
}

pub fn default_opacity() -> StyleProp<f32> {
    StyleProp::Value(1.0)
}

pub fn default_outline_color() -> Option<StyleProp<Color>> {
    None.into()
}

pub fn default_translate() -> StyleProp<[f32; 2]> {
    [0., 0.].into()
}

impl Default for FillPaint {
    fn default() -> Self {
        FillPaint {
            antialias: default_antialiass(),
            color: default_color(),
            opacity: default_opacity(),
            outline_color: default_outline_color(),
            translate: default_translate(),
            translate_anchor: None::<String>.into(),
            pattern: None,

        }
    }
}