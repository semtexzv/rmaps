use ::prelude::*;
use super::{
    Type,
    eval::*,
    val::*,
};


pub fn expect_type(t: Type, v: Value) -> ExprResult {
    return if v.get_type() == t {
        Ok(v)
    } else {
        Err(EvalError::InvalidType {
            expected: t,
            got: v.get_type(),
        })
    };
}

pub fn expect_num(v: Value) -> StdResult<f64, EvalError> {
    return if let Value::Num(n) = v {
        Ok(n)
    } else {
        Err(EvalError::InvalidType {
            expected: Type::Number,
            got: v.get_type(),
        })
    };
}

pub fn expect_bool(v: Value) -> StdResult<bool, EvalError> {
    return if let Value::Bool(n) = v {
        Ok(n)
    } else {
        Err(EvalError::InvalidType {
            expected: Type::Boolean,
            got: v.get_type(),
        })
    };
}

pub fn expect_color(v: Value) -> StdResult<Color, EvalError> {
    return if let Value::Color(n) = v {
        Ok(n)
    } else {
        Err(EvalError::InvalidType {
            expected: Type::Color,
            got: v.get_type(),
        })
    };
}

pub fn expect_len<T>(a: &[T], count: usize) -> StdResult<(), EvalError> {
    if a.len() >= count {
        return Ok(());
    } else {
        return Err(EvalError::InvalidNumberOfArguments {
            got: a.len(),
            expected: count,
        });
    }
}


use std::fmt;
use std::marker::PhantomData;

use serde::de::{self, Deserialize, Deserializer, SeqAccess, Visitor};

#[derive(Debug, Clone, )]
pub struct Prefix<T, R>(T, R);

impl<'de, T, R> Deserialize<'de> for Prefix<T, R>
    where
        T: Deserialize<'de>,
        R: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
        where D: Deserializer<'de> {
        struct PrefixVisitor<T, R>(PhantomData<(T, R)>);

        impl<'de, T, R> Visitor<'de> for PrefixVisitor<T, R>
            where T: Deserialize<'de>, R: Deserialize<'de> {
            type Value = Prefix<T, R>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("sequence")
            }

            fn visit_seq<A>(self, mut seq: A) -> StdResult<Self::Value, A::Error>
                where A: SeqAccess<'de>,
            {
                let t = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let r = R::deserialize(de::value::SeqAccessDeserializer::new(seq))?;
                Ok(Prefix(t, r))
            }
        }

        deserializer.deserialize_seq(PrefixVisitor(PhantomData))
    }
}
