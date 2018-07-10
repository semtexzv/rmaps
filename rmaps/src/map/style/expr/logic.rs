use prelude::*;
use super::*;

use serde::Deserialize;
use serde::de::Error;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum LogicOp {
    #[serde(rename = "!")]
    Not,
    #[serde(rename = "==")]
    Eq,
    #[serde(rename = "!=")]
    Neq,
    #[serde(rename = ">")]
    Gt,
    #[serde(rename = ">=")]
    Geq,
    #[serde(rename = "<")]
    Lt,
    #[serde(rename = "<=")]
    Leq,
    #[serde(rename = "all")]
    All,
    #[serde(rename = "any")]
    Any,
    #[serde(rename = "none")]
    None,
}

impl LogicOp {
    fn min_args(&self) -> usize {
        0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Logic(LogicOp, Vec<BaseExpr>);

impl<'de> Deserialize<'de> for Logic {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error> where
        D: Deserializer<'de> {
        use serde::de::Error;
        let PrefixHelper(op, exprs) = Deserialize::deserialize(deserializer)?;
        return Ok(Logic(op, exprs));
    }
}

impl Expr for Logic {
    fn is_zoom(&self) -> bool {
        return self.1.iter().any(|e| e.is_zoom());
    }

    fn is_feature(&self) -> bool {
        return self.1.iter().any(|e| e.is_feature());
    }

    fn eval(&self, ctx: &EvaluationContext) -> ExprResult {
        return Ok(match (&self.0, &self.1[..]) {
            (LogicOp::Not, [a]) => {
                (!expect_bool(a.eval(ctx)?)?).into()
            }
            (LogicOp::Neq, [a, b]) => {
                (a.eval(ctx)? != b.eval(ctx)?)
            }
            (LogicOp::Eq, [a, b]) => {
                (a.eval(ctx)? == b.eval(ctx)?)
            }
            (LogicOp::Lt, [a, b]) => {
                (a.eval(ctx)? < b.eval(ctx)?)
            }
            (LogicOp::Leq, [a, b]) => {
                (a.eval(ctx)? <= b.eval(ctx)?)
            }
            (LogicOp::Gt, [a, b]) => {
                (a.eval(ctx)? > b.eval(ctx)?)
            }
            (LogicOp::Geq, [a, b]) => {
                (a.eval(ctx)? >= b.eval(ctx)?)
            }
            (LogicOp::All, [exprs..]) => {
                let r: StdResult<Vec<_>, _> = exprs.iter().map(|e| e.eval(ctx).and_then(|a| expect_bool(a))).collect();
                r?.iter().all(|a| *a)
            }
            (LogicOp::Any, [exprs..]) => {
                let r: StdResult<Vec<_>, _> = exprs.iter().map(|e| e.eval(ctx).and_then(|a| expect_bool(a))).collect();
                r?.iter().any(|a| *a)
            }
            (LogicOp::None, [exprs..]) => {
                let r: StdResult<Vec<_>, _> = exprs.iter().map(|e| e.eval(ctx).and_then(|a| expect_bool(a))).collect();
                r?.iter().all(|a| !*a)
            }
            (a, b) => {
                return Err(ExprEvalError::custom(format!("Invalid combination of logical operation and arguments: op {:?}, args : {:?}", self.0, self.1)));
            }
        }.into());
    }
}