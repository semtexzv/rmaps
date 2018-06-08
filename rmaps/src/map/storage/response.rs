use prelude::*;

pub struct Response {
    resource: super::resource::Resource,
    data: Vec<u8>,
}