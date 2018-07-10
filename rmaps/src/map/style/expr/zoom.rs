use prelude::*;
use super::*;

use serde::Deserialize;

#[derive(Debug,Clone,PartialEq)]
pub struct Zoom {}

impl<'de> Deserialize<'de> for Zoom {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error> where
        D: Deserializer<'de> {
        #[derive(Deserialize)]
        enum _Marker {
            #[serde(rename = "zoom")]
            _Mark
        }

        let PrefixHelper(mark, _): PrefixHelper<_Marker, BaseExpr> = Deserialize::deserialize(deserializer)?;
        return Ok(Zoom {});
    }
}

impl Expr for Zoom {
    fn is_zoom(&self) -> bool {
        true
    }

    fn is_feature(&self) -> bool {
        false
    }

    fn eval(&self, ctx: &EvaluationContext) -> ExprResult {
        Ok(ExprVal::Num(ctx.zoom.unwrap() as _))
    }
}