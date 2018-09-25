use ::prelude::*;

use map::style::StyleProp;
use map::render::property::*;

fn get_outline_color(style: &::map::style::FillLayer) -> &StyleProp<Color> {
    if let Some(ref color) = style.paint.outline_color {
        &color
    } else {
        &style.paint.color
    }
}


fn get_pattern(style: &::map::style::FillLayer) -> StyleProp<Option<String>> {
    style.paint.pattern.clone().into()
}

#[derive(Debug, Clone, Default, LayerProperties)]
#[properties(FillLayer)]
pub struct FillLayerProperties {
    #[property(paint = "antialias")]
    pub antialias: Property<bool, True, False>,
    #[property(custom = "get_pattern")]
    pub pattern: Property<Option<String>, False, False>,

}


#[derive(Debug, Clone, Default, PaintProperties)]
#[properties(FillLayer)]
pub struct FillFeatureProperties {
    #[property(paint = "opacity")]
    opacity: Property<f32>,
    #[property(paint = "color")]
    color: Property<Color>,
    #[property(custom = "get_outline_color")]
    outline_color: Property<Color>,
}
