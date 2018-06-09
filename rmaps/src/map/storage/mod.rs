pub mod resource;
pub mod response;

pub use self::resource::*;

pub trait Source {
    // TODO, async loading, Callbacks or messages,
    // Resource -> Response
}

use act;
use ::act::Actor;
use ::act_codegen::{actor_impls, derive_actor_trait, Actor};

#[derive(Actor)]
pub struct FileSource {
    handle: FileSourceHandle,
    inbox: ::act::Inbox,
}

impl FileSource {
    pub fn new() -> Self {
        let (inbox, handle) = FileSourceHandle::new_inbox_pair();
        FileSource {
            handle,
            inbox,
        }
    }
}

#[derive_actor_trait]
pub trait ResourceAcceptor {
    fn accept(&mut self, data: response::Response);
}


#[actor_impls(FileSource)]
pub mod actor_impls {
    use super::*;

    impl FileSource {
        pub fn get(&mut self, accept: ResourceAcceptorHandle, res: resource::Resource) {

        }
        pub fn test(&mut self) {
            panic!("Processing message");
        }
    }

    impl ResourceAcceptor for FileSource {
        fn accept(&mut self, data: response::Response) {
            let mut h:  ResourceAcceptorHandle = self.handle().clone().into();
            self.handle.get(h,unimplemented!());
        }
    }
}