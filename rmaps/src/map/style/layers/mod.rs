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
    pub visibility: StyleProp<String>
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

use super::expr::DescribeType;

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum StyleProp<T: DescribeType + Debug> {
    Value(T),
    Expr(super::expr::TypedExpr<T>),
}

impl<T: DescribeType + Debug> StyleProp<T> {
    pub fn is_zoom(&self) -> bool {
        return if let StyleProp::Expr(e) = self {
            e.is_zoom()
        } else {
            false
        };
    }

    pub fn is_feature(&self) -> bool {
        return if let StyleProp::Expr(e) = self {
            e.is_feature()
        } else {
            false
        };
    }
}


impl<T: Debug + Default + super::expr::DescribeType> Default for StyleProp<T> {
    fn default() -> Self {
        StyleProp::Value(Default::default())
    }
}
