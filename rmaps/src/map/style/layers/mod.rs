use prelude::*;

pub mod background;
pub mod fill;
pub mod line;
pub mod raster;
pub mod symbol;


pub use self::background::*;
pub use self::fill::*;
pub use self::line::*;
pub use self::raster::*;
pub use self::symbol::*;

use super::expr::Filter;

#[derive(Deserialize, Debug, Clone)]
pub struct LayerCommon {
    pub id: String,
    pub source: Option<String>,
    #[serde(rename = "source-layer")]
    pub source_layer: Option<String>,
    pub minzoom: Option<f32>,
    pub maxzoom: Option<f32>,
    pub filter: Option<Filter>,
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


