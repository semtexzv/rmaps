use prelude::*;


#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum Value {
    String(String),
    Num(f32),
    Bool(bool),
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum PropKey {
    #[serde(rename = "$type")]
    Type,
    #[serde(rename = "$id")]
    Id,
    Key(String),
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
        }  else {
            panic!()
        };



        let serde_err = |e| {
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