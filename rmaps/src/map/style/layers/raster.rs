use prelude::*;

use super::{
    LayerCommon,
    BaseLayout,
    Visibility,
    StyleProp,
};

#[derive(Deserialize, Debug, Clone)]
pub struct RasterLayer {
    #[serde(flatten)]
    pub common: LayerCommon,
    #[serde(default = "BaseLayout::default")]
    pub layout: BaseLayout,
    pub  paint: Option<RasterPaint>,
}


#[derive(Deserialize, Debug, Clone)]
pub struct RasterPaint {
    #[serde(rename = "raster-opacity")]
    opacity: Option<StyleProp<f32>>,

    #[serde(rename = "raster-hue-rotate")]
    hue_rotate: Option<StyleProp<f32>>,

    #[serde(rename = "raster-brightness-min")]
    brightness_min: Option<StyleProp<f32>>,

    #[serde(rename = "raster-brightness-max")]
    brightness_max: Option<StyleProp<f32>>,

    #[serde(rename = "raster-saturation")]
    saturation: Option<StyleProp<f32>>,

    #[serde(rename = "raster-contrast")]
    contrast: Option<StyleProp<f32>>,

    #[serde(rename = "raster-fade-duration")]
    fade_duration: Option<StyleProp<f32>>,

}