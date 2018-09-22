use prelude::*;


#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum PropKey {
    Type,
    Id,
    Key(String),
}

impl<'de> Deserialize<'de> for PropKey {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
        where
            D: Deserializer<'de> {
        struct Vis;

        impl<'de> de::Visitor<'de> for Vis {
            type Value = PropKey;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Property key")
            }

            fn visit_str<E: ::std::error::Error>(self, v: &str) -> StdResult<Self::Value, E> {
                return Ok(match v {
                    "$type" => PropKey::Type,
                    "$id" => PropKey::Id,
                    a @ _ => PropKey::Key(a.to_string())
                });
            }
        }
        Ok(deserializer.deserialize_str(Vis)?)
    }
}


#[derive(Debug, Clone)]
pub enum Filter {
    Raw(bool),
    Has(PropKey),
    NotHas(PropKey),
    In(PropKey, Vec<Value>),
    NotIn(PropKey, Vec<Value>),
    Eq(PropKey, Value),
    Neq(PropKey, Value),
    Gt(PropKey, Value),
    Geq(PropKey, Value),
    Lt(PropKey, Value),
    Leq(PropKey, Value),
    All(Vec<Filter>),
    Any(Vec<Filter>),
    None(Vec<Filter>),
}

use ::common::serde::{
    self,
    Serialize, Deserialize, Serializer, Deserializer};
use ::common::json;


fn from_jvalue<T: ::common::serde::de::DeserializeOwned>(val: &json::Value) -> StdResult<T, json::Error> {
    return json::from_value(val.clone());
}


impl<'de> Deserialize<'de> for Filter {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de> {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Help {
            Bool(bool),
            Arr(Vec<json::Value>),
        };
        let data: Help = Deserialize::deserialize(deserializer)?;
        if let Help::Bool(b) = data {
            return Ok(Filter::Raw(b));
        }

        let mut data = if let Help::Arr(d) = data {
            d
        } else {
            panic!()
        };


        let serde_err = |_e| {
            serde::de::Error::custom("Invalid filter")
        };

        match data[..] {
            [json::Value::String(ref first), ref mut rest..] => {
                return Ok(match (first.as_ref(), rest) {
                    ("has", [key]) => {
                        Filter::Has(from_jvalue(key).map_err(serde_err)?)
                    }
                    ("!has", [key]) => {
                        Filter::NotHas(from_jvalue(key).map_err(serde_err)?)
                    }
                    ("==", [key, value]) => {
                        Filter::Eq(
                            from_jvalue(key).map_err(serde_err)?,
                            from_jvalue(value).map_err(serde_err)?,
                        )
                    }
                    ("!=", [key, value]) => {
                        Filter::Neq(
                            from_jvalue(key).map_err(serde_err)?,
                            from_jvalue(value).map_err(serde_err)?,
                        )
                    }
                    (">", [key, value]) => {
                        Filter::Gt(
                            from_jvalue(key).map_err(serde_err)?,
                            from_jvalue(value).map_err(serde_err)?,
                        )
                    }
                    (">=", [key, value]) => {
                        Filter::Geq(
                            from_jvalue(key).map_err(serde_err)?,
                            from_jvalue(value).map_err(serde_err)?,
                        )
                    }
                    ("<", [key, value]) => {
                        Filter::Lt(
                            from_jvalue(key).map_err(serde_err)?,
                            from_jvalue(value).map_err(serde_err)?,
                        )
                    }
                    ("<=", [key, value]) => {
                        Filter::Leq(
                            from_jvalue(key).map_err(serde_err)?,
                            from_jvalue(value).map_err(serde_err)?,
                        )
                    }
                    ("in", [key, rest..]) => {
                        let vals = rest.iter()
                            .map(|v| from_jvalue(v).map_err(serde_err))
                            .collect::<StdResult<Vec<_>, _>>()?;

                        Filter::In(from_jvalue(key).map_err(serde_err)?, vals)
                    }
                    ("!in", [key, rest..]) => {
                        let vals = rest.iter()
                            .map(|v| from_jvalue(v).map_err(serde_err))
                            .collect::<StdResult<Vec<_>, _>>()?;

                        Filter::NotIn(from_jvalue(key).map_err(serde_err)?, vals)
                    }
                    ("all", rest) => {
                        let filters = rest.iter()
                            .map(|v| from_jvalue(v).map_err(serde_err))
                            .collect::<StdResult<Vec<Filter>, _>>()?;
                        Filter::All(filters)
                    }
                    ("any", rest) => {
                        let filters = rest.iter()
                            .map(|v| from_jvalue(v).map_err(serde_err))
                            .collect::<StdResult<Vec<Filter>, _>>()?;
                        Filter::Any(filters)
                    }
                    ("none", rest) => {
                        let filters = rest.iter()
                            .map(|v| from_jvalue(v).map_err(serde_err))
                            .collect::<StdResult<Vec<Filter>, _>>()?;
                        Filter::None(filters)
                    }
                    _ => {
                        return Err(serde::de::Error::custom("Invalid filter"));
                    }
                });
            }
            _ => {}
        }

        unimplemented!()
    }
}


pub struct FilterEvaluator<'a> {
    feature: &'a ::mvt::Feature,
}

use geometry::Value;

impl<'a> FilterEvaluator<'a> {
    pub fn new(feature: &'a ::mvt::Feature) -> Self {
        FilterEvaluator {
            feature,
        }
    }
    pub fn satisfies_opt(feature: &::mvt::Feature, filter: &Option<Filter>) -> bool {
        return if let Some(filter) = filter {
            let res = FilterEvaluator::new(feature).evaluate(filter);
            res
        } else { true };
    }
    pub fn satisfies(feature: &::mvt::Feature, filter: &Filter) -> bool {
        FilterEvaluator::new(feature).evaluate(filter)
    }

    fn id(&self) -> u64 {
        return self.feature.id;
    }

    fn typ(&self) -> String {
        self.feature.typ.to_string()
    }
    fn get(&self, key: &PropKey) -> Option<Value> {
        match key {
            PropKey::Id => Some(Value::UInt(self.feature.id)),
            PropKey::Type => Some(Value::String(self.typ())),
            PropKey::Key(k) => self.feature.get(k).map(|v| v.clone()),
        }
    }
    fn evaluate(&self, filter: &Filter) -> bool {
        return match filter {
            Filter::Raw(v) => *v,
            Filter::Has(PropKey::Id) => true,
            Filter::Has(PropKey::Type) => true,
            Filter::Has(PropKey::Key(ref key)) => self.feature.has(key),
            Filter::NotHas(k) => !self.evaluate(&Filter::Has(k.clone())),
            Filter::In(k, vals) => vals.iter().any(|v| Some(v) == self.get(k).as_ref()),
            Filter::NotIn(k, vals) => !vals.iter().any(|v| Some(v) == self.get(k).as_ref()),

            Filter::Eq(k, v) => Some(v) == self.get(k).as_ref(),
            Filter::Neq(k, v) => Some(v) != self.get(k).as_ref(),

            Filter::Gt(k, v) => Some(v) > self.get(k).as_ref(),
            Filter::Geq(k, v) => Some(v) >= self.get(k).as_ref(),
            Filter::Lt(k, v) => Some(v) < self.get(k).as_ref(),
            Filter::Leq(k, v) => Some(v) <= self.get(k).as_ref(),
            Filter::All(filters) => filters.iter().all(|f| self.evaluate(f)),
            Filter::Any(filters) => filters.iter().any(|f| self.evaluate(f)),
            Filter::None(filters) => !filters.iter().any(|f| self.evaluate(f)),
            _ => false,
        };
    }
}