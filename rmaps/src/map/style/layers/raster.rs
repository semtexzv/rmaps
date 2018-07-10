use prelude::*;

use super::{
    LayerCommon,
    BaseLayout,
    Visibility,
    Function,
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
    opacity: Option<Function<f32>>,

    #[serde(rename = "raster-hue-rotate")]
    hue_rotate: Option<Function<f32>>,

    #[serde(rename = "raster-brightness-min")]
    brightness_min: Option<Function<f32>>,

    #[serde(rename = "raster-brightness-max")]
    brightness_max: Option<Function<f32>>,

    #[serde(rename = "raster-saturation")]
    saturation: Option<Function<f32>>,

    #[serde(rename = "raster-contrast")]
    contrast: Option<Function<f32>>,

    #[serde(rename = "raster-fade-duration")]
    fade_duration: Option<Function<f32>>,

}