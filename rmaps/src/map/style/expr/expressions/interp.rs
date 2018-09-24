use super::prelude::*;


#[derive(Debug, Clone)]
pub enum Interp {
    Interpolate(Interpolate),
    Step(Step),
}

impl Expression for Interp {
    fn is_zoom(&self) -> bool {
        delegate_to_inner! {self; [Interp::Interpolate, Interp::Step]; (v) => v.is_zoom()}
    }
    fn is_feature(&self) -> bool {
        delegate_to_inner! {self;  [Interp::Interpolate, Interp::Step]; (v) => v.is_feature()}
    }
    fn eval(&self, ctx: &EvaluationContext) -> ExprResult {
        delegate_to_inner! {self; [Interp::Interpolate, Interp::Step]; (v) => v.eval(ctx)}
    }
}


#[derive(Debug, Clone)]
pub enum InterpolateType {
    Linear,
    Exponential(f64),
    Cubic(f64, f64, f64, f64),
}

#[inline]
fn clamp(min: f64, max: f64, val: f64) -> f64 {
    f64::min(f64::max(val, 0.0), 1.0)
}

use std::ops::{
    Add, Mul, Sub,
};

fn lerp<T : ::common::lerp::Lerp<f64>>(a: T, b: T, factor: f64) -> T {
    a.lerp(b,factor as f64)
}

impl InterpolateType {
    fn get_factor(&self, a: f64, b: f64, value: f64) -> f64 {
        let range = b - a;
        let progress = value - a;
        return clamp(0., 1., match self {
            InterpolateType::Linear => {
                progress / range
            }
            InterpolateType::Exponential(base) => {
                progress / range
                /*
                (f64::powf(*base, progress) - 1.) /
                    (f64::powf(*base, range) - 1.)
                    */
            }
            InterpolateType::Cubic(x1, y1, x2, y2) => {
                panic!("Cubic bezier interpolation not yet supported")
            }
        });
    }
}

impl<'de> Deserialize<'de> for InterpolateType {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de> {
        let mut input: Vec<json::Value> = Deserialize::deserialize(deserializer)?;
        if input.len() < 1 {
            return Err(D::Error::custom("Array too short"));
        }

        let name = input.remove(0);

        return if name.as_str() == Some("linear") {
            Ok(InterpolateType::Linear)
        } else if name.as_str() == Some("exponential") && input.len() >= 1 {
            let base: StdResult<f64, _> = json::from_value(input.remove(0));

            base.map(|v| InterpolateType::Exponential(v)).map_err(|_| D::Error::custom("Invalid exponential exponent"))
        } else if name.as_str() == Some("cubic-bezier") && input.len() >= 4 {
            let points: StdResult<Vec<f64>, _> = input.into_iter().map(json::from_value).collect();

            points.map(|p| InterpolateType::Cubic(p[0], p[1], p[2], p[3])).map_err(|_| D::Error::custom("Invalid bezier control points"))
        } else {
            Err(D::Error::custom("Could not parse expression as interpolation specifier"))
        };
    }
}

#[derive(Debug, Clone)]
pub struct Stop {
    val: f64,
    out: Expr,
}

#[derive(Debug, Clone)]
pub struct Interpolate {
    typ: InterpolateType,
    input: Expr,
    stops: Vec<Stop>,
}

impl<'de> Deserialize<'de> for Interpolate {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error> where
        D: Deserializer<'de> {
        struct Vis;
        impl<'de> Visitor<'de> for Vis {
            type Value = Interpolate;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Interpolate expression")
            }

            fn visit_seq<A>(self, mut seq: A) -> StdResult<Self::Value, A::Error> where A: SeqAccess<'de>, {
                let typ = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(1, &self))?;


                let input = TYPE.set(&Type::Number, || {
                    seq.next_element()
                })?.ok_or_else(|| de::Error::invalid_length(2, &self))?;


                let mut stops = vec![];
                while let (Some(val), Some(out)) = (seq.next_element()?, TYPE.set(&Type::Color, || { seq.next_element() })?) {
                    stops.push(Stop {
                        val,
                        out,
                    })
                };

                Ok(Interpolate {
                    typ,
                    input,
                    stops,
                })
            }
        }

        Ok(deserializer.deserialize_seq(Vis)?)
    }
}

impl Expression for Interpolate {
    fn is_zoom(&self) -> bool {
        self.input.is_zoom() || self.stops.iter().any(|s| s.out.is_zoom())
    }

    fn is_feature(&self) -> bool {
        let inp = self.input.is_feature();
        let stops = self.stops.iter().any(|s| s.out.is_feature());

        inp || stops
    }

    fn eval(&self, ctx: &EvaluationContext) -> ExprResult {
        let val = expect_num(self.input.eval(ctx)?)?;


        let lower = self.stops.iter().rfind(|s| s.val <= val);
        let upper = self.stops.iter().find(|s| s.val >= val);
        match (lower, upper) {
            (Some(l), Some(h)) => {
                let factor = self.typ.get_factor(l.val, h.val, val);

                let low = l.out.eval(ctx)?;
                let high = h.out.eval(ctx)?;

                fn do_lerp(low: &Value, high: &Value, factor: f64) -> ExprResult {
                    if low.get_type() != high.get_type() {
                        return Err(EvalError::custom(format!("Mismatch between interpolate output types: low: {:?} high : {:?} ", low.get_type(), high.get_type())));
                    }

                    Ok(match (low, high) {
                        (Value::Num(a), Value::Num(b)) => Value::Num(lerp(*a, *b, factor)),
                        (Value::Color(a), Value::Color(b)) => {
                            Value::Color(lerp(*a, *b, factor))
                        }
                        (Value::String(a), Value::String(b)) => {
                            let a = Color::from_str(a).unwrap();
                            let b = Color::from_str(b).unwrap();
                            Value::Color(lerp(a, b, factor))
                        }
                        (Value::List(a), Value::List(b)) => {
                            let mut res = vec![];
                            if a.len() != b.len() {
                                return Err(EvalError::custom(format!("Mismatch between interpolate array lengths: a: {:?} b : {:?} ", a.len(), b.len())));
                            }

                            for (a, b) in a.iter().zip(b.iter()) {
                                res.push(do_lerp(a, b, factor)?)
                            }
                            Value::List(res)
                        }
                        (a @ _, b @ _) => {
                            return Err(EvalError::custom(format!("Interpolate not supported for type pair: {:?} and {:?}", a.get_type(), b.get_type())));
                        }
                    })
                }

                let res = do_lerp(&low, &high, factor);


                return res;
            }
            (Some(x), None) | (None, Some(x)) => {
                return x.out.eval(ctx);
                // Return
            }
            (None, None) => {
                panic!("No values to interpolate between found")
                // Error, no valid values provided
            }
        }
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
pub struct Step {
    input: Expr,
    default: Expr,
    stops: Vec<Stop>,
}

impl<'de> Deserialize<'de> for Step {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error> where
        D: Deserializer<'de> {
        struct Vis;

        impl<'de> Visitor<'de> for Vis {
            type Value = Step;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Step expression")
            }

            fn visit_seq<A>(self, mut seq: A) -> StdResult<Self::Value, A::Error> where
                A: SeqAccess<'de>, {
                let input = TYPE.set(&Type::Number, || {
                    seq.next_element()
                })?.ok_or_else(|| de::Error::invalid_length(1, &self))?;

                let default = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(2, &self))?;

                let mut stops = vec![];
                while let (Some(val), Some(out)) = (seq.next_element()?, seq.next_element()?) {
                    stops.push(Stop {
                        val,
                        out,
                    })
                };

                Ok(Step {
                    input,
                    default,
                    stops,
                })
            }
        }
        Ok(deserializer.deserialize_seq(Vis)?)
    }
}

parse! {Step as exp;
    "step", input : BaseExpr as Type::Number, default : BaseExpr as exp, ... arr : Vec<json::Value> => {
        let mut iter = arr.into_iter();
        let mut stops = vec![];

        return 'l: loop {
            match (iter.next().map(json::from_value), iter.next().map(|v| parse_val_expect(v,exp))) {
                (Some(Ok(k)), Some(Ok(v))) => {
                    stops.push(Stop {
                        val: k,
                        out: v,
                    });
                }
                (a @ Some(Err(_)), b @ _) | (a @ _, b @ Some(Err(_))) => {
                    return Err(format_err!("Could not parse step arm : input : {:?}, output : {:?}", a, b).into());
                }
                _ => {
                    break 'l Ok(Step {
                        input,
                        default,
                        stops: stops,
                    });
                }
            }
        };
    }
}

impl Expression for Step {
    fn is_zoom(&self) -> bool {
        self.input.is_zoom() || self.default.is_zoom() || self.stops.iter().any(|a| a.out.is_zoom())
    }

    fn is_feature(&self) -> bool {
        self.input.is_feature() || self.default.is_feature() || self.stops.iter().any(|a| a.out.is_feature())
    }

    fn eval(&self, ctx: &EvaluationContext) -> ExprResult {
        let val = expect_num(self.input.eval(ctx)?)?;
        let stop = self.stops.iter().find(|a| a.val <= val);
        return if let Some(stop) = stop {
            stop.out.eval(ctx)
        } else {
            self.default.eval(ctx)
        };
    }
}