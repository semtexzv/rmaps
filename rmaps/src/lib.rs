//#![feature(custom_attribute)]
#![feature(slice_patterns)]
#![feature(proc_macro)]
#![feature(proc_macro_mod)]

#![allow(unused_imports)]
pub extern crate common;
pub extern crate mapbox_tiles;
pub extern crate css_color_parser;


pub extern crate act;
#[macro_use]
pub extern crate act_codegen;


pub mod prelude;
pub mod map;

use act_codegen::actor_impls;

/*
#[derive(Actor)]
pub struct A1 {
    handle : A1Handle,
    inbox : ::act::Inbox,

}

#[derive(Actor)]
pub struct A2 {
    handle : A2Handle,
    inbox : ::act::Inbox,
}

#[actor_impls]
impl A1 {
    pub fn test(&mut self) {}
    pub fn pong(&mut self, txt: String) {}
}

#[actor_impls]
impl A2 {
    fn ping(&mut self, mut source: A1Handle, txt: String) {
        source.pong(txt)
    }
}
*/

pub fn init() {
    map::storage::actor_impls::setup();
    //map::storage::setup_FileSource();
}
