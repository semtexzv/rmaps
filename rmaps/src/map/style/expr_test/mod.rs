pub use prelude::*;
pub use serde::{
    Deserializer,
    Deserialize,
    Serializer,
    Serialize,
    de::{
        DeserializeOwned,
        Error as DeError,
    },
};


pub type Array = Vec<Value>;
pub type Object = BTreeMap<String, Value>;

impl From<::common::geometry::Value> for Value {
    fn from(v: ::common::geometry::Value) -> Self {
        match v {
            ::common::geometry::Value::Null => Value::Null,
            ::common::geometry::Value::Bool(b) => Value::Bool(b),
            ::common::geometry::Value::Int(a) => Value::Num(a as _),
            ::common::geometry::Value::UInt(a) => Value::Num(a as _),
            ::common::geometry::Value::Float(a) => Value::Num(a as _),
            ::common::geometry::Value::String(s) => Value::String(s),
            ::common::geometry::Value::List(a) => Value::List(a.into_iter().map(|v| v.into()).collect()),
            ::common::geometry::Value::Object(o) => Value::Object(o.into_iter().map(|(k, v)| (k, v.into())).collect()),
        }
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq, PartialOrd)]
#[serde(untagged)]
pub enum Value {
    Null,
    Bool(bool),
    Num(f64),
    String(String),
    Color(Color),
    List(Array),
    Object(Object),
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
    #[serde(rename = "_")]
    Any,
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


pub trait ParseExpr: Sized {
    fn parse(value: json::Value, expected_type: Type) -> Result<Self>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct BaseExpr {}

impl ParseExpr for BaseExpr {
    fn parse(value: json::Value, expected_type: Type) -> Result<Self> {
        unimplemented!()
    }
}

fn from_val<T: ParseExpr>(val: json::Value) -> Result<T> {
    from_val_expect(val, Type::Any)
}

fn from_val_expect<T: ParseExpr>(val: json::Value, expected: Type) -> Result<T> {
    T::parse(val, expected)
}

macro_rules! parse_expr {
    (@match )
    ($type:ty;$($name:expr, $($arg:ident:$arg_typ:ty),* => $eval:block)*) => {
        impl ParseExpr for $type {
            fn parse(value : json::Value, expected: Type) -> Result<Self> {

            }
                let PrefixHelper(name, values) : PrefixHelper<String,json::Value> = json::from_value(value)?;
                match (name.deref(),&values[..]) {
                    $(($name,[$($arg),*]) => {
                        if let ($(Ok($arg)),*)  = ($(json::from_value::<$arg_typ>($arg.clone())),*) {
                            return Ok($eval)
                        }

                    })*,
                    _ => {}
                }
                bail!("Not this expression");

            }
        }
    };
}


pub mod assert;