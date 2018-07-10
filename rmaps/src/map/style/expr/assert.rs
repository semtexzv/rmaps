use ::prelude::*;

use super::*;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, PartialOrd, PartialEq, Ord, Eq)]
pub enum ArrayMarker {
    #[serde(rename = "array")]
    _Marker
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ArrayAssert {
    Check(ArrayMarker, BaseExpr),
    CheckType(ArrayMarker, Type, BaseExpr),
}


impl Expr for ArrayAssert {
    fn is_zoom(&self) -> bool {
        match self {
            ArrayAssert::Check(_, e) => e.is_zoom(),
            ArrayAssert::CheckType(_, _, e) => e.is_zoom(),
        }
    }

    fn is_feature(&self) -> bool {
        match self {
            ArrayAssert::Check(_, e) => e.is_feature(),
            ArrayAssert::CheckType(_, _, e) => e.is_feature(),
        }
    }

    fn eval(&self, ctx: &EvaluationContext) -> ExprResult {
        match self {
            ArrayAssert::Check(_, e) => {
                return Ok(if let ExprVal::List(_) = e.eval(ctx)? {
                    ExprVal::Bool(true)
                } else {
                    ExprVal::Bool(false)
                });
            }
            ArrayAssert::CheckType(_, typ, e) => {
                return Ok(if let ExprVal::List(l) = e.eval(ctx)? {
                    return Ok(l.iter().all(|v| v.typ() == *typ).into());
                } else {
                    ExprVal::Bool(false)
                });
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Assert(Type, Vec<BaseExpr>);

impl<'de> Deserialize<'de> for Assert {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error> where
        D: Deserializer<'de> {
        use serde::de::Error;

        let PrefixHelper(typ, rest) = Deserialize::deserialize(deserializer)?;
        return Ok(Assert(typ, rest));
    }
}


impl Expr for Assert {
    fn is_zoom(&self) -> bool {
        return self.1.iter().any(|v| v.is_zoom());
    }

    fn is_feature(&self) -> bool {
        return self.1.iter().any(|v| v.is_feature());
    }

    fn eval(&self, ctx: &EvaluationContext) -> ExprResult {
        for e in self.1.iter() {
            let v = e.eval(ctx)?;
            if v.typ() == self.0 {
                return Ok(v);
            }
        }
        return Ok(ExprVal::Null);
    }
}
