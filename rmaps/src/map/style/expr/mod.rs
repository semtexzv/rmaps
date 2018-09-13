use prelude::*;

pub use serde::{
    Deserializer,
    Deserialize,
    de::DeserializeOwned,
    de::DeserializeSeed,
    de::Error as DeError,
};

scoped_thread_local!(pub static TYPE: Type);
scoped_thread_local!(pub static NAME: String);

macro_rules! expr {
    ($e:tt) => {Parse::parse(json!($e),Type::Any)};
}

macro_rules! delegate_to_inner {
    (@pat $var:path ; $pat:pat) => {
        $var($pat)
    };
    ($self:expr; [$($variants:path),* ]; $pat:pat => $expr:expr $(; $default:expr )*) => {
        match $self {
            $(delegate_to_inner!{@pat $variants; $pat} => $expr,)*
            $(_ => $default,)*
        }
    };
}


#[macro_use]
pub mod parse;


pub mod expressions;
pub mod util;
pub mod val;
pub mod eval;

pub use self::{
    parse::*,
    util::*,
    val::*,
    eval::*,
    expressions::*,
};


#[derive(Debug, Deserialize, Clone, Copy, Hash, PartialOrd, PartialEq, Ord, Eq)]
pub enum Type {
    #[serde(rename = "null")]
    Null,
    #[serde(rename = "color")]
    Color,
    #[serde(rename = "object")]
    Object,
    #[serde(rename = "array")]
    Array,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "_")]
    Any,
}

impl FromStr for Type {
    type Err = serde::de::value::Error;

    fn from_str(s: &str) -> StdResult<Self, Self::Err> {
        use serde::de::IntoDeserializer;

        let deser: serde::de::value::StrDeserializer<_> = s.into_deserializer();
        Ok(Self::deserialize(deser)?)
    }
}


#[derive(Debug, Clone)]
pub struct Expr(Box<dyn Expression>);

use serde::de::{
    self,
    Visitor,
    SeqAccess,
    MapAccess,
};
use ::std::fmt;


impl<'de> Deserialize<'de> for Expr {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error> where D: Deserializer<'de> {
        struct ExprVisitor;

        impl<'de> Visitor<'de> for ExprVisitor {
            type Value = Box<dyn Expression>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Expression or literal value")
            }

            fn visit_bool<E>(self, v: bool) -> StdResult<Self::Value, E> where
                E: de::Error, {
                Ok(Box::new(ValueVisitor.visit_bool(v)?))
            }


            fn visit_i64<E>(self, v: i64) -> StdResult<Self::Value, E> where E: de::Error, {
                Ok(Box::new(ValueVisitor.visit_i64(v)?))
            }

            fn visit_u64<E>(self, v: u64) -> StdResult<Self::Value, E> where E: de::Error, {
                Ok(Box::new(ValueVisitor.visit_u64(v)?))
            }

            fn visit_f64<E>(self, v: f64) -> StdResult<Self::Value, E> where E: de::Error, {
                Ok(Box::new(ValueVisitor.visit_f64(v)?))
            }

            fn visit_str<E>(self, v: &str) -> StdResult<Self::Value, E> where E: de::Error, {
                Ok(Box::new(ValueVisitor.visit_str(v)?))
            }

            fn visit_none<E>(self) -> StdResult<Self::Value, E> where E: de::Error, {
                Ok(Box::new(ValueVisitor.visit_none()?))
            }


            fn visit_seq<A>(self, mut seq: A) -> StdResult<Self::Value, A::Error> where A: SeqAccess<'de>, {
                let name: String = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;


                fn deser<'a, A: SeqAccess<'a>, T, F>(seq: A, f: F) -> StdResult<Box<dyn Expression>, A::Error>
                    where T: Expression,
                          F: FnOnce(de::value::SeqAccessDeserializer<A>) -> StdResult<T, A::Error> {
                    Ok(Box::new(f(de::value::SeqAccessDeserializer::new(seq))?))
                }
                let expected_type = if TYPE.is_set() { TYPE.with(|t| t.clone()) } else { Type::Any };
                TYPE.set(&expected_type, || {
                    NAME.set(&name, || {
                        match name.deref() {
                            "array" | "boolean" | "collator" |
                            "literal" | "number" | "object" | "string" |
                            "to-color" | "to-number" | "to-string" | "typeof" =>
                                deser(seq, |a| types::Types::deserialize(a)),

                            "feature-state" | "geometry-type" | "id" | "properties" =>
                                deser(seq, |a| feature::FeatureExpr::deserialize(a)),

                            "at" | "get" | "has" | "length" => deser(seq, |a| lookup::Lookup::deserialize(a)),
                            "zoom" => Ok(Box::new(zoom::Zoom {}) as Box<dyn Expression>),

                            "interpolate" => deser(seq, |a| interp::Interpolate::deserialize(a)),
                            "step" => deser(seq, |a| interp::Step::deserialize(a)),

                            "!" | "!=" | "<" | "<=" | "==" | ">" | ">=" | "all" | "any" | "case" | "coalesce" | "match" =>
                                deser(seq, |a| decision::Decision::deserialize(a)),

                            "let" | "var" => deser(seq, |a| variables::Variable::deserialize(a)),

                            "concat" | "downcase" | "is-supported-script" | "resolved-locale" | "upcase" =>
                                deser(seq, |a| string::Str::deserialize(a)),

                            "rgb" | "rgba" | "to-rgba" => deser(seq, |a| color::ColorExpr::deserialize(a)),

                            "-" | "*" | "/" | "%" | "^" | "+" | "abs" | "acos" | "asin" | "atan" | "ceil" | "cos" | "e" |
                            "floor " | "ln" | "ln2" | "log10" | "log2" | "max" | "min" | "pi" | "round" | "sin" | "sqrt" |
                            "tan" => deser(seq, |a| math::Math::deserialize(a)),
                            _ => {
                                return Err(A::Error::custom(format!("{} is not a valid expression identifier", name)));
                            }
                        }
                    })
                })
            }

            /*
            fn visit_map<A>(self, map: A) -> StdResult<Self::Value, A::Error> where A: MapAccess<'de> {
                Ok(Box::new(ValueVisitor.visit_map(map)?))
            }
            */
        }


        Ok(Expr(deserializer.deserialize_any(ExprVisitor)?))
    }
}

impl Expression for Expr {
    fn is_zoom(&self) -> bool {
        let x = self.0.deref().is_zoom();

        x
    }

    fn is_feature(&self) -> bool {
        let f = self.0.deref().is_feature();
        f
    }

    fn eval(&self, ctx: &EvaluationContext) -> eval::ExprResult {
        self.0.deref().eval(ctx)
    }
}

pub trait Expression: Debug + 'static + ExprClone {
    fn is_zoom(&self) -> bool;
    fn is_feature(&self) -> bool;

    fn eval(&self, ctx: &eval::EvaluationContext) -> ExprResult;
}

pub trait ExprClone {
    fn clone_box(&self) -> Box<dyn Expression>;
}


impl<T> ExprClone for T
    where
        T: 'static + Expression + Clone,
{
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn Expression> {
    fn clone(&self) -> Box<dyn Expression> {
        self.clone_box()
    }
}


pub trait DescribeType: Debug + 'static {
    fn describe_type() -> Type;
}

macro_rules! describe_type {
    ($($type:ty)*, $t:expr) => {
        describe_type!{@inner $($type)* , $t}
        describe_type!{@inner $(Option<$type>)* , $t}
        describe_type!{@inner $([$type;2])* , Type::Array}
        describe_type!{@inner $([$type;3])* , Type::Array}
        describe_type!{@inner $([$type;4])* , Type::Array}
        describe_type!{@inner $(Vec<$type>)* , Type::Array}
    };
    (@inner $($type:ty)*, $t:expr) => {
        $(impl DescribeType for $type {
            fn describe_type() -> Type {
                $t
            }
        })*
    };
}

describe_type!(Color, Type::Color);
describe_type!(i32 u32 isize usize f32 f64, Type::Number);
describe_type!(String, Type::String);
describe_type!(bool, Type::Boolean);

/// Utility structs that passes expected type into `Deserialize` implementation of `BaseExpr`, through
/// scoped thread local variable
#[derive(Debug, Clone)]
pub struct TypedExpr<T: DescribeType>(pub Expr, pub PhantomData<T>);

impl<T: DescribeType> TypedExpr<T> {
    pub fn new(e: Expr) -> Self {
        TypedExpr(e, PhantomData)
    }
}

impl<T: DescribeType> TypedExpr<T> {
    #[inline]
    pub fn is_zoom(&self) -> bool {
        self.0.is_zoom()
    }
    #[inline]
    pub fn is_feature(&self) -> bool {
        self.0.is_feature()
    }
}

impl<'de, T: DescribeType> Deserialize<'de> for TypedExpr<T> {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error> where
        D: Deserializer<'de> {
        TYPE.set(&T::describe_type(), || {
            Ok(TypedExpr(Deserialize::deserialize(deserializer)?, ::std::marker::PhantomData))
        })
    }
}

impl<T: DescribeType + Clone> Expression for TypedExpr<T> {
    fn is_zoom(&self) -> bool {
        self.0.is_zoom()
    }
    #[inline(never)]
    fn is_feature(&self) -> bool {
        self.0.is_feature()
    }
    fn eval(&self, ctx: &EvaluationContext) -> ExprResult {
        self.0.eval(ctx)
    }
}
