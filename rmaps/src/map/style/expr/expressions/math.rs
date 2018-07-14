use super::prelude::*;

use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub enum MathOp {
    #[serde(rename = "-")]
    Minus,
    #[serde(rename = "*")]
    Times,
    #[serde(rename = "/")]
    Div,
    #[serde(rename = "%")]
    Remainder,
    #[serde(rename = "^")]
    Power,
    #[serde(rename = "+")]
    Sum,
    #[serde(rename = "abs")]
    Abs,
    #[serde(rename = "acos")]
    Acos,
    #[serde(rename = "asin")]
    Asin,
    #[serde(rename = "atan")]
    Atan,
    #[serde(rename = "ceil")]
    Ceil,
    #[serde(rename = "cos")]
    Cos,
    #[serde(rename = "e")]
    E,
    #[serde(rename = "floor")]
    Floor,
    #[serde(rename = "ln")]
    Ln,
    #[serde(rename = "ln2")]
    Ln2,
    #[serde(rename = "log10")]
    Log10,
    #[serde(rename = "log2")]
    Log2,
    #[serde(rename = "max")]
    Max,
    #[serde(rename = "min")]
    Min,
    #[serde(rename = "pi")]
    Pi,
    #[serde(rename = "round")]
    Round,
    #[serde(rename = "sin")]
    Sin,
    #[serde(rename = "sqrt")]
    Sqrt,
    #[serde(rename = "tan")]
    Tan,
}


impl FromStr for MathOp {
    type Err = serde::de::value::Error;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        use serde::de::IntoDeserializer;

        let deser: serde::de::value::StrDeserializer<_> = s.into_deserializer();
        Ok(Self::deserialize(deser)?)
    }
}

derive_parse!(MathOp);

#[derive(Debug, Clone)]
pub struct Math(MathOp, Vec<Expr>);

impl<'de> Deserialize<'de> for Math {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error> where
        D: Deserializer<'de> {
        NAME.with(|n| {
            let t = FromStr::from_str(&n)
                .map_err(|_| D::Error::custom("invalid type specifier for math operation"))?;
            let exprs = Deserialize::deserialize(deserializer)?;
            Ok(Math(t, exprs))
        })
    }
}
/*
impl Parse for Math {
    fn parse(value: json::Value, expected: Type) -> ParseResult<Self> {

        let First(op, rest) = First::parse(value, Type::String).map_err(|_| ParseError::NotThis)?;
        let mut rest: Vec<json::Value> = json::from_value(rest)?;
        let rest = rest.into_iter().map(|v| Expr::parse(v, Type::Number)).collect::<ParseResult<_>>()?;

        Ok(Math(op, rest))
    }
}
*/

impl Math {
    fn min_args(&self) -> usize {
        return match self.0 {
            MathOp::Minus => 1,
            MathOp::Times => 2,
            MathOp::Div => 2,
            MathOp::Remainder => 2,
            MathOp::Power => 2,
            MathOp::Sum => 2,
            MathOp::Abs => 1,
            MathOp::Acos => 1,
            MathOp::Asin => 1,
            MathOp::Atan => 1,
            MathOp::Ceil => 1,
            MathOp::Cos => 1,
            MathOp::E => 0,
            MathOp::Floor => 1,
            MathOp::Ln => 1,
            MathOp::Ln2 => 0,
            MathOp::Log10 => 1,
            MathOp::Log2 => 1,
            MathOp::Max => 1,
            MathOp::Min => 1,
            MathOp::Pi => 0,
            MathOp::Round => 1,
            MathOp::Sin => 1,
            MathOp::Sqrt => 1,
            MathOp::Tan => 1,
        };
    }
}

fn eval_num(e: &Expr, ctx: &EvaluationContext) -> StdResult<f64, EvalError> {
    let e = e.eval(ctx)?;
    return Ok(e.as_number().unwrap());
}

impl Expression for Math {
    fn is_zoom(&self) -> bool {
        return self.1.iter().any(|e| e.is_zoom());
    }

    fn is_feature(&self) -> bool {
        return self.1.iter().any(|e| e.is_feature());
    }

    fn eval(&self, ctx: &EvaluationContext) -> ExprResult {
        let _ = expect_len(&self.1, self.min_args())?;
        return Ok(match (&self.0, &self.1[..]) {
            (MathOp::Minus, [e]) => {
                (-eval_num(e, ctx)?)
            }
            (MathOp::Minus, [a, b]) => {
                (eval_num(a, ctx)? - eval_num(b, ctx)?)
            }
            (MathOp::Times, [a, rest..]) => {
                let mut r = eval_num(a, ctx)?;
                for b in rest {
                    r *= eval_num(b, ctx)?;
                }
                r
            }
            (MathOp::Div, [a, b]) => {
                eval_num(a, ctx)? / eval_num(b, ctx)?
            }
            (MathOp::Remainder, [a, b]) => {
                (eval_num(a, ctx)? as i64 % eval_num(b, ctx)? as i64) as f64
            }
            (MathOp::Power, [a, b]) => {
                f64::powf(eval_num(a, ctx)?, eval_num(b, ctx)?)
            }
            (MathOp::Sum, [a, rest..]) => {
                let mut r = eval_num(a, ctx)?;
                for b in rest {
                    r += eval_num(b, ctx)?;
                }
                r
            }
            (MathOp::Abs, [a]) => {
                f64::abs(eval_num(a, ctx)?)
            }
            (MathOp::Acos, [a]) => {
                f64::acos(eval_num(a, ctx)?)
            }
            (MathOp::Asin, [a]) => {
                f64::asin(eval_num(a, ctx)?)
            }
            (MathOp::Atan, [a]) => {
                f64::atan(eval_num(a, ctx)?)
            }
            (MathOp::Ceil, [a]) => {
                f64::ceil(eval_num(a, ctx)?)
            }
            (MathOp::Cos, [a]) => {
                f64::cos(eval_num(a, ctx)?)
            }
            (MathOp::E, []) => {
                ::std::f64::consts::E
            }
            (MathOp::Floor, [a]) => {
                f64::floor(eval_num(a, ctx)?)
            }
            (MathOp::Ln, [a]) => {
                f64::log(eval_num(a, ctx)?, ::std::f64::consts::E)
            }
            (MathOp::Ln2, []) => {
                f64::log(2., ::std::f64::consts::E)
            }
            (MathOp::Log10, [a]) => {
                f64::log10(eval_num(a, ctx)?)
            }
            (MathOp::Log2, [a]) => {
                f64::log2(eval_num(a, ctx)?)
            }
            (MathOp::Max, [a, rest..]) => {
                let mut r = eval_num(a, ctx)?;
                for b in rest {
                    r = f64::max(r, eval_num(b, ctx)?);
                }
                r
            }
            (MathOp::Min, [a, rest..]) => {
                let mut r = eval_num(a, ctx)?;
                for b in rest {
                    r = f64::min(r, eval_num(b, ctx)?);
                }
                r
            }
            (MathOp::Pi, []) => {
                ::std::f64::consts::PI
            }
            (MathOp::Round, [a]) => {
                f64::round(eval_num(a, ctx)?)
            }
            (MathOp::Sin, [a]) => {
                f64::sin(eval_num(a, ctx)?)
            }
            (MathOp::Sqrt, [a]) => {
                f64::sqrt(eval_num(a, ctx)?)
            }
            (MathOp::Tan, [a]) => {
                f64::tan(eval_num(a, ctx)?)
            }
            _ => {
                return Err(EvalError::custom(format!("Mismatched math operation and arguments : {:?} used on {:?}", self.0, self.1)));
            }
        }.into());
    }
}