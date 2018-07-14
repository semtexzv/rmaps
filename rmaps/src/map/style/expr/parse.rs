use ::prelude::*;
use super::*;

#[derive(Debug)]
pub enum ParseError {
    NotThis,
    Other(Error),
}

impl ParseError {
    fn not_this() -> Self {
        ParseError::NotThis
    }
    fn other(e: Error) -> Self {
        ParseError::Other(e)
    }
}

impl<E: Into<::common::failure::Error>> From<E> for ParseError {
    fn from(v: E) -> Self {
        ParseError::Other(v.into())
    }
}

pub type ParseResult<T> = StdResult<T, ParseError>;

pub trait Parse: Sized {
    fn parse(value: json::Value, expected: Type) -> StdResult<Self, ParseError>;
}

pub fn parse_val<T: Parse>(val: json::Value) -> ParseResult<T> {
    parse_val_expect(val, Type::Any)
}

pub fn parse_val_expect<T: Parse>(val: json::Value, expected: Type) -> ParseResult<T> {
    T::parse(val, expected)
}


pub struct First<H: Parse>(pub H, pub json::Value);

impl<H: Parse> First<H> {
    fn inner(self) -> (H, json::Value) {
        return (self.0, self.1);
    }
}

impl<H: Parse> Parse for First<H> {
    fn parse(value: json::Value, expected: Type) -> ParseResult<Self> {
        let mut arr: Vec<json::Value> = json::from_value(value).map_err(|e| ParseError::Other(e.into()))?;
        if arr.len() == 0 {
            return Err(ParseError::Other(format_err!("Array to short")));
        }

        let head = parse_val(arr.remove(0))?;

        return Ok(First(head, json::Value::Array(arr)));
    }
}

macro_rules! derive_parse {
    ($($t:ty)*) => {
        $(impl Parse for $t {
            fn parse(value: json::Value, expected: Type) -> ParseResult<Self> {
                Ok(json::from_value(value).map_err(|e| ParseError::Other(e.into()))?)
            }
        })*
    };
}

derive_parse!(String bool u8 u16 u32 u64 usize i8 i16 i32 i64 isize);
derive_parse!(Type);
derive_parse!(json::Value);


macro_rules! parse {
    ($($tt:tt)*) => {};
}
/*
macro_rules! parse {
    (@exp as $custom:expr ) => {
        $custom
    };
    (@exp ) => {
        Type::Any
    };
    (@branch $val:expr ; ; $($last_arg:ident : $last_type:ty $( as $last_exp:expr)?)?=> $block:block ) => {
         $(let mut rest: Vec<json::Value> = json::from_value($val)?;
         let mut $last_arg: Vec<_> = rest.into_iter().map(|v| parse_val_expect(v, parse!{@exp $(as $last_exp)?})).collect::<ParseResult<Vec<_>>>()?;)?
         return { $block };
    };
    (@branch $val:expr; $arg:ident : $type:ty $( as $exp:expr )? $(, $more_arg:ident : $more_type:ty $( as $more_exp:expr )?  )* ; $($last_arg:ident : $last_type:ty $( as $last_exp:expr)?)? => $block:block ) => {
        if let Ok(First($arg, mut value)) = First::<$type>::parse($val,parse!{@exp $( as $exp)?}) {
            parse!{@branch value; $($more_arg : $more_type $( as $more_exp)?),* ; $($last_arg : $last_type $(as $last_exp)?)? => $block }
        }
    };
    (@inner $name:expr; $val:expr; $($variant:expr $(,$arg:ident : $type:ty $( as $exp:expr )? )*  $(, ... $last_arg:ident : $last_type:ty $( as $last_exp:expr )?)?   => $block:block)*$(,)*) => {
        $(if $name == $variant {
            parse!{@branch $val; $($arg : $type $(as $exp)?),* ; $($last_arg : $last_type $(as $last_exp)?)? => $block }
        })*

    };
    ($type:ty as $expect:ident ; $($rest:tt)*) => {
        impl Parse for $type {
            fn parse(value : json::Value, expected: Type) -> ParseResult<Self> {
                let $expect = expected;
                // If this parse fails, that means that this is not an expression
                let First(name, mut value) = First::<String>::parse(value, Type::String).map_err(|_| ParseError::NotThis)?;

                parse!{@inner name.deref(); value.clone(); $($rest)*}
                return Err(ParseError::NotThis);
            }
        }
    };
}*/
