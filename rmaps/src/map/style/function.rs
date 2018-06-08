use prelude::*;

use common::serde::{self, Deserialize, Serialize, de::DeserializeOwned, Deserializer};
use common::json;

#[derive(Debug, Clone)]
pub enum FunctionStop<T: DeserializeOwned + Clone> {
    Value(f32, T),
    ValueAndZoom {
        value: f32,
        zoom: f32,
        res: T,
    },
}

fn from_jvalue<T: ::common::serde::de::DeserializeOwned>(val: &json::Value) -> StdResult<T, json::Error> {
    return json::from_value(val.clone());
}


impl<'de, T: DeserializeOwned + Clone> Deserialize<'de> for FunctionStop<T> {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de> {
        let serde_err = |_e| {
            serde::de::Error::custom("Invalid Function stop")
        };

        let data: Vec<json::Value> = Deserialize::deserialize(deserializer)?;
        match data[..] {
            [json::Value::Object(ref obj), ref x] => {
                let zoom = obj.get("zoom").unwrap();
                let value = obj.get("value").unwrap();

                return Ok(
                    FunctionStop::ValueAndZoom {
                        value: from_jvalue(value).map_err(serde_err)?,
                        zoom: from_jvalue(zoom).map_err(serde_err)?,
                        res: from_jvalue(&x).map_err(serde_err)?,
                    }
                );
            }
            [json::Value::Number(ref n), ref x] => {
                return Ok(FunctionStop::Value(n.as_f64().unwrap() as _, from_jvalue(&x).map_err(serde_err)?));
            }
            _ => {
                return Err(serde::de::Error::custom("Invalid Function stop"));
            }
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum FunctionType {
    Identity,
    Exponential,
    Interval,
    Categorical,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum Function<T: DeserializeOwned + Clone> {
    #[serde(bound(deserialize = "T : DeserializeOwned"))]
    Raw(T),
    Interpolated {
        property: Option<String>,
        base: Option<f32>,
        #[serde(rename = "type")]
        typ: Option<String>,
        #[serde(bound(deserialize = "T : DeserializeOwned"))]
        default: Option<T>,
        #[serde(rename = "colorSpace")]
        color_space: Option<String>,
        #[serde(bound(deserialize = "T : DeserializeOwned"))]
        stops: Vec<FunctionStop<T>>,
    },
}

impl<T: DeserializeOwned + Clone> Function<T> {
    pub fn eval(&self) -> T {
        match self {
            Function::Raw(c) => c.clone(),
            _ => panic!()
        }
    }
}