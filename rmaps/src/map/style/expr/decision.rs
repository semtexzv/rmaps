use ::prelude::*;
use super::*;

use serde::Deserialize;
use serde::de::Error;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum Decision {
    Case(Case),
    Match(Match),
    Coalesce(Coalesce),
}

impl Expr for Decision {
    fn is_zoom(&self) -> bool {
        match self {
            Decision::Case(c) => c.is_zoom(),
            Decision::Match(m) => m.is_zoom(),
            Decision::Coalesce(c) => c.is_zoom(),
        }
    }

    fn is_feature(&self) -> bool {
        match self {
            Decision::Case(c) => c.is_feature(),
            Decision::Match(m) => m.is_feature(),
            Decision::Coalesce(c) => c.is_feature(),
        }
    }

    fn eval(&self, ctx: &EvaluationContext) -> ExprResult {
        match self {
            Decision::Case(c) => c.eval(ctx),
            Decision::Match(m) => m.eval(ctx),
            Decision::Coalesce(c) => c.eval(ctx),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Case(Vec<(BaseExpr, BaseExpr)>, BaseExpr);

impl<'de> Deserialize<'de> for Case {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de> {
        #[derive(Deserialize)]
        enum _Marker {
            #[serde(rename = "case")]
            _Mark,
        }
        let PrefixHelper(_Marker::_Mark, mut exprs) = Deserialize::deserialize(deserializer)?;

        println!("Exprs: {:#?}", exprs);
        let mut iter = exprs.into_iter();

        let mut arms = vec![];
        'l: loop {
            match (iter.next(), iter.next()) {
                (Some(k), Some(v)) => {
                    arms.push((k, v));
                }
                (Some(default), None) => {
                    break 'l Ok(Case(arms, default));
                }
                a @ _ => {
                    break 'l Err(D::Error::custom(format!("Invalid match expression {:?}", a)));
                }
            }
        }
    }
}


impl Expr for Case {
    fn is_zoom(&self) -> bool {
        return self.0.iter().any(|(a, b)| a.is_zoom() || b.is_zoom()) || self.1.is_zoom();
    }

    fn is_feature(&self) -> bool {
        return self.0.iter().any(|(a, b)| a.is_feature() || b.is_feature()) || self.1.is_feature();
    }

    fn eval(&self, ctx: &EvaluationContext) -> ExprResult {
        for (cond, val) in self.0.iter() {
            let test = expect_type(Type::Boolean, cond.eval(ctx)?)?;
            if let ExprVal::Bool(test) = test {
                return val.eval(ctx);
            }
        }
        self.1.eval(ctx)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Coalesce(Vec<BaseExpr>);

impl<'de> Deserialize<'de> for Coalesce {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de> {
        let d: Vec<json::Value> = Deserialize::deserialize(deserializer)?;
        if d.len() < 2 {
            return Err(D::Error::custom("Array too short"));
        }

        if d[0] == json!("coalesce") {
            let exprs: StdResult<Vec<BaseExpr>, _> = d.into_iter().skip(1).map(|v| json::from_value(v)).collect();
            return Ok(Coalesce(exprs.map_err(|_| D::Error::custom("Could not parse type"))?));
        }
        return Err(D::Error::custom("Could not parse type"));
    }
}

impl Expr for Coalesce {
    fn is_zoom(&self) -> bool {
        self.0.iter().any(|v| v.is_zoom())
    }

    fn is_feature(&self) -> bool {
        self.0.iter().any(|v| v.is_feature())
    }

    fn eval(&self, ctx: &EvaluationContext) -> ExprResult {
        for e in self.0.iter() {
            let v = e.eval(ctx)?;
            if v.typ() != Type::Null {
                return Ok(v);
            }
        }
        return Ok(ExprVal::Null);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    label: ExprVal,
    expr: BaseExpr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Match {
    input: BaseExpr,
    arms: Vec<MatchArm>,
    default: BaseExpr,
}

impl<'de> Deserialize<'de> for Match {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, <D as Deserializer<'de>>::Error> where D: Deserializer<'de> {
        #[derive(Deserialize)]
        enum _Marker {
            #[serde(rename = "match")]
            _Mark
        }
        let PrefixHelper(_Marker::_Mark, mut exprs) = Deserialize::deserialize(deserializer)?;

        let input = exprs.remove(0);
        let mut iter = exprs.into_iter();

        let mut arms = vec![];
        'l: loop {
            match (iter.next(), iter.next()) {
                (Some(BaseExpr::Value(k)), Some(v)) => {
                    arms.push(MatchArm {
                        label: k,
                        expr: v,
                    });
                }
                (Some(default), None) => {
                    break 'l Ok(Match {
                        input,
                        arms,
                        default,
                    });
                }
                a @ _ => {
                    break 'l Err(D::Error::custom(format!("Invalid match expression {:?}", a)));
                }
            }
        }
    }
}


impl Expr for Match {
    fn is_zoom(&self) -> bool {
        return self.input.is_zoom()
            || self.arms.iter().any(|a| a.expr.is_zoom())
            || self.default.is_zoom();
    }

    fn is_feature(&self) -> bool {
        return self.input.is_feature()
            || self.arms.iter().any(|a| a.expr.is_feature())
            || self.default.is_feature();
    }

    fn eval(&self, ctx: &EvaluationContext) -> ExprResult {
        let val = self.input.eval(ctx)?;

        for MatchArm { label, expr } in self.arms.iter() {
            if &val == label {
                return expr.eval(ctx);
            }
        }
        return self.default.eval(ctx);
    }
}



