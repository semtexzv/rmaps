use super::prelude::*;


#[derive(Debug, Clone)]
pub enum Types {
    Array(ArrayAssert),
    Assert(Assert),
    Literal(Literal),
    Convert(Convert),
    Typeof(TypeOf),
}

impl Expression for Types {
    fn is_zoom(&self) -> bool {
        delegate_to_inner! {self; [Types::Array, Types::Assert, Types::Literal, Types::Convert, Types::Typeof]; (v) => v.is_zoom()}
    }
    fn is_feature(&self) -> bool {
        delegate_to_inner! {self; [Types::Array, Types::Assert, Types::Literal, Types::Convert, Types::Typeof]; (v) => v.is_feature()}
    }
    fn eval(&self, ctx: &EvaluationContext) -> ExprResult {
        delegate_to_inner! {self; [Types::Array, Types::Assert, Types::Literal, Types::Convert, Types::Typeof]; (v) => v.eval(ctx)}
    }
}

impl<'de> Deserialize<'de> for Types {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error> where
        D: Deserializer<'de> {
        NAME.with(|n| {
            match n.deref() {
                "array" => Ok(Types::Array(Deserialize::deserialize(deserializer)?)),
                "boolean" | "number" | "object" | "string" => Ok(Types::Assert(Deserialize::deserialize(deserializer)?)),
                "literal" => Ok(Types::Literal(Deserialize::deserialize(deserializer)?)),
                "to-boolean" | "to-color" | "to-number" | "to-string" => Ok(Types::Convert(Deserialize::deserialize(deserializer)?)),
                "typeof" => Ok(Types::Typeof(Deserialize::deserialize(deserializer)?)),
                _ => {
                    panic!("unknown expression {:?}", n);
                }
            }
        })
    }
}

#[derive(Debug, Clone)]
pub struct ArrayAssert(Option<Type>, Expr);

impl<'de> Deserialize<'de> for ArrayAssert {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error> where
        D: Deserializer<'de> {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Help {
            ArrT(Type, Expr),
            Arr(Vec<Expr>),
        };
        Ok(match Deserialize::deserialize(deserializer)? {
            Help::Arr(mut v) => ArrayAssert(None, v.remove(0)),
            Help::ArrT(t, v) => ArrayAssert(Some(t), v)
        })
    }
}


impl Expression for ArrayAssert {
    fn is_zoom(&self) -> bool {
        self.1.is_zoom()
    }

    fn is_feature(&self) -> bool {
        self.1.is_feature()
    }

    fn eval(&self, ctx: &EvaluationContext) -> ExprResult {
        match self {
            ArrayAssert(None, e) => {
                return Ok(if let Value::List(_) = e.eval(ctx)? {
                    Value::Bool(true)
                } else {
                    Value::Bool(false)
                });
            }
            ArrayAssert(Some(typ), e) => {
                return Ok(if let Value::List(l) = e.eval(ctx)? {
                    return Ok(l.iter().all(|v| v.get_type() == *typ).into());
                } else {
                    Value::Bool(false)
                });
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Assert(Type, Vec<Expr>);

impl<'de> Deserialize<'de> for Assert {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error> where D: Deserializer<'de> {
        return NAME.with(|n| {
            let t = FromStr::from_str(&n)
                .map_err(|_| D::Error::custom("invalid type specifier for assert"))?;
            let exprs = Deserialize::deserialize(deserializer)?;
            Ok(Assert(t, exprs))
        });
    }
}


impl Expression for Assert {
    fn is_zoom(&self) -> bool {
        return self.1.iter().any(|v| v.is_zoom());
    }

    fn is_feature(&self) -> bool {
        return self.1.iter().any(|v| v.is_feature());
    }

    fn eval(&self, ctx: &EvaluationContext) -> ExprResult {
        for e in self.1.iter() {
            let v = e.eval(ctx)?;
            if v.get_type() == self.0 {
                return Ok(v);
            }
        }
        return Ok(Value::Null);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Literal(Value);

impl<'de> Deserialize<'de> for Literal {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error> where D: Deserializer<'de> {
        TYPE.set(&Type::String, || {
            let v = Deserialize::deserialize(deserializer)?;
            // TODO, check type
            Ok(Literal(v))
        })
    }
}


impl Expression for Literal {
    fn is_zoom(&self) -> bool {
        false
    }

    fn is_feature(&self) -> bool {
        false
    }
    fn eval(&self, ctx: &EvaluationContext) -> ExprResult {
        return Ok(self.0.clone());
    }
}


#[derive(Debug, Clone)]
pub struct Convert(Type, Vec<Expr>);

impl<'de> Deserialize<'de> for Convert {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error> where
        D: Deserializer<'de> {
        let typ = NAME.with(|n|
            Ok(match n.deref() {
                "to-boolean" => Type::Boolean,
                "to-color" => Type::Color,
                "to-number" => Type::Number,
                "to-string" => Type::String,
                _ => {
                    return Err(D::Error::custom("Invalid name for convert expr"));
                }
            }))?;
        let v = Deserialize::deserialize(deserializer)?;
        Ok(Convert(typ, v))
    }
}

impl Expression for Convert {
    fn is_zoom(&self) -> bool {
        self.1.iter().any(|e| e.is_zoom())
    }

    fn is_feature(&self) -> bool {
        self.1.iter().any(|e| e.is_feature())
    }

    fn eval(&self, ctx: &EvaluationContext) -> ExprResult {
        unimplemented!()
    }
}


#[derive(Debug, Clone)]
pub struct TypeOf(Expr);

impl<'de> Deserialize<'de> for TypeOf {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error> where
        D: Deserializer<'de> {
        let mut v: Vec<Expr> = Deserialize::deserialize(deserializer)?;
        Ok(TypeOf(v.remove(0)))
    }
}

impl Expression for TypeOf {
    fn is_zoom(&self) -> bool {
        self.0.is_zoom()
    }

    fn is_feature(&self) -> bool {
        self.0.is_feature()
    }

    fn eval(&self, ctx: &EvaluationContext) -> ExprResult {
        let v = self.0.eval(ctx)?;
        return Ok(format!("{:?}", v.get_type()).into());
    }
}

