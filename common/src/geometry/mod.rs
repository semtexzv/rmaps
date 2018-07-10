use prelude::*;

use std::collections::BTreeMap;


pub type Array = Vec<Value>;
pub type Object = BTreeMap<String, Value>;
use std::borrow::Cow;

#[derive(Debug, Deserialize, Clone, PartialOrd, PartialEq)]
#[serde(untagged)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    UInt(u64),
    Float(f64),
    String(String),
    List(Array),
    Object(Object),
}


impl ::std::cmp::PartialEq<u64> for Value {
    fn eq(&self, other: &u64) -> bool {
        if let Value::Int(v) = self {
            return *v as u64 == *other;
        }
        return false;
    }
}

impl<'a> ::std::cmp::PartialEq<&'a str> for Value {
    fn eq(&self, other: &&'a str) -> bool {
        if let Value::String(s) = self {
            return s == other;
        }
        return false;
    }
}

pub type Point<T> = [T; 2];
pub type LineString<T> = Vec<Point<T>>;
pub type Polygon<T> = Vec<Vec<Point<T>>>;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Geometry<T> {
    Point(Point<T>),
    MultiPoint(Vec<Point<T>>),
    LineString(LineString<T>),
    MultiLineString(Vec<LineString<T>>),
    Polygon(Polygon<T>),
    MultiPolygon(Vec<Polygon<T>>),
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Feature<T: Debug> {
    pub id: Option<Value>,
    pub properties: BTreeMap<String, Value>,
    pub geometry: Geometry<T>,
}
