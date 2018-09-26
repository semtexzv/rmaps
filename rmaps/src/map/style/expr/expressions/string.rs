use super::prelude::*;


#[derive(Debug, Clone)]
pub enum Str {
    Concat(Vec<Expr>),
    Downcase(Expr),
    Upcase(Expr),
}

impl<'de> Deserialize<'de> for Str {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error> where
        D: Deserializer<'de> {
        struct Vis;

        impl<'de> Visitor<'de> for Vis {
            type Value = Str;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("String expr")
            }

            fn visit_seq<A>(self, mut seq: A) -> StdResult<Self::Value, A::Error> where
                A: SeqAccess<'de>, {
                NAME.with(|n| {
                    match n.deref() {
                        "concat" => {
                            let d = Deserialize::deserialize(de::value::SeqAccessDeserializer::new(seq))?;
                            Ok(Str::Concat(d))
                        }
                        "downcase" => {
                            let s = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(1, &self))?;
                            Ok(Str::Downcase(s))
                        }
                        "upcase" => {
                            let s = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(1, &self))?;
                            Ok(Str::Upcase(s))
                        }
                        _ => unimplemented!()
                    }
                })
            }
        }

        Ok(deserializer.deserialize_seq(Vis)?)
    }
}


impl Expression for Str {
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
                    if let Value::String(s) = v {
                        res.push_str(&s);
                    }
                }

                Ok(Value::String(res))
            }
            Str::Downcase(e) => {
                let mut v = expect_type(Type::String, e.eval(ctx)?)?;
                if let Value::String(s) = v {
                    return Ok(Value::String(s.to_lowercase()));
                };
                panic!("Unexpected expression value ")
            }
            Str::Upcase(e) => {
                let mut v = expect_type(Type::String, e.eval(ctx)?)?;
                if let Value::String(s) = v {
                    return Ok(Value::String(s.to_uppercase()));
                };
                panic!("Unexpected expression value ")
            }
        };
    }
}