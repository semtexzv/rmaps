//#![feature(custom_attribute)]
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
pub extern crate mvt;
#[macro_use]
pub extern crate rmaps_derive;
pub extern crate geometry;
pub extern crate image;

pub extern crate tess2;
#[macro_use]
pub extern crate lazy_static;


extern crate serde;


pub extern crate imgui;
pub extern crate imgui_glium_renderer;


pub mod prelude;
pub mod map;

use prelude::*;
