use ::prelude::*;
use super::*;


#[derive(Debug,Clone,PartialEq)]
pub enum Str {
    Concat(Vec<BaseExpr>),
    Downcase(BaseExpr),
    Upcase(BaseExpr),
}

impl<'de> Deserialize<'de> for Str {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error> where
        D: Deserializer<'de> {
        #[derive(Deserialize)]
        enum _Marker {
            #[serde(rename = "concat")]
            Concat,
            #[serde(rename = "concat")]
            Downcase,
            #[serde(rename = "concat")]
            Upcase,
        }
        let PrefixHelper(mark, mut exprs) = Deserialize::deserialize(deserializer)?;

        match mark {
            _Marker::Concat => Ok(Str::Concat(exprs)),
            _Marker::Downcase => Ok(Str::Downcase(exprs.remove(0))),
            _Marker::Upcase => Ok(Str::Upcase(exprs.remove(0))),
        }
    }
}

impl Expr for Str {
    fn is_zoom(&self) -> bool {
        return match self {
            Str::Concat(v) => v.iter().any(|e| e.is_zoom()),
            Str::Downcase(e) => e.is_zoom(),
            Str::Upcase(e) => e.is_zoom(),
        };
    }

    fn is_feature(&self) -> bool {
        return match self {
            Str::Concat(v) => v.iter().any(|e| e.is_feature()),
            Str::Downcase(e) => e.is_feature(),
            Str::Upcase(e) => e.is_feature(),
        };
    }

    fn eval(&self, ctx: &EvaluationContext) -> ExprResult {
        return match self {
            Str::Concat(exprs) => {
                let mut res = String::new();

                for e in exprs {
                    let v = expect_type(Type::String, e.eval(ctx)?)?;
                    if let ExprVal::String(s) = v {
                        res.push_str(&s);
                    }
                }

                Ok(ExprVal::String(res))
            }
            Str::Downcase(e) => {
                let mut v = expect_type(Type::String, e.eval(ctx)?)?;
                if let ExprVal::String(s) = v {
                    return Ok(ExprVal::String(s.to_lowercase()));
                };
                panic!("Unexpected expression value ")
            }
            Str::Upcase(e) =>{
                let mut v = expect_type(Type::String, e.eval(ctx)?)?;
                if let ExprVal::String(s) = v {
                    return Ok(ExprVal::String(s.to_uppercase()));
                };
                panic!("Unexpected expression value ")
            }
        };
    }
}