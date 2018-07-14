use ::prelude::*;

const ACCESS_TOKEN : &str = "pk.eyJ1Ijoic2VtdGV4enYiLCJhIjoiY2luc3hvamlvMDBsdnZza2wybndkZmY1bCJ9.6p1bjo8wrzd5jmp9KdinwQ";

const BASE : &str = "https://api.mapbox.com/";

pub fn is_mapbox_url(url : &str) -> bool {
    return url.starts_with("mapbox://")
}
