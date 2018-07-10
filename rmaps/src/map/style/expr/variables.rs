use ::prelude::*;
use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Variable {
    Let(Vec<(String, BaseExpr)>, BaseExpr),
    Var(String),
}

impl<'de> Deserialize<'de> for Variable {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error> where
        D: Deserializer<'de> {
        use serde::de::Error;

        #[derive(Deserialize)]
        enum _Mark {
            #[serde(rename = "let")]
            Let,
            #[serde(rename = "var")]
            Var,
        };

        let PrefixHelper(mark, mut exprs): PrefixHelper<_Mark, json::Value> = Deserialize::deserialize(deserializer)?;

        if let _Mark::Var = mark {
            return Ok(Variable::Var(json::from_value(exprs.remove(0)).map_err(|_| D::Error::custom("invalid var argument"))?));
        }

        if let _Mark::Let = mark {
            let mut iter = exprs.into_iter();

            let mut bindings = vec![];
            'l: loop {
                match (iter.next(), iter.next()) {
                    (Some(a), Some(b)) => {
                        bindings.push((
                            json::from_value(a).map_err(|_| D::Error::custom("Invalid let binding name"))?,
                            json::from_value(b).map_err(|_| D::Error::custom("Invalid let binding expression"))?
                        ));
                    }
                    (Some(e), None) => {
                        return Ok(Variable::Let(bindings, json::from_value(e).map_err(|_| D::Error::custom("Invalid let output expr"))?));
                    }
                    _ => {
                        return Err(D::Error::custom("Invaliud let binding number of arguments"));
                    }
                }
            }
        }

        panic!("Should not happen")
    }
}

impl Expr for Variable {
    fn is_zoom(&self) -> bool {
        return match self {
            Variable::Let(a, b) => a.iter().any(|(k, v)| v.is_zoom()) || b.is_zoom(),
            _ => false,
        };
    }

    fn is_feature(&self) -> bool {
        return match self {
            Variable::Let(a, b) => a.iter().any(|(k, v)| v.is_feature()) || b.is_feature(),
            _ => false,
        };
    }

    fn eval(&self, ctx: &EvaluationContext) -> ExprResult {
        match self {
            Variable::Let(bindings, expr) => {
                let mut old_bindings = BTreeMap::new();

                if let Ok(mut ctx_bindings) = ctx.bindings.try_borrow_mut() {
                    for (k, e) in bindings {
                        if let Some(old) = ctx_bindings.insert(k.clone(), e.clone()) {
                            old_bindings.insert(k.clone(), old);
                        }
                    }
                }

                let res = expr.eval(ctx)?;

                if let Ok(mut ctx_bindings) = ctx.bindings.try_borrow_mut() {
                    for (k, e) in old_bindings {
                        ctx_bindings.insert(k, e);
                    }
                }
                return Ok(res);
            }
            Variable::Var(name) => {
                let e = { ctx.bindings.borrow().get(name).map(|x| x.clone()) };
                if let Some(e) = e {
                    return e.eval(ctx);
                }
                panic!("Did not find expression by name");
            }
        }
    }
}