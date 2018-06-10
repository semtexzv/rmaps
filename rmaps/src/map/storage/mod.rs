use prelude::*;

pub mod resource;
pub mod response;

pub mod local;

pub use self::resource::*;
pub use self::response::Response as ResResponse;

#[derive_actor_trait]
pub trait FileAcceptor {
    fn resource_ready(&mut self, res: ResResponse);
}

#[derive_actor_trait]
pub trait FileSource {
    fn can_handle(&self, res: Resource) -> bool;
    fn get(&mut self, res: Resource, accept_addr: Box<FileAcceptorAddr + Send + 'static>);
}

#[derive(Actor)]
pub struct DefaultFileSource {
    sources: Vec<Box<FileSourceAddr + Send + 'static>>
}

impl DefaultFileSource {
    pub fn new() -> Self {
        let b: Box<FileSourceAddr + Send + 'static> = Box::new(local::LocalFileSource::spawn());
        let bb: Box<FileSourceAddr + Send + 'static> = Box::new(local::LocalFileSource::spawn());

        DefaultFileSource {
            sources: vec![
                b,
                bb
            ]
        }
    }

    pub fn spawn() -> DefaultFileSourceAddr {
        DefaultFileSourceAddr {
            addr: start_in_thread::<DefaultFileSource,_>(|| DefaultFileSource::new() )
        }
    }
}


