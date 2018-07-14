use super::prelude::*;


#[derive(Debug, Clone)]
pub enum Decision {
    Case(Case),
    Match(Match),
    Coalesce(Coalesce),
    Logic(Logic),
}

impl<'de> Deserialize<'de> for Decision {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error> where
        D: Deserializer<'de> {
        NAME.with(|n| {
            match n.deref() {
                "case" => Ok(Decision::Case(Deserialize::deserialize(deserializer)?)),
                "match" => Ok(Decision::Match(Deserialize::deserialize(deserializer)?)),
                "coalesce" => Ok(Decision::Coalesce(Deserialize::deserialize(deserializer)?)),
                _ => Ok(Decision::Logic(Deserialize::deserialize(deserializer)?)),
            }
        })
    }
}

impl Expression for Decision {
    fn is_zoom(&self) -> bool {
        delegate_to_inner! {self; [Decision::Case, Decision::Match, Decision::Coalesce, Decision::Logic]; (v) => v.is_zoom()}
    }
    fn is_feature(&self) -> bool {
        delegate_to_inner! {self;  [Decision::Case, Decision::Match, Decision::Coalesce,  Decision::Logic]; (v) => v.is_feature()}
    }
    fn eval(&self, ctx: &EvaluationContext) -> ExprResult {
        delegate_to_inner! {self; [Decision::Case, Decision::Match, Decision::Coalesce,  Decision::Logic]; (v) => v.eval(ctx)}
    }
}


#[derive(Debug, Clone)]
pub struct Case(Vec<(Expr, Expr)>, Expr);

impl<'de> Deserialize<'de> for Case {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error> where
        D: Deserializer<'de> {
        unimplemented!()
    }
}


impl Expression for Case {
    fn is_zoom(&self) -> bool {
        return self.0.iter().any(|(a, b)| a.is_zoom() || b.is_zoom()) || self.1.is_zoom();
    }

    fn is_feature(&self) -> bool {
        return self.0.iter().any(|(a, b)| a.is_feature() || b.is_feature()) || self.1.is_feature();
    }

    fn eval(&self, ctx: &EvaluationContext) -> ExprResult {
        for (cond, val) in self.0.iter() {
            let test = expect_type(Type::Boolean, cond.eval(ctx)?)?;
            if let Value::Bool(test) = test {
                return val.eval(ctx);
            }
        }
        self.1.eval(ctx)
    }
}

#[derive(Debug, Clone)]
pub struct Coalesce(Vec<Expr>);

impl<'de> Deserialize<'de> for Coalesce {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error> where
        D: Deserializer<'de> {
        unimplemented!()
    }
}

impl Expression for Coalesce {
    fn is_zoom(&self) -> bool {
        self.0.iter().any(|v| v.is_zoom())
    }

    fn is_feature(&self) -> bool {
        self.0.iter().any(|v| v.is_feature())
    }

    fn eval(&self, ctx: &EvaluationContext) -> ExprResult {
        for e in self.0.iter() {
            let v = e.eval(ctx)?;
            if v.get_type() != Type::Null {
                return Ok(v);
            }
        }
        return Ok(Value::Null);
    }
}

#[derive(Debug, Clone)]
pub struct MatchArm {
    label: Value,
    expr: Expr,
}

#[derive(Debug, Clone)]
pub struct Match {
    input: Expr,
    arms: Vec<MatchArm>,
    default: Expr,
}

impl<'de> Deserialize<'de> for Match {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error> where
        D: Deserializer<'de> {
        unimplemented!()
    }
}
/*
parse! { Match as exp;
    "match", input : BaseExpr as Type::String, ... exprs : Vec<json::Value> => {
        let mut iter = exprs.into_iter();

        let mut arms = vec![];
        return 'l: loop {
            match (iter.next(), iter.next().map(|v| parse_val_expect(v,exp))){
                (Some(k), Some(Ok(v))) => {
                    arms.push(MatchArm {
                        label:  parse_val_expect::<Value>(k,Type::Array)?,
                        expr: v,
                    });
                }
                (Some(default), None) => {
                    break 'l Ok(Match {
                        input,
                        arms,
                        default :  parse_val_expect(default,exp)?
                    });
                }
                a @ _ => {
                    return Err(format_err!("Invalid match expression {:?}", a).into());
                }
            }
        };
    }
}

*/
impl Expression for Match {
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


impl FromStr for LogicOp {
    type Err = serde::de::value::Error;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        use serde::de::IntoDeserializer;

        let deser: serde::de::value::StrDeserializer<_> = s.into_deserializer();
        Ok(Self::deserialize(deser)?)
    }
}


impl LogicOp {
    fn min_args(&self) -> usize {
        0
    }
}

#[derive(Debug, Clone)]
pub struct Logic(LogicOp, Vec<Expr>);

impl<'de> Deserialize<'de> for Logic {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error> where
        D: Deserializer<'de> {
        unimplemented!()
    }
}
/*
impl Parse for Logic {
    fn parse(value: json::Value, expected: Type) -> ParseResult<Self> {

        let First(op, rest) = First::parse(value, Type::String)?;
        let mut rest: Vec<json::Value> = json::from_value(rest)?;
        let rest = rest.into_iter().map(|v| Expr::parse(v, Type::String)).collect::<ParseResult<_>>()?;

        Ok(Logic(op, rest))
    }
}
*/


impl Expression for Logic {
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
                return Err(EvalError::custom(format!("Invalid combination of logical operation and arguments: op {:?}, args : {:?}", self.0, self.1)));
            }
        }.into());
    }
}



