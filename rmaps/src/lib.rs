//#![feature(custom_attribute)]
#![feature(specialization)]
#![recursion_limit="512"]
#![feature(slice_patterns)]
#![feature(never_type)]
#![feature(pattern_parentheses)]
#![feature(associated_type_defaults)]
#![feature(box_syntax)]

#![feature(try_from)]
#![feature(trace_macros)]
#![feature(macro_at_most_once_rep)]
#![feature(nll)]

#![allow(unused_imports, dead_code, unused_mut, unused_variables, unused_macros, unreachable_code, unreachable_patterns, unused_parens)]
#[macro_use]
pub extern crate common;

#[macro_use]
pub extern crate rmaps_derive;
#[macro_use]
pub extern crate lazy_static;

extern crate serde;


pub mod prelude;
pub mod map;

use prelude::*;
