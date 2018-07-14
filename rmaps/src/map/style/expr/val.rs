use ::prelude::*;
use super::*;

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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Null,
    Bool(bool),
    Num(f64),
    String(String),
    Color(Color),
    List(Array),
    Object(Object),
}

pub struct ValueVisitor;

impl<'de> serde::de::Visitor<'de> for ValueVisitor {
    type Value = Value;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Value")
    }

    fn visit_bool<E>(self, v: bool) -> StdResult<Self::Value, E> where
        E: de::Error, {
        Ok(Value::Bool(v))
    }

    fn visit_i64<E>(self, v: i64) -> StdResult<Self::Value, E> where
        E: de::Error, {
        Ok(Value::Num(v as _))
    }

    fn visit_u64<E>(self, v: u64) -> StdResult<Self::Value, E> where
        E: de::Error, {
        Ok(Value::Num(v as _))
    }

    fn visit_f64<E>(self, v: f64) -> StdResult<Self::Value, E> where
        E: de::Error, {
        Ok(Value::Num(v as _))
    }

    fn visit_str<E>(self, v: &str) -> StdResult<Self::Value, E> where E: de::Error, {
        if TYPE.is_set() {
            TYPE.with(|t| {
                match t {
                    Type::Color => {
                        let c = Color::from_str(v).map_err(|_| de::Error::invalid_type(de::Unexpected::Str(v), &"Color"));

                        Ok(Value::Color(c?))
                    }
                    _ => {
                        Ok(Value::String(v.into()))
                    }
                }
            })
        } else {
            Ok(Value::String(v.into()))
        }
    }

    fn visit_bytes<E>(self, v: &[u8]) -> StdResult<Self::Value, E> where
        E: de::Error, {
        unimplemented!()
    }

    fn visit_none<E>(self) -> StdResult<Self::Value, E> where
        E: de::Error, {
        Ok(Value::Null)
    }

    fn visit_unit<E>(self) -> StdResult<Self::Value, E> where
        E: de::Error, {
        Ok(Value::Null)
    }

    fn visit_seq<A>(self, seq: A) -> StdResult<Self::Value, A::Error> where
        A: de::SeqAccess<'de>, {
        Ok(Value::List(Vec::<Value>::deserialize(de::value::SeqAccessDeserializer::new(seq))?))
    }

    fn visit_map<A>(self, map: A) -> StdResult<Self::Value, A::Error> where
        A: de::MapAccess<'de>, {
        Ok(Value::Object(BTreeMap::<String, Value>::deserialize(de::value::MapAccessDeserializer::new(map))?))
    }
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error> where
        D: Deserializer<'de> {
        deserializer.deserialize_any(ValueVisitor)
    }
}


impl Value {
    pub fn as_bool(&self) -> Option<bool> {
        if let Value::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    pub fn as_number(&self) -> Option<f64> {
        if let Value::Num(n) = self {
            Some(*n)
        } else {
            None
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        if let Value::String(s) = self {
            Some(s)
        } else {
            None
        }
    }
    pub fn as_array(&self) -> Option<&Array> {
        if let Value::List(a) = self {
            Some(a)
        } else {
            None
        }
    }
    pub fn as_object(&self) -> Option<&Object> {
        if let Value::Object(o) = self {
            Some(o)
        } else {
            None
        }
    }
}

use std::convert::{TryInto, TryFrom};


macro_rules! impl_converts {
    ($arm:tt,$($typ:ty) * ) => {
        $(impl From<$typ> for Value {
            fn from(t: $typ) -> Self {
                Value::$arm(t as _)
            }
        }

        impl TryFrom<Value> for $typ {
            type Error = Type;

            fn try_from(value: Value) -> StdResult<Self, Type> {
                 match value {
                    Value::$arm(v) => Ok(v as _),
                    a @ _ => Err(a.get_type()),
                }
            }
        }

        impl From<Option<$typ>> for Value {
            fn from(t: Option<$typ>) -> Self {
                match t {
                    Some(t) => Value::$arm(t as _),
                    None => Value::Null,
                }
            }
        }

        impl TryFrom<Value> for Option<$typ> {
            type Error = Type;

            fn try_from(value: Value) -> StdResult<Self, Type> {
                 match value {
                    Value::Null => Ok(None),
                    Value::$arm(v) => Ok(Some(v as _)),
                    a @ _ => Err(a.get_type()),
                }
            }
        }


        )*
    };
}

impl_converts!(Bool,bool);
impl_converts!(String,String);
impl_converts!(Num,f64 f32 i32 i64 u32 u64);
impl_converts!(Color,Color);

impl Value {
    fn typ(&self) -> Type {
        return match self {
            Value::Null => Type::Null,
            Value::Bool(_) => Type::Boolean,
            Value::Num(_) => Type::Number,
            Value::String(_) => Type::String,
            Value::Color(_) => Type::Color,
            Value::List(_) => Type::Array,
            Value::Object(_) => Type::Object,
        };
    }
    pub fn get_type(&self) -> Type {
        return self.typ();
    }
}


impl Parse for Value {
    fn parse(value: json::Value, expected: Type) -> ParseResult<Self> {
        match expected {
            Type::Color => Ok(Value::Color(json::from_value(value.clone())?)),
            Type::String => Ok(Value::String(json::from_value(value.clone())?)),
            _ => {
                Ok(json::from_value(value.clone())?)
            }
        }
    }
}

impl Expression for Value {
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

