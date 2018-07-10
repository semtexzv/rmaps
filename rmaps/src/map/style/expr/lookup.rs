use ::prelude::*;
use super::*;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum _AtMarker {
    #[serde(rename = "at")]
    _Mark
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum _GetMarker {
    #[serde(rename = "get")]
    _Mark
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum _HasMarker {
    #[serde(rename = "has")]
    _Mark
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum _LengthMarker {
    #[serde(rename = "length")]
    _Mark
}


#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum Lookup {
    At(_AtMarker, BaseExpr, BaseExpr),
    Get(_GetMarker, BaseExpr),
    GetExplicit(_GetMarker, BaseExpr, BaseExpr),
    Has(_HasMarker, BaseExpr),
    HasExplicit(_HasMarker, BaseExpr, BaseExpr),
    Length(_LengthMarker, BaseExpr),
}


impl Expr for Lookup {
    fn is_zoom(&self) -> bool {
        return match self {
            Lookup::At(_, a, b) => a.is_zoom() || b.is_zoom(),
            Lookup::Get(_, a) => a.is_zoom(),
            Lookup::GetExplicit(_, a, b) => a.is_zoom() || b.is_zoom(),
            Lookup::Has(_, a) => a.is_zoom(),
            Lookup::HasExplicit(_, a, b) => a.is_zoom() || b.is_zoom(),
            Lookup::Length(_, a) => a.is_zoom(),
        };
    }

    fn is_feature(&self) -> bool {
        return match self {
            Lookup::At(_, a, b) => a.is_feature() || b.is_feature(),
            Lookup::Get(_, _) => true,
            Lookup::GetExplicit(_, a, b) => a.is_feature() || b.is_feature(),
            Lookup::Has(_, _) => true,
            Lookup::HasExplicit(_, a, b) => a.is_feature() || b.is_feature(),
            Lookup::Length(_, a) => a.is_feature(),
        };
    }

    fn eval(&self, ctx: &EvaluationContext) -> ExprResult {
        match self {
            Lookup::At(_, pos, arr) => {
                let iv = expect_type(Type::Number, pos.eval(ctx)?)?;
                let i = iv.as_number().unwrap() as usize;

                let av = expect_type(Type::Array, arr.eval(ctx)?)?;
                let a = av.as_array().unwrap();
                assert!(i < a.len());
                return Ok(a[i].clone());
            }
            Lookup::Get(_, name) => {
                let n = expect_type(Type::String, name.eval(ctx)?)?;
                let n = n.as_str().unwrap();

                return if let Some(v) = ctx.get(n) {
                    Ok(v.clone())
                } else {
                    Ok(ExprVal::Null)
                };
            }
            Lookup::GetExplicit(_, name, o) => {
                let n = expect_type(Type::String, name.eval(ctx)?)?;
                let n = n.as_str().unwrap();

                let o = expect_type(Type::Object, o.eval(ctx)?)?;
                let o = o.as_object().unwrap();

                return if let Some(v) = o.get(n) {
                    Ok(v.clone())
                } else {
                    Ok(ExprVal::Null)
                };
            }
            Lookup::Has(_, name) => {
                let n = expect_type(Type::String, name.eval(ctx)?)?;
                let n = n.as_str().unwrap();

                return Ok(ctx.get(n).is_some().into());
            }

            Lookup::HasExplicit(_, n, o) => {
                let n = expect_type(Type::String, n.eval(ctx)?)?;
                let n = n.as_str().unwrap();

                let o = expect_type(Type::Object, o.eval(ctx)?)?;
                let o = o.as_object().unwrap();

                return Ok(o.contains_key(n).into());
            }
            Lookup::Length(_, e) => {
                return Ok(ExprVal::Num(match e.eval(ctx)? {
                    ExprVal::String(s) => s.len(),
                    ExprVal::List(l) => l.len(),
                    a @ _ => {
                        panic!("Error")
                    }
                } as f64));
            }
        }
        unimplemented!()
    }
}