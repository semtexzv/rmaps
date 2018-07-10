use prelude::*;

pub use serde::{
    Deserializer,
    Deserialize,
    de::DeserializeOwned,
};

pub use std::cell::RefCell;

pub fn parse_basics<'de, D: Deserializer<'de>>(name: &'static str, min_args: usize, deserializer: D) -> StdResult<Vec<json::Value>, D::Error> {
    use ::serde::de::Error;
    let mut arr: Vec<json::Value> = Deserialize::deserialize(deserializer)?;
    if arr.len() < min_args + 1 {
        return Err(D::Error::custom(format!("Array too short for {} expression", name)));
    }

    let parsed_name = arr.remove(0);
    if parsed_name.as_str() != Some(name) {
        return Err(D::Error::invalid_value(serde::de::Unexpected::Option, &"Basics"));
    }
    return Ok(arr);
}

pub struct PrefixHelper<H: DeserializeOwned, T: DeserializeOwned>(H, Vec<T>);

impl<'de, H: DeserializeOwned, T: DeserializeOwned> Deserialize<'de> for PrefixHelper<H, T> {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error> where
        D: Deserializer<'de> {
        use serde::de::Error;

        let mut arr: Vec<json::Value> = Deserialize::deserialize(deserializer)?;
        if arr.len() == 0 {
            return Err(D::Error::custom("Array too short"));
        }

        let head = json::from_value(arr.remove(0)).map_err(|e| D::Error::custom(format!("Could not parse head : {:?}", e)))?;
        let tail: StdResult<Vec<T>, _> = arr.into_iter().map(json::from_value).collect();
        let tail = tail.map_err(|e| D::Error::custom(format!("Could not parse tail : {:?}", e)))?;


        Ok(PrefixHelper(head, tail))
    }
}

macro_rules! expr {
    ($e:tt) => {::json::from_value::<BaseExpr>(json!($e))};
}

pub mod assert;
pub mod convert;
pub mod feature;
pub mod lookup;
pub mod decision;
pub mod interp;

pub mod variables;

pub mod logic;
pub mod math;
pub mod string;
pub mod color;
pub mod zoom;

pub type Array = Vec<ExprVal>;
pub type Object = BTreeMap<String, ExprVal>;

impl From<::common::geometry::Value> for ExprVal {
    fn from(v: ::common::geometry::Value) -> Self {
        match v {
            ::common::geometry::Value::Null => ExprVal::Null,
            ::common::geometry::Value::Bool(b) => ExprVal::Bool(b),
            ::common::geometry::Value::Int(a) => ExprVal::Num(a as _),
            ::common::geometry::Value::UInt(a) => ExprVal::Num(a as _),
            ::common::geometry::Value::Float(a) => ExprVal::Num(a as _),
            ::common::geometry::Value::String(s) => ExprVal::String(s),
            ::common::geometry::Value::List(a) => ExprVal::List(a.into_iter().map(|v| v.into()).collect()),
            ::common::geometry::Value::Object(o) => ExprVal::Object(o.into_iter().map(|(k, v)| (k, v.into())).collect()),
        }
    }
}


#[derive(Debug, Deserialize, Clone, PartialEq, PartialOrd)]
#[serde(untagged)]
pub enum ExprVal {
    Null,
    Bool(bool),
    Num(f64),
    String(String),
    Color(Color),
    List(Array),
    Object(Object),
}

impl ExprVal {
    fn as_bool(&self) -> Option<bool> {
        if let ExprVal::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    fn as_number(&self) -> Option<f64> {
        if let ExprVal::Num(n) = self {
            Some(*n)
        } else {
            None
        }
    }
    fn as_str(&self) -> Option<&str> {
        if let ExprVal::String(s) = self {
            Some(s)
        } else {
            None
        }
    }
    fn as_array(&self) -> Option<&Array> {
        if let ExprVal::List(a) = self {
            Some(a)
        } else {
            None
        }
    }
    fn as_object(&self) -> Option<&Object> {
        if let ExprVal::Object(o) = self {
            Some(o)
        } else {
            None
        }
    }
}

use std::convert::TryInto;

macro_rules! impl_converts {
    ($arm:tt,$($typ:ty) * ) => {
        $(impl From<$typ> for ExprVal {
            fn from(t: $typ) -> Self {
                ExprVal::$arm(t as _)
            }
        }

        impl TryInto<$typ> for ExprVal {
            type Error = Type;
            fn try_into(self) -> StdResult<$typ,Type> {
                match self {
                    ExprVal::$arm(v) => Ok(v as _),
                    _ => Err(self.get_type()),
                }
            }
        })*
    };
}

impl_converts!(Bool,bool);
impl_converts!(String,String);
impl_converts!(Num,f64 f32 i32 i64 u32 u64);
impl_converts!(Color,Color);

impl ExprVal {
    fn typ(&self) -> Type {
        return match self {
            ExprVal::Null => Type::Null,
            ExprVal::Bool(_) => Type::Boolean,
            ExprVal::Num(_) => Type::Number,
            ExprVal::String(_) => Type::String,
            ExprVal::Color(_) => Type::Color,
            ExprVal::List(_) => Type::Array,
            ExprVal::Object(_) => Type::Object,
        };
    }
    pub fn get_type(&self) -> Type {
        return self.typ();
    }
}


#[derive(Debug, Deserialize, Clone, Copy, Hash, PartialOrd, PartialEq, Ord, Eq)]
pub enum Type {
    #[serde(rename = "null")]
    Null,
    #[serde(rename = "color")]
    Color,
    #[serde(rename = "object")]
    Object,
    #[serde(rename = "array")]
    Array,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "boolean")]
    Boolean,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum BaseExpr {
    ArrayAssert(Box<assert::ArrayAssert>),
    Assert(Box<assert::Assert>),
    Color(Box<color::ColorExpr>),
    Convert(Box<convert::Convert>),
    TypeOf(Box<convert::TypeOf>),
    GeomType(feature::GeomType),
    Id(feature::Id),
    Properties(feature::Properties),
    Interpolate(Box<interp::Interpolate>),
    Step(Box<interp::Step>),
    Logic(Box<logic::Logic>),
    Lookup(Box<lookup::Lookup>),
    Decision(Box<decision::Decision>),
    Math(Box<math::Math>),
    String(Box<string::Str>),
    Variable(Box<variables::Variable>),
    Zoom(zoom::Zoom),
    Value(ExprVal),
}


pub struct EvaluationContext {
    pub zoom: Option<f32>,
    pub feature_data: Option<RefCell<::mapbox_tiles::Feature>>,
    pub bindings: RefCell<BTreeMap<String, BaseExpr>>,

}

impl EvaluationContext {
    fn get(&self, name: &str) -> Option<ExprVal> {
        None
    }
}

pub trait Expr {
    fn is_zoom(&self) -> bool;
    fn is_feature(&self) -> bool;

    fn eval(&self, ctx: &EvaluationContext) -> ExprResult;
}



#[derive(Debug)]
pub enum ExprEvalError {
    InvalidType {
        expected: Type,
        got: Type,
    },
    InvalidNumberOfArguments {
        expected: usize,
        got: usize,
    },
    Custom(String),
}

impl ExprEvalError {
    fn custom(m: impl Into<String>) -> ExprEvalError {
        ExprEvalError::Custom(m.into())
    }
}

pub type ExprResult = StdResult<ExprVal, ExprEvalError>;

pub fn expect_type(t: Type, v: ExprVal) -> ExprResult {
    return if v.typ() == t {
        Ok(v)
    } else {
        Err(ExprEvalError::InvalidType {
            expected: t,
            got: v.typ(),
        })
    };
}

pub fn expect_num(v: ExprVal) -> StdResult<f64, ExprEvalError> {
    return if let ExprVal::Num(n) = v {
        Ok(n)
    } else {
        Err(ExprEvalError::InvalidType {
            expected: Type::Number,
            got: v.typ(),
        })
    };
}

pub fn expect_bool(v: ExprVal) -> StdResult<bool, ExprEvalError> {
    return if let ExprVal::Bool(n) = v {
        Ok(n)
    } else {
        Err(ExprEvalError::InvalidType {
            expected: Type::Boolean,
            got: v.typ(),
        })
    };
}

pub fn expect_color(v: ExprVal) -> StdResult<Color, ExprEvalError> {
    return if let ExprVal::Color(n) = v {
        Ok(n)
    } else {
        Err(ExprEvalError::InvalidType {
            expected: Type::Color,
            got: v.typ(),
        })
    };
}

pub fn expect_len<T>(a: &[T], count: usize) -> StdResult<(), ExprEvalError> {
    if a.len() >= count {
        return Ok(());
    } else {
        return Err(ExprEvalError::InvalidNumberOfArguments {
            got: a.len(),
            expected: count,
        });
    }
}


impl Expr for ExprVal {
    fn is_zoom(&self) -> bool {
        false
    }

    fn is_feature(&self) -> bool {
        false
    }

    fn eval(&self, ctx: &EvaluationContext) -> ExprResult {
        return Ok(self.clone());
    }
}

impl Expr for BaseExpr {
    fn is_zoom(&self) -> bool {
        return match self {
            BaseExpr::ArrayAssert(inner) => inner.deref().is_zoom(),
            BaseExpr::Assert(inner) => inner.deref().is_zoom(),
            BaseExpr::Color(inner) => inner.deref().is_zoom(),
            BaseExpr::Convert(inner) => inner.deref().is_zoom(),
            BaseExpr::TypeOf(inner) => inner.deref().is_zoom(),
            BaseExpr::GeomType(inner) => inner.deref().is_zoom(),
            BaseExpr::Id(inner) => inner.deref().is_zoom(),
            BaseExpr::Properties(inner) => inner.deref().is_zoom(),
            BaseExpr::Interpolate(inner) => inner.deref().is_zoom(),
            BaseExpr::Step(inner) => inner.deref().is_zoom(),

            BaseExpr::Logic(inner) => inner.deref().is_zoom(),
            BaseExpr::Lookup(inner) => inner.deref().is_zoom(),
            BaseExpr::Decision(inner) => inner.deref().is_zoom(),
            BaseExpr::Math(inner) => inner.deref().is_zoom(),
            BaseExpr::String(inner) => inner.deref().is_zoom(),
            BaseExpr::Variable(inner) => inner.deref().is_zoom(),
            BaseExpr::Zoom(inner) => inner.deref().is_zoom(),
            BaseExpr::Value(inner) => inner.deref().is_zoom(),
        };
    }

    fn is_feature(&self) -> bool {
        return match self {
            BaseExpr::ArrayAssert(inner) => inner.deref().is_feature(),
            BaseExpr::Assert(inner) => inner.deref().is_feature(),
            BaseExpr::Color(inner) => inner.deref().is_feature(),
            BaseExpr::Convert(inner) => inner.deref().is_feature(),
            BaseExpr::TypeOf(inner) => inner.deref().is_feature(),
            BaseExpr::GeomType(inner) => inner.deref().is_feature(),
            BaseExpr::Id(inner) => inner.deref().is_feature(),
            BaseExpr::Properties(inner) => inner.deref().is_feature(),
            BaseExpr::Interpolate(inner) => inner.deref().is_feature(),
            BaseExpr::Step(inner) => inner.deref().is_feature(),
            BaseExpr::Logic(inner) => inner.deref().is_feature(),
            BaseExpr::Lookup(inner) => inner.deref().is_feature(),
            BaseExpr::Decision(inner) => inner.deref().is_feature(),
            BaseExpr::Math(inner) => inner.deref().is_feature(),
            BaseExpr::String(inner) => inner.deref().is_feature(),
            BaseExpr::Variable(inner) => inner.deref().is_feature(),
            BaseExpr::Zoom(inner) => inner.deref().is_feature(),
            BaseExpr::Value(inner) => inner.is_feature(),
        };
    }

    fn eval(&self, ctx: &EvaluationContext) -> ExprResult {
        return match self {
            BaseExpr::ArrayAssert(inner) => inner.deref().eval(ctx),
            BaseExpr::Assert(inner) => inner.deref().eval(ctx),
            BaseExpr::Color(inner) => inner.deref().eval(ctx),
            BaseExpr::Convert(inner) => inner.deref().eval(ctx),
            BaseExpr::TypeOf(inner) => inner.deref().eval(ctx),
            BaseExpr::GeomType(inner) => inner.deref().eval(ctx),
            BaseExpr::Id(inner) => inner.deref().eval(ctx),
            BaseExpr::Properties(inner) => inner.deref().eval(ctx),
            BaseExpr::Interpolate(inner) => inner.deref().eval(ctx),
            BaseExpr::Step(inner) => inner.deref().eval(ctx),
            BaseExpr::Logic(inner) => inner.deref().eval(ctx),
            BaseExpr::Lookup(inner) => inner.deref().eval(ctx),
            BaseExpr::Decision(inner) => inner.deref().eval(ctx),
            BaseExpr::Math(inner) => inner.deref().eval(ctx),
            BaseExpr::String(inner) => inner.deref().eval(ctx),
            BaseExpr::Variable(inner) => inner.deref().eval(ctx),
            BaseExpr::Zoom(inner) => inner.deref().eval(ctx),
            BaseExpr::Value(inner) => inner.deref().eval(ctx),
        };
    }
}

#[test]
fn test_expr() {
    use super::*;

    fn check_expr(e: BaseExpr) {
        println!("zoom: {:?}, feature: {:?}, expr: {:#?}", e.is_zoom(), e.is_feature(), e);
    }

    let mut expr = expr!(["sqrt", ["/", ["get", "population"], ["+", ["zoom"], 200 ]]]);
    check_expr(expr.unwrap());
    let case = expr!([
        "case",
            // features that have both:
            [ "all", ["has", "name_zh"], ["has", "name_en"] ],
            ["concat", ["get", "name_zh"], "\n", ["get", "name_en"]],
            // features that have only name_zh:
            [ "has", "name_zh" ],
            ["get", "name_zh"],
            // features that have only name_en:
            [ "has", "name_en" ],
            [ "get", "name_en" ],
            // features that have neither:
            ""
    ]);


    check_expr(case.unwrap());

    let l_expr = expr!([
        "let",
        "rgba", ["to-rgba", ["to-color", ["get", "color_property"]]],
        ["let",
            "r", ["number", ["*", 255, ["at", 0, ["var", "rgba"]]]],
            "g", ["number", ["*", 255, ["at", 1, ["var", "rgba"]]]],
            "b", ["number", ["*", 255, ["at", 2, ["var", "rgba"]]]],
            "a", ["number", ["at", 3, ["var", "rgba"]]],
            ["let",
                "avg", ["+", ["*", 0.299, ["var", "r"]], ["*", 0.587, ["var", "g"]], ["*", 0.114, ["var", "b"]]],
                ["let",
                    "desat_r", ["+", ["*", 0.4, ["var", "avg"]], ["*", 0.4, 128], ["*", 0.2, ["var", "r"]]],
                    "desat_g", ["+", ["*", 0.4, ["var", "avg"]], ["*", 0.4, 128], ["*", 0.2, ["var", "g"]]],
                    "desat_b", ["+", ["*", 0.4, ["var", "avg"]], ["*", 0.4, 128], ["*", 0.2, ["var", "b"]]],
                    ["rgba", ["var", "desat_r"],  ["var", "desat_g"], ["var", "desat_b"], ["var", "a"]]
                ]
            ]
        ]
    ]);
    check_expr(l_expr.unwrap());
}