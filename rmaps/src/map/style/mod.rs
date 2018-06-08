use prelude::*;

use common::json;

mod expr;
mod function;
mod color;

mod layers;

pub use self::layers::*;


use self::function::Function;

use self::color::Color;


#[derive(Deserialize, Debug, Clone)]
pub struct Style {
    pub version: i32,
    pub name: Option<String>,
    //metadata: json::Value,
    pub center: Option<[f64; 2]>,
    pub zoom: Option<f32>,
    pub sources: BTreeMap<String, StyleSource>,
    pub sprite: Option<String>,
    pub glyphs: Option<String>,
    pub layers: Vec<StyleLayer>,

}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum StyleSource {
    #[serde(rename = "vector")]
    Vector {},
    #[serde(rename = "raster")]
    Raster {},
    #[serde(rename = "image")]
    Image {},
}

#[derive(Deserialize, Debug, Clone)]
pub struct LayerCommon {
    pub id: String,
    pub source: Option<String>,
    #[serde(rename = "source-layer")]
    pub source_layer: Option<String>,
    pub minzoom: Option<f32>,
    pub maxzoom: Option<f32>,
    pub filter: Option<expr::FilterVal>,
}

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
pub struct FillLayer {
    #[serde(flatten)]
    pub common: LayerCommon,
    #[serde(default = "BaseLayout::default")]
    pub layout: BaseLayout,
    pub paint: Option<FillPaint>,
}


#[derive(Deserialize, Debug, Clone)]
pub struct LineLayer {
    #[serde(flatten)]
    pub common: LayerCommon,
    pub layout: Option<LineLayout>,
    pub paint: Option<LinePaint>,
}


#[derive(Deserialize, Debug, Clone)]
pub struct SymbolLayer {
    #[serde(flatten)]
    pub  common: LayerCommon,
    pub  layout: Option<SymbolLayout>,
    pub paint: Option<SymbolPaint>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RasterLayer {
    #[serde(flatten)]
    pub common: LayerCommon,
    #[serde(default = "BaseLayout::default")]
    pub layout: BaseLayout,
    pub  paint: Option<RasterPaint>,
}


#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum StyleLayer {
    #[serde(rename = "background")]
    Background(BackgroundLayer),
    #[serde(rename = "fill")]
    Fill(FillLayer),
    #[serde(rename = "line")]
    Line(LineLayer),
    #[serde(rename = "symbol")]
    Symbols(SymbolLayer),
    #[serde(rename = "raster")]
    Raster(RasterLayer),
}

#[derive(Deserialize, Debug, Clone)]
pub enum Visibility {
    #[serde(rename = "visible")]
    Visible,
    #[serde(rename = "none")]
    Invisible,
}

impl Default for Visibility {
    fn default() -> Self {
        Visibility::Visible
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct BaseLayout {
    #[serde(default = "Visibility::default")]
    visibility: Visibility
}

impl Default for BaseLayout {
    fn default() -> Self {
        BaseLayout {
            visibility: Default::default()
        }
    }
}



#[derive(Deserialize, Debug, Clone)]
pub struct BackgroundPaint {
    #[serde(rename = "background-color")]
    #[serde(default = "default_background_color")]
    pub color: Function<color::Color>,

    #[serde(rename = "background-opacity")]
    #[serde(default = "default_backround_opacity")]
    pub opacity: Function<f32>,

    #[serde(rename = "background-pattern")]
    pub pattern: Option<Function<String>>,
}

use std::str::FromStr;
fn default_background_color() -> Function<color::Color> {
    return Function::Raw(color::Color(::css_color_parser::Color::from_str("#00000").unwrap()));
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

#[derive(Deserialize, Debug, Clone)]
pub struct FillPaint {
    #[serde(rename = "fill-antialias")]
    pub antialias: Option<Function<bool>>,

    #[serde(rename = "fill-opacity")]
    pub  opacity: Option<Function<f32>>,

    #[serde(rename = "fill-color")]
    pub    color: Option<Function<color::Color>>,

    #[serde(rename = "fill-outline-color")]
    pub    outline_color: Option<Function<color::Color>>,

    #[serde(rename = "fill-translate")]
    pub   translate: Option<Function<[f32; 2]>>,

    #[serde(rename = "fill-translate-anchor")]
    pub translate_anchor: Option<String>,

    #[serde(rename = "fill-pattern")]
    pub pattern: Option<Function<String>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LineLayout {
    #[serde(rename = "line-cap")]
    cap: Option<String>,
    #[serde(rename = "line-join")]
    join: Option<String>,
    #[serde(rename = "line-miter-limit")]
    miter_limit: Option<Function<f32>>,
    #[serde(rename = "line-round-limit")]
    round_limit: Option<Function<f32>>,

    visibility: Option<Visibility>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LinePaint {
    #[serde(rename = "line-opacity")]
    opacity: Option<Function<f32>>,

    #[serde(rename = "line-color")]
    color: Option<Function<Color>>,

    #[serde(rename = "line-translate")]
    translate: Option<Function<[f32; 2]>>,

    #[serde(rename = "line-translate-anchor")]
    translate_anchor: Option<String>,

    #[serde(rename = "line-width")]
    width: Option<Function<f32>>,

    #[serde(rename = "line-gap_width")]
    gap_width: Option<Function<f32>>,

    #[serde(rename = "line-offset")]
    offset: Option<Function<f32>>,

    #[serde(rename = "line-blur")]
    blur: Option<Function<f32>>,

    #[serde(rename = "line-dasharray")]
    dash_array: Option<Function<Vec<f32>>>,

    #[serde(rename = "line-pattern")]
    pattern: Option<Function<String>>,
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

#[test]
fn test_style_parse() {
    println!("{:#?}", json::from_str::<Style>(include_str!("bright.json")).unwrap());
}