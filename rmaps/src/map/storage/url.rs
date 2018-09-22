use ::prelude::*;

const ACCESS_TOKEN: &str = "pk.eyJ1Ijoic2VtdGV4enYiLCJhIjoiY2luc3hvamlvMDBsdnZza2wybndkZmY1bCJ9.6p1bjo8wrzd5jmp9KdinwQ";

const BASE: &str = "https://api.mapbox.com";

pub fn is_mapbox_url(url: &str) -> bool {
    return url.starts_with("mapbox://");
}

use super::Request;
use ::common::regex::{
    Regex, CaptureNames, Captures, Match, Matches,
};


/*

STYLE:
    mapbox://styles/semtexzv/cjjjv418k6m0b2rok0oiejd4i
=>  https://api.mapbox.com/styles/v1/semtexzv/cjjjv418k6m0b2rok0oiejd4i?access_token={TOKEN}

Sprites:
    mapbox://sprites/semtexzv/cjjjv418k6m0b2rok0oiejd4i
=>  https://api.mapbox.com/styles/v1/semtexzv/cjjjv418k6m0b2rok0oiejd4i/sprite.png?access_token={TOKEN}

Tilejson urls:

    mapbox://mapbox.92olaqdt,mapbox.mapbox-streets-v7
    https://api.mapbox.com/v4/mapbox.92olaqdt,mapbox.mapbox-streets-v7.json?access_token={TOKEN}
=>

*/

pub fn normalize_style(url: &str) -> String {
    let style_re = Regex::new(r#"mapbox://(styles/)(?P<spec>.*)"#).unwrap();
    return style_re.replace(url, |caps: &Captures| {
        let spec = caps.name("spec").map(|x| x.as_str()).unwrap_or("");

        format!("{base}/styles/v1/{spec}?access_token={token}", base = BASE, spec = spec, token = ACCESS_TOKEN)
    }).to_string();
}
pub fn normalize_source(url: &str) -> String {
    let source_re = Regex::new(r#"mapbox://(?P<spec>mapbox.*)"#).unwrap();
    return source_re.replace(url, |caps: &Captures| {
        let spec = caps.name("spec").map(|x| x.as_str()).unwrap_or("");

        format!("{base}/v4/{spec}.json?access_token={token}", base = BASE, spec = spec, token = ACCESS_TOKEN)
    }).to_string();
}


pub fn normalize_sprite(url: &str, filetype: &str) -> String {
    let sprite_re = Regex::new(r#"mapbox://sprites/(?P<username>[a-zA-Z0-9]+)/(?P<id>[a-zA-Z0-9]+)"#).unwrap();
    return sprite_re.replace(url, |caps: &Captures| {
        let username = caps.name("username").map(|x| x.as_str()).unwrap_or("_");
        let id = caps.name("id").map(|x| x.as_str()).unwrap_or("_");

        format!("{base}/styles/v1/{user}/{id}/sprite.{filetype}?access_token={token}",
                base = BASE,
                user = username,
                id = id,
                filetype = filetype,
                token = ACCESS_TOKEN)
    }).to_string();
}




pub fn normailize_glyph(url: &str) -> String {
    let glyph_re = Regex::new(r#"mapbox://fonts/(?P<username>[[:alpha:]]+)/(?P<rest>.*)"#).unwrap();
    return "".into();
}

