use super::prelude::*;


#[derive(Debug, Clone)]
pub enum ColorExpr {
    Rgb(Expr, Expr, Expr),
    Rgba(Expr, Expr, Expr, Expr),
    ToRgba(Expr),
}

impl<'de> Deserialize<'de> for ColorExpr {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error> where
        D: Deserializer<'de> {
        struct Vis;

        impl<'de> Visitor<'de> for Vis {
            type Value = ColorExpr;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("color expression")
            }

            fn visit_seq<A>(self, mut seq: A) -> StdResult<Self::Value, A::Error> where A: SeqAccess<'de>, {
                NAME.with(|n| {
                    match n.deref() {
                        "rgb" => {
                            let r = TYPE.set(&Type::Number, || {
                                seq.next_element()
                            })?.ok_or_else(|| de::Error::invalid_length(1, &self))?;
                            let g = TYPE.set(&Type::Number, || {
                                seq.next_element()
                            })?.ok_or_else(|| de::Error::invalid_length(2, &self))?;
                            let b = TYPE.set(&Type::Number, || {
                                seq.next_element()
                            })?.ok_or_else(|| de::Error::invalid_length(3, &self))?;

                            Ok(ColorExpr::Rgb(r, g, b))
                        }
                        "rgba" => {
                            let r = TYPE.set(&Type::Number, || {
                                seq.next_element()
                            })?.ok_or_else(|| de::Error::invalid_length(1, &self))?;
                            let g = TYPE.set(&Type::Number, || {
                                seq.next_element()
                            })?.ok_or_else(|| de::Error::invalid_length(2, &self))?;
                            let b = TYPE.set(&Type::Number, || {
                                seq.next_element()
                            })?.ok_or_else(|| de::Error::invalid_length(3, &self))?;
                            let a = TYPE.set(&Type::Number, || {
                                seq.next_element()
                            })?.ok_or_else(|| de::Error::invalid_length(3, &self))?;
                            Ok(ColorExpr::Rgba(r, g, b, a))
                        }
                        "to-rgba" => {
                            let c = TYPE.set(&Type::Color, || {
                                seq.next_element()
                            })?.ok_or_else(|| de::Error::invalid_length(1, &self))?;
                            Ok(ColorExpr::ToRgba(c))
                        }
                        _ => {
                            panic!("Unknown color expr")
                        }
                    }
                })
            }
        }
        Ok(deserializer.deserialize_seq(Vis)?)
    }
}

parse! { ColorExpr as expected;
    "rgb" ,
    a : BaseExpr as Type::Number,
    b : BaseExpr as Type::Number ,
    c : BaseExpr as Type::Number => {
        Ok(ColorExpr::Rgb(a,b,c))
    }
    "rgba" ,
    a : BaseExpr as Type::Number,
    b : BaseExpr as Type::Number ,
    c : BaseExpr as Type::Number,
    d : BaseExpr as Type::Number => {
        Ok(ColorExpr::Rgba(a,b,c,d))
    }
    "to-rgba",
    c : BaseExpr as Type::Color => {
        Ok(ColorExpr::ToRgba(c))
    }
}

impl Expression for ColorExpr {
    fn is_zoom(&self) -> bool {
        match self {
            ColorExpr::Rgb(a, b, c) => a.is_zoom() || b.is_zoom() || c.is_zoom(),
            ColorExpr::Rgba(a, b, c, d) => a.is_zoom() || b.is_zoom() || c.is_zoom() || d.is_zoom(),
            ColorExpr::ToRgba(x) => x.is_zoom(),
        }
    }

    fn is_feature(&self) -> bool {
        match self {
            ColorExpr::Rgb(a, b, c) => a.is_feature() || b.is_feature() || c.is_feature(),
            ColorExpr::Rgba(a, b, c, d) => a.is_feature() || b.is_feature() || c.is_feature() || d.is_feature(),
            ColorExpr::ToRgba(x) => x.is_feature(),
        }
    }

    fn eval(&self, ctx: &EvaluationContext) -> ExprResult {
        match self {
            ColorExpr::Rgb(a, b, c) => {
                let ea = expect_num(a.eval(ctx)?)?;
                let eb = expect_num(b.eval(ctx)?)?;
                let ec = expect_num(c.eval(ctx)?)?;

                return Ok(Value::Color(Color::new(ea as f32, eb as f32, ec as f32, 1.)));
            }
            ColorExpr::Rgba(a, b, c, d) => {
                let ea = expect_num(a.eval(ctx)?)?;
                let eb = expect_num(b.eval(ctx)?)?;
                let ec = expect_num(c.eval(ctx)?)?;
                let ed = expect_num(d.eval(ctx)?)?;

                return Ok(Value::Color(Color::new(ea as f32, eb as f32, ec as f32, ed as f32)));
            }
            ColorExpr::ToRgba(x) => {
                let ea = expect_color(x.eval(ctx)?)?;
                return Ok(Value::List(
                    vec![
                        ea.r().into(),
                        ea.g().into(),
                        ea.b().into(),
                        ea.a().into(),
                    ]
                ));
            }
        }
    }
}
