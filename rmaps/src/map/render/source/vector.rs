use ::prelude::*;

use super::Source;

use map::style::*;
use map::storage::*;

pub struct VectorSource {
    data: StyleSource,
    file_source: Addr<DefaultFileSource>,
}
/*
impl Source for VectorSource {

}*/