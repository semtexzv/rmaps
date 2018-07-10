use ::prelude::*;
use super::*;

pub struct Array(Option<Type>, Option<usize>, BaseExpr);

pub struct Assert(Type, Vec<BaseExpr>);

pub struct Pass(BaseExpr);
trace_macros!(true);

parse_expr! { Array;
    "array", expr : BaseExpr => {
        Array(None,None,expr)
    }
    "array", typ : Type, val : BaseExpr => {
        Array(Some(typ),None, val)
    }
    "array", typ : Type, len : usize, val : BaseExpr => {
        Array(Some(typ), Some(len), val)
    }
}

trace_macros!(false);
