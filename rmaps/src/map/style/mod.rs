use prelude::*;

use common::json;

mod expr;
mod function;
mod color;

mod layers;

pub use self::layers::*;


use self::function::Function;

use self::color::Color;


#[derive(Debug,Deserialize,Clone)]
pub struct TileJson {
    scheme: Option<String>,
    tiles: Option<Vec<String>>,
    minzoom: Option<f32>,
    maxzoom: Option<f32>,
    bounds: Option<[f32; 4]>,
    #[serde(rename = "tileSize")]
    tile_size : Option<i32>,

}
#[derive(Debug,Deserialize,Clone)]
pub struct SourceData{
    url : Option<String>,
    #[serde(flatten)]
    tilejson : TileJson,
}

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
    Vector(SourceData),
    #[serde(rename = "raster")]
    Raster(SourceData),
    #[serde(rename = "image")]
    Image(SourceData)
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


/*
#[test]
fn test_style_parse() {
    println!("{:#?}", json::from_str::<Style>(include_str!("bright.json")).unwrap());
}
*/