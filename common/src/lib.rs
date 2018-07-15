#![feature(use_extern_macros)]
#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_parens, unused_imports)]


#[macro_use]
pub extern crate gl;
pub extern crate glium;
pub extern crate glium_derive;
pub extern crate cgmath;

pub extern crate num;

pub extern crate rayon;

pub extern crate regex;

#[macro_use]
pub extern crate log;

extern crate fern;
pub extern crate chrono;
pub extern crate time;


pub extern crate failure;
pub extern crate itertools;

pub extern crate serde;
pub extern crate serde_derive;
#[macro_use]
pub extern crate serde_json as json;

#[macro_use]
pub extern crate scoped_tls;

pub extern crate palette;

pub extern crate actix;
pub extern crate actix_web;

pub extern crate futures;
pub extern crate tokio;

pub extern crate url;
pub extern crate uuid;
pub extern crate enumflags;
pub extern crate enumflags_derive;
pub extern crate css_color_parser;
#[macro_use]
pub extern crate derive_more;


pub mod prelude;
pub mod export;
pub mod color;
pub mod util;
pub mod coord;
pub mod mercator;

pub mod geometry;

pub fn init_log() {
    use fern::colors::{Color, ColoredLevelConfig};

    let mut e = ::std::fs::create_dir_all("./logs/");

    let normal_filter = log::LevelFilter::Trace;
    let other_filter = log::LevelFilter::Off;


    let mut colors = fern::colors::ColoredLevelConfig::new()
        .debug(Color::Green)
        .info(Color::Blue)
        .warn(Color::Yellow)
        .error(Color::Red);

    fern::Dispatch::new()
        .format(move |out, msg, data| {
            out.finish(format_args!(
                "{} {} - {} {}",
                chrono::Local::now().format("%H:%M:%S"),
                data.target(),
                colors.color(data.level()),
                msg
            ))
        })
        .level(normal_filter)


        .level_for("tokio_io", other_filter)
        .level_for("tokio_reactor", other_filter)
        .level_for("tokio_core", other_filter)
        .level_for("tokio_threadpool", other_filter)
        .level_for("mio", other_filter)
        .level_for("rocket", other_filter)
        .level_for("actix", other_filter)
        .level_for("actix_web", other_filter)
        .level_for("trust_dns_proto", other_filter)
        .level_for("launch", other_filter)
        .level_for("hyper", other_filter)
        .level_for("winit", other_filter)
        .level_for("_", other_filter)
        .chain(::std::io::stdout())
        .apply().unwrap();
}