use super::prelude::*;

#[derive(Debug, Clone)]
pub enum Lookup {
    At(Expr, Expr),
    Get(Expr, Option<Expr>),
    Has(Expr, Option<Expr>),
    Length(Expr),
}

impl<'de> Deserialize<'de> for Lookup {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error> where
        D: Deserializer<'de> {
        return NAME.with(|n| {
            match n.deref() {
                "at" => {
                    struct Vis;
                    impl<'de> Visitor<'de> for Vis {
                        type Value = Lookup;

                        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                            formatter.write_str("arguments")
                        }

                        fn visit_seq<A>(self, mut seq: A) -> StdResult<Self::Value, A::Error> where A: SeqAccess<'de>, {
                            let first = TYPE.set(&Type::Number, || {
                                seq.next_element()
                            })?.ok_or_else(|| de::Error::invalid_length(1, &self))?;

                            let second = TYPE.set(&Type::Array, || {
                                seq.next_element()
                            })?.ok_or_else(|| de::Error::invalid_length(1, &self))?;

                            Ok(Lookup::At(first, second))
                        }
                    }
                    Ok(deserializer.deserialize_seq(Vis)?)
                }
                "get" => {
                    struct Vis;
                    impl<'de> Visitor<'de> for Vis {
                        type Value = Lookup;

                        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                            formatter.write_str("arguments")
                        }

                        fn visit_seq<A>(self, mut seq: A) -> StdResult<Self::Value, A::Error> where A: SeqAccess<'de>, {
                            let first = TYPE.set(&Type::String, || {
                                seq.next_element()
                            })?.ok_or_else(|| de::Error::invalid_length(1, &self))?;

                            let second = TYPE.set(&Type::Object, || {
                                seq.next_element()
                            })?;

                            Ok(Lookup::Get(first, second))
                        }
                    }
                    Ok(deserializer.deserialize_seq(Vis)?)
                }
                "has" => {
                    struct Vis;
                    impl<'de> Visitor<'de> for Vis {
                        type Value = Lookup;

                        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                            formatter.write_str("arguments")
                        }

                        fn visit_seq<A>(self, mut seq: A) -> StdResult<Self::Value, A::Error> where A: SeqAccess<'de>, {
                            let first = TYPE.set(&Type::String, || {
                                seq.next_element()
                            })?.ok_or_else(|| de::Error::invalid_length(1, &self))?;

                            let second = TYPE.set(&Type::Object, || {
                                seq.next_element()
                            })?;

                            Ok(Lookup::Has(first, second))
                        }
                    }
                    Ok(deserializer.deserialize_seq(Vis)?)
                }
                "length" => {
                    struct Vis;
                    impl<'de> Visitor<'de> for Vis {
                        type Value = Lookup;

                        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                            formatter.write_str("arguments")
                        }

                        fn visit_seq<A>(self, mut seq: A) -> StdResult<Self::Value, A::Error> where A: SeqAccess<'de>, {
                            let first = TYPE.set(&Type::String, || {
                                seq.next_element()
                            })?.ok_or_else(|| de::Error::invalid_length(1, &self))?;

                            Ok(Lookup::Length(first))
                        }
                    }
                    Ok(deserializer.deserialize_seq(Vis)?)
                }
                _ => {
                    panic!("Not a valid ident")
                }
            }
        });
    }
}

impl Expression for Lookup {
    fn is_zoom(&self) -> bool {
        false
    }

    fn is_feature(&self) -> bool {
        true
    }

    fn eval(&self, ctx: &EvaluationContext) -> ExprResult {
        Ok(::common::rand::random::<f32>().into())
    }
}
/*

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
                    Ok(Value::Null)
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
                    Ok(Value::Null)
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
                return Ok(Value::Num(match e.eval(ctx)? {
                    Value::String(s) => s.len(),
                    Value::List(l) => l.len(),
                    a @ _ => {
                        panic!("Error")
                    }
                } as f64));
            }
        }
        unimplemented!()
    }
}
*/