use prelude::*;

use common::serde::{self, Deserialize, Serialize, de::DeserializeOwned, Deserializer};
use common::json;

#[derive(Debug, Clone)]
pub enum FunctionStop<T: DeserializeOwned + Clone> {
    Value(f32, T),
    ValueAndZoom {
        value: json::Value,
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
                        value: from_bjvalue(value).map_err(serde_err)?,
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


pub struct FunctionEvaluationContext<'a> {
    feature: Option<&'a ::mapbox_tiles::Feature>,
    zoom: f64,
}

const FACTOR: f64 = 100000.0;




impl<'a> FunctionEvaluationContext<'a> {
    pub fn new_zoom(zoom: f64) -> Self {
        return FunctionEvaluationContext {
            feature: None,
            zoom: zoom,
        };
    }
    pub fn evaluate<T: DeserializeOwned + Clone>(&self, fun: &Function<T>) -> T {
        struct FunctionZoomHelp<T> {
            stops: BTreeMap<f64, T>,
        }

        struct FunctionPropHelp<T> {
            stops: BTreeMap<json::Value, T>,
        }

        struct FunctionCombHelp<T> {
            stops: BTreeMap<(f64, json::Value), T>,
        }

        match fun {
            Function::Raw(v) => v.clone(),
            Function::Interpolated { property: None, base, typ, default, stops, .. } => {
                let mut help = BTreeMap::<i64, T>::new();
                for s in stops.into_iter() {
                    if let FunctionStop::Value(k, v) = s {
                        help.insert((*k as f64 * FACTOR) as i64, v.clone());
                    }
                }
                let factored_zoom = (self.zoom * FACTOR) as i64;
                let upper_bound = help.range(&factored_zoom..);

                if upper_bound.count() == 0 {
                    return default.clone().unwrap().clone();
                }


                panic!()
            }
            Function::Interpolated { property: Some(prop), base, typ, default, stops, .. } => {
                panic!()
            }
        }
    }
}