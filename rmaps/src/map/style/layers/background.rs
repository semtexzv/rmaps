use prelude::*;

use super::{
    LayerCommon,
    BaseLayout,
    Visibility,
};

use super::super::function::Function;
use super::super::color::Color;

#[derive(Debug, Deserialize, Clone)]
pub struct BackgroundLayer {
    #[serde(flatten)]
    pub common: LayerCommon,
    #[serde(default = "Default::default")]
    pub layout: BaseLayout,
    #[serde(default = "Default::default")]
    pub paint: BackgroundPaint,
}


#[derive(Deserialize, Debug, Clone)]
pub struct BackgroundPaint {
    #[serde(rename = "background-color")]
    #[serde(default = "default_background_color")]
    pub color: Function<Color>,

    #[serde(rename = "background-opacity")]
    #[serde(default = "default_backround_opacity")]
    pub opacity: Function<f32>,

    #[serde(rename = "background-pattern")]
    pub pattern: Option<Function<String>>,
}

fn default_background_color() -> Function<Color> {
    return Function::Raw(Color(::css_color_parser::Color::from_str("#00000").unwrap()));
}

fn default_backround_opacity() -> Function<f32> {
    return Function::Raw(1.0);
}

impl Default for BackgroundPaint {
    fn default() -> Self {
        BackgroundPaint {
            color: default_background_color(),
            opacity: default_backround_opacity(),
            pattern: None,
        }
    }
}