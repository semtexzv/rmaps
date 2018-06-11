//#![feature(custom_attribute)]
#![feature(slice_patterns)]
#![feature(proc_macro)]
#![feature(proc_macro_mod)]
#![feature(never_type)]
#![feature(associated_type_defaults)]
#![feature(box_syntax)]


#![allow(unused_imports)]
pub extern crate common;
pub extern crate mapbox_tiles;
pub extern crate css_color_parser;

pub extern crate act;
pub extern crate act_codegen;


/*
pub extern crate act;
#[macro_use]
pub extern crate act_codegen;
*/


pub mod prelude;
pub mod map;

use prelude::*;


pub fn init() {
    use common::prelude::*;

    ::std::thread::spawn(move || {
        let sys = actix::System::new("test");
        sys.run();
    });
    //map::storage::actor_impls::setup();
    //map::storage::setup_FileSource();
}
