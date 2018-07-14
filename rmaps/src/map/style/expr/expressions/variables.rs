use super::prelude::*;


#[derive(Debug, Clone)]
pub enum Variable {
    Let(Vec<(String, Expr)>, Expr),
    Var(String),
}

impl<'de> Deserialize<'de> for Variable {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error> where
        D: Deserializer<'de> {
        struct Vis;

        impl<'de> Visitor<'de> for Vis {
            type Value = Variable;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Variable expression")
            }

            fn visit_seq<A>(self, mut seq: A) -> StdResult<Self::Value, A::Error> where
                A: SeqAccess<'de>, {
                #[derive(Deserialize)]
                #[serde(untagged)]
                enum Wrap {
                    Name(String),
                    Expr(Expr),
                }

                let mut bindings = vec![];

                NAME.with(|n| {
                    match n.deref() {
                        "let" => {
                            'l: loop {
                                match (seq.next_element()?, seq.next_element()?) {
                                    (Some(Wrap::Name(name)), Some(b)) => {
                                        bindings.push((n.clone(), b));
                                    }
                                    (Some(Wrap::Expr(e)), None) => {
                                        break 'l Ok(Variable::Let(bindings,e));
                                    }
                                    _ => {
                                        unimplemented!()
                                    }
                                }
                            }
                        }
                        "var" => {
                            Ok(Variable::Var(seq.next_element()?.ok_or_else(|| de::Error::invalid_length(2, &self))?))
                        }
                        _ => {
                            panic!("Not a valid ident")
                        }
                    }
                })
            }
        }
        Ok(deserializer.deserialize_seq(Vis)?)
    }
}

/*
parse! {Variable as expected;
    "let", ... exprs: Vec<json::Value> => {

        let mut iter = exprs.into_iter();

            let mut bindings = vec![];
            'l: loop {
                match (iter.next(), iter.next()) {
                    (Some(a), Some(b)) => {
                        bindings.push((
                            json::from_value(a)?,
                            parse_val_expect(b, expected)?
                        ));
                    }
                    (Some(e), None) => {
                        return Ok(Variable::Let(bindings, parse_val_expect(e,expected)?));
                    }
                    _ => {
                        return Err(format_err!("Invalid number of bindings").into());

                    }
                }
            }
    }
    "var", name : String => {
        Ok(Variable::Var(name))
    }
}
*/

impl Expression for Variable {
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