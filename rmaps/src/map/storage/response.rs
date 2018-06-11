use prelude::*;

#[derive(Debug)]
pub struct Response {
    pub resource: super::resource::Resource,
    pub data: Vec<u8>,
}