use prelude::*;
use super::*;

use std::cell::RefCell;

pub struct EvaluationContext<'a> {
    pub zoom: Option<f32>,
    pub feature_data: Option<&'a ::mvt::Feature>,
    pub bindings: RefCell<BTreeMap<String, Expr>>,
}

#[derive(Debug)]
pub enum EvalError {
    InvalidType {
        expected: Type,
        got: Type,
    },
    InvalidNumberOfArguments {
        expected: usize,
        got: usize,
    },
    Custom(String),
}

impl EvalError {
    pub fn invalid_type(expected: Type, got: Type) -> EvalError {
        EvalError::InvalidType {
            expected,
            got,
        }
    }
    pub fn invalid_arguments(expected: usize, got: usize) -> EvalError {
        EvalError::InvalidNumberOfArguments {
            expected,
            got,
        }
    }
    pub fn custom(m: impl Into<String>) -> EvalError {
        EvalError::Custom(m.into())
    }
}

pub type ExprResult = StdResult<Value, EvalError>;
