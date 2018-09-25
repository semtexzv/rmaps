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

#[derive(Debug, Clone, Default, Properties)]
#[properties(LineLayer)]
pub struct LineLayerProperties {
    /*
    #[property(paint = "antialias", nofeature)]
    pub antialias: PaintProperty<bool>,
    */
}


#[derive(Debug, Clone, Default, Properties)]
#[properties(LineLayer)]
pub struct LineFeatureProperties {
    #[property(paint = "opacity")]
    opacity: Property<f32>,
    #[property(paint = "color")]
    color: Property<Color>,

    #[property(paint = "width")]
    width: Property<f32>,
    #[property(paint = "gap_width")]
    gap_width: Property<f32>,
}
