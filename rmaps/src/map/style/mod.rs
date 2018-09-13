use prelude::*;

use common::json;

pub mod expr;
pub mod sprite;
mod filter;

mod layers;

pub use self::layers::*;

#[derive(Debug, Deserialize, Clone)]
pub struct TileJson {
    scheme: Option<String>,
    pub tiles: Option<Vec<String>>,
    minzoom: Option<f32>,
    maxzoom: Option<f32>,
    bounds: Option<[f32; 4]>,
    #[serde(rename = "tileSize")]
    tile_size: Option<i32>,
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
    pub layers: Vec<BaseStyleLayer>,

}

#[derive(Debug, Clone, Deserialize)]
pub enum SourceType {
    #[serde(rename = "raster")]
    Raster,
    #[serde(rename = "vector")]
    Vector,
    #[serde(rename = "geojson")]
    Geojson,
    #[serde(rename = "image")]
    Image,

}

#[derive(Debug, Deserialize, Clone)]
pub struct StyleSource {
    #[serde(flatten)]
    pub tilejson: TileJson,
    #[serde(rename = "type")]
    pub typ: SourceType,
    pub url: Option<String>,
}

impl StyleSource {
    pub fn tile_urls(&self) -> Vec<String> {
        return self.tilejson.tiles.as_ref().map(|x| x.clone()).unwrap_or(vec![]);
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum BaseStyleLayer {
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
    #[serde(rename = "fill-extrusion")]
    FillExtrusion(json::Value),
}


/*
#[test]
fn test_style_parse() {
    println!("{:#?}", json::from_str::<Style>(include_str!("bright.json")).unwrap());
}
*/