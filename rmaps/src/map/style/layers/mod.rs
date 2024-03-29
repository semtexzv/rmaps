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

pub mod defaults {
    use super::*;

    pub fn default_antialias() -> StyleProp<bool> {
        true.into()
    }

    pub fn default_color() -> StyleProp<Color> {
        StyleProp::Value(Color::default())
    }

    pub fn default_opacity() -> StyleProp<f32> {
        StyleProp::Value(1.0)
    }

    pub fn default_outline_color() -> Option<StyleProp<Color>> {
        None.into()
    }

    pub fn default_translate() -> StyleProp<[f32; 2]> {
        [0., 0.].into()
    }

    pub fn default_line_width() -> StyleProp<f32> { 1.0.into() }

    pub fn default_gap_width() -> StyleProp<f32> { 0.0.into() }
}


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

use super::expr::{
    DescribeType,
    TypedExpr,
    Type,
    Value,
};

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum StyleProp<T: DescribeType + Debug> {
    Value(T),
    Expr(TypedExpr<T>),
}

impl<T> From<Option<StyleProp<T>>> for StyleProp<Option<T>>
    where T: DescribeType,
          Option<T>: DescribeType
{
    fn from(v: Option<StyleProp<T>>) -> Self {
        match v {
            Some(StyleProp::Value(v)) => StyleProp::Value(Some(v)),
            None => StyleProp::Value(None),
            Some(StyleProp::Expr(TypedExpr(e, _))) => StyleProp::Expr(TypedExpr::new(e)),
        }
    }
}

impl<T: DescribeType + Debug> StyleProp<T> {
    pub fn is_zoom(&self) -> bool {
        match self {
            StyleProp::Expr(e) => e.is_zoom(),
            _ => false
        }
    }

    pub fn is_feature(&self) -> bool {
        match self {
            StyleProp::Expr(e) => e.is_feature(),
            _ => false
        }
    }
}

impl<T: DescribeType + Debug> From<T> for StyleProp<T> {
    fn from(v: T) -> Self {
        StyleProp::Value(v)
    }
}


impl<T: Debug + Default + super::expr::DescribeType> Default for StyleProp<T> {
    fn default() -> Self {
        StyleProp::Value(Default::default())
    }
}
