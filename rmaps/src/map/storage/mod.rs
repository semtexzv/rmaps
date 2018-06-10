use prelude::*;

pub mod resource;
pub mod response;

pub mod local;

pub use self::resource::*;
pub use self::response::Response as ResResponse;

pub trait Source {
    // TODO, async loading, Callbacks or messages,
    // Resource -> Response

    fn can_handle(&self, url: &str) -> bool;
}

#[derive(Actor)]
pub struct FileSource {
    sources: Vec<Box<Source + Send + 'static>>
}

impl FileSource {
    pub fn new() -> Self {
        FileSource {
            sources: vec![
                Box::new(local::LocalFileSource::new())
            ]
        }
    }
}

#[actor_impl]
impl FileSource{
    fn handle(&mut self, res : Resource) -> Result<response::Response> {
        panic!("A")
    }
}