use ::prelude::*;
use super::*;

use serde::Deserialize;

#[derive(Debug, Clone, PartialEq)]
pub struct Convert(Type, Vec<BaseExpr>);


impl<'de> Deserialize<'de> for Convert {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de> {
        use serde::de::Error;

        let PrefixHelper(typ, tail): PrefixHelper<String, _> = Deserialize::deserialize(deserializer)?;

        let typ = match typ.deref() {
            "to-boolean" => Type::Boolean,
            "to-color" => Type::Color,
            "to-number" => Type::Number,
            "to-string" => Type::String,
            _ => {
                return Err(D::Error::custom("unknown destination type"));
            }
        };
        return Ok(Convert(typ, tail));
    }
}

impl Expr for Convert {
    fn is_zoom(&self) -> bool {
        self.1.iter().any(|e| e.is_zoom())
    }

    fn is_feature(&self) -> bool {
        self.1.iter().any(|e| e.is_feature())
    }

    fn eval(&self, ctx: &EvaluationContext) -> ExprResult {
        unimplemented!()
    }
}


#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum _TypeOfMarker {
    #[serde(rename = "typeof")]
    _Mark
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TypeOf(_TypeOfMarker, BaseExpr);

impl Expr for TypeOf {
    fn is_zoom(&self) -> bool {
        self.1.is_zoom()
    }

    fn is_feature(&self) -> bool {
        self.1.is_feature()
    }

    fn eval(&self, ctx: &EvaluationContext) -> ExprResult {
        let v = self.1.eval(ctx)?;
        return Ok(format!("{:?}", v.typ()).into());
    }
}

