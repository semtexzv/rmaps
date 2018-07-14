use super::prelude::*;


#[derive(Debug, Clone)]
pub enum FeatureExpr {
    GeomType,
    Id,
    Properties,
}

impl Expression for FeatureExpr {
    fn is_zoom(&self) -> bool {
        false
    }
    fn is_feature(&self) -> bool {
        true
    }
    fn eval(&self, ctx: &EvaluationContext) -> ExprResult {
        unimplemented!()
        // delegate_to_inner! {self;  [FeatureExpr::GeomType, FeatureExpr::Id, FeatureExpr::Properties]; (v) => v.eval(ctx)}
    }
}


impl<'de> Deserialize<'de> for FeatureExpr {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error> where D: Deserializer<'de> {
        NAME.with(|n| {
            match n.deref() {
                "geometry-type" => Ok(FeatureExpr::GeomType),
                "id" => Ok(FeatureExpr::Id),
                "properties" => Ok(FeatureExpr::Properties),
                _ => Err(D::Error::custom("Not a valid feature expression"))
            }
        })
    }
}
