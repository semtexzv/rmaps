use ::prelude::*;

use super::*;

use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum _GeomTypeMarker {
    #[serde(rename = "geometry-type")]
    _Mark
}

#[derive(Debug, Clone, PartialEq)]
pub struct GeomType(_GeomTypeMarker);

impl<'de> Deserialize<'de> for GeomType {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de> {
        use serde::de::Error;

        let d: Vec<_GeomTypeMarker> = Deserialize::deserialize(deserializer)?;
        return Ok(GeomType(_GeomTypeMarker::_Mark));
    }
}


impl Expr for GeomType {
    fn is_zoom(&self) -> bool {
        false
    }

    fn is_feature(&self) -> bool {
        true
    }

    fn eval(&self, ctx: &EvaluationContext) -> ExprResult {
        unimplemented!()
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum _IdMarker {
    #[serde(rename = "id")]
    _Mark
}

#[derive(Debug, Clone, PartialEq)]
pub struct Id(_IdMarker);


impl<'de> Deserialize<'de> for Id {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de> {
        use serde::de::Error;

        let d: Vec<_IdMarker> = Deserialize::deserialize(deserializer)?;
        return Ok(Id(_IdMarker::_Mark));
    }
}

impl Expr for Id {
    fn is_zoom(&self) -> bool {
        false
    }

    fn is_feature(&self) -> bool {
        true
    }

    fn eval(&self, ctx: &EvaluationContext) -> ExprResult {
        unimplemented!()
    }
}


#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum _PropertiesMarker {
    #[serde(rename = "properties")]
    _Mark
}

#[derive(Debug, Clone, PartialEq)]
pub struct Properties(_PropertiesMarker);


impl<'de> Deserialize<'de> for Properties {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de> {
        use serde::de::Error;

        let d: Vec<_PropertiesMarker> = Deserialize::deserialize(deserializer)?;
        return Ok(Properties(_PropertiesMarker::_Mark));
    }
}

impl Expr for Properties {
    fn is_zoom(&self) -> bool {
        false
    }

    fn is_feature(&self) -> bool {
        true
    }

    fn eval(&self, ctx: &EvaluationContext) -> ExprResult {
        unimplemented!()
    }
}
