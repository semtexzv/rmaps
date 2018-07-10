use prelude::*;
use super::*;

use serde::Deserialize;
use serde::de::Error;


#[derive(Debug, Clone, PartialEq)]
pub enum InterpType {
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

#[inline]
fn lerp<T: Clone + Add<Output=T> + Sub<Output=T> + Mul<f64, Output=T>, >(a: T, b: T, factor: f64) -> T {
    a.clone() + (b - a) * factor
}

impl InterpType {
    fn get_factor(&self, a: f64, b: f64, value: f64) -> f64 {
        let range = b - a;
        let progress = value - a;
        return clamp(0., 1., match self {
            InterpType::Linear => {
                progress / range
            }
            InterpType::Exponential(base) => {
                (f64::powf(*base, progress) - 1.) /
                    (f64::powf(*base, range) - 1.)
            }
            InterpType::Cubic(x1, y1, x2, y2) => {
                panic!("Cubic bezier interpolation not yet supported")
            }
        });
    }
}

impl<'de> Deserialize<'de> for InterpType {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de> {
        let mut input: Vec<json::Value> = Deserialize::deserialize(deserializer)?;
        if input.len() < 1 {
            return Err(D::Error::custom("Array too short"));
        }

        let name = input.remove(0);

        return if name.as_str() == Some("linear") {
            Ok(InterpType::Linear)
        } else if name.as_str() == Some("exponential") && input.len() >= 1 {
            let base: StdResult<f64, _> = json::from_value(input.remove(0));

            base.map(|v| InterpType::Exponential(v)).map_err(|_| D::Error::custom("Invalid exponential exponent"))
        } else if name.as_str() == Some("cubic-bezier") && input.len() >= 4 {
            let points: StdResult<Vec<f64>, _> = input.into_iter().map(json::from_value).collect();

            points.map(|p| InterpType::Cubic(p[0], p[1], p[2], p[3])).map_err(|_| D::Error::custom("Invalid bezier control points"))
        } else {
            Err(D::Error::custom("Could not parse expression as interpolation specifier"))
        };
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Stop {
    val: f64,
    out: BaseExpr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Interpolate {
    typ: InterpType,
    input: BaseExpr,
    stops: Vec<Stop>,
}

impl<'de> Deserialize<'de> for Interpolate {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de> {
        let mut arr = parse_basics("interpolate", 2, deserializer)?;

        let spec = json::from_value(arr.remove(0)).map_err(|e| D::Error::custom(format!("Could not parse interpolation spec : {:?}", e)))?;
        let input_expr = json::from_value(arr.remove(0)).map_err(|e| D::Error::custom(format!("Could not parse input expression : {:?}", e)))?;

        let mut iter = arr.into_iter();
        let mut stops = vec![];

        return 'l: loop {
            match (iter.next().map(json::from_value), iter.next().map(json::from_value)) {
                (Some(Ok(k)), Some(Ok(v))) => {
                    stops.push(Stop {
                        val: k,
                        out: v,
                    });
                }
                (None, None) => {
                    break 'l Ok(Interpolate {
                        typ: spec,
                        input: input_expr,
                        stops,
                    });
                }
                (a @ Some(Err(_)), b @ _) | (a @ _, b @ Some(Err(_))) => {
                    break 'l Err(D::Error::custom(format!("Could not parse interpolate arm : input : {:?}, output : {:?}", a, b)));
                }
                _ => {
                    break 'l Err(D::Error::custom("Could not parse expression as \"interpolate\", inner error"));
                }
            }
        };
    }
}

impl Expr for Interpolate {
    fn is_zoom(&self) -> bool {
        self.input.is_zoom() || self.stops.iter().any(|s| s.out.is_zoom())
    }

    fn is_feature(&self) -> bool {
        self.input.is_feature() || self.stops.iter().any(|s| s.out.is_feature())
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

                fn do_lerp(low: &ExprVal, high: &ExprVal, factor: f64) -> ExprResult {
                    if low.get_type() != high.get_type() {
                        return Err(ExprEvalError::custom(format!("Mismatch between interpolate output types: low: {:?} high : {:?} ", low.get_type(), high.get_type())));
                    }

                    Ok(match (low, high) {
                        (ExprVal::Num(a), ExprVal::Num(b)) => ExprVal::Num(lerp(*a, *b, factor)),
                        (ExprVal::Color(a), ExprVal::Color(b)) => ExprVal::Color(lerp(*a, *b, factor)),
                        (ExprVal::String(a), ExprVal::String(b)) => ExprVal::Color(lerp(Color::from_str(a).unwrap(), Color::from_str(b).unwrap(), factor)),
                        (ExprVal::List(a), ExprVal::List(b)) => {
                            let mut res = vec![];
                            if a.len() != b.len() {
                                return Err(ExprEvalError::custom(format!("Mismatch between interpolate array lengths: a: {:?} b : {:?} ", a.len(), b.len())));
                            }

                            for (a, b) in a.iter().zip(b.iter()) {
                                res.push(do_lerp(a, b, factor)?)
                            }
                            ExprVal::List(res)
                        }
                        (a @ _, b @ _) => {
                            return Err(ExprEvalError::custom(format!("Interpolate not supported for type pair: {:?} and {:?}", a.get_type(), b.get_type())));
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

#[derive(Debug, Clone, PartialEq)]
pub struct Step {
    input: BaseExpr,
    default: BaseExpr,
    stops: Vec<Stop>,
}


impl<'de> Deserialize<'de> for Step {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de> {
        let mut arr = parse_basics("step", 2, deserializer)?;

        let input = json::from_value(arr.remove(0)).map_err(|_| D::Error::custom("Invalid input expression"))?;
        let default = json::from_value(arr.remove(0)).map_err(|_| D::Error::custom("invalid default step"))?;

        let mut iter = arr.into_iter();
        let mut stops = vec![];

        return 'l: loop {
            match (iter.next().map(json::from_value), iter.next().map(json::from_value)) {
                (Some(Ok(k)), Some(Ok(v))) => {
                    stops.push(Stop {
                        val: k,
                        out: v,
                    });
                }
                (a @ Some(Err(_)), b @ _) | (a @ _, b @ Some(Err(_))) => {
                    break 'l Err(D::Error::custom(format!("Could not parse step arm : input : {:?}, output : {:?}", a, b)));
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


impl Expr for Step {
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