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

use super::filter::Filter;


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

impl Into<bool> for Visibility {
    fn into(self) -> bool {
        match self {
            Visibility::Visible => true,
            _ => false,
        }
    }
}

impl Default for Visibility {
    fn default() -> Self {
        Visibility::Visible
    }
}


#[derive(Deserialize, Debug, Clone)]
pub struct BaseLayout {
    #[serde(default = "Default::default")]
    pub visibility: Function<String>
}

impl Default for BaseLayout {
    fn default() -> Self {
        BaseLayout {
            visibility: Default::default()
        }
    }
}

pub trait StyleLayer {
    type PaintType;
    type LayoutType;

    fn get_paint(&self) -> &Self::PaintType;
    fn get_layout(&self) -> &Self::LayoutType;
}


#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum Function<T : super::expr::DescribeType + Debug> {
    Value(T),
    Expr(super::expr::TypedExpr<T>),
}


impl<T: Debug + Default + super::expr::DescribeType> Default for Function<T> {
    fn default() -> Self {
        Function::Value(Default::default())
    }
}
