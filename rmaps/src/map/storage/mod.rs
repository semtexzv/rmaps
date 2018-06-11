use prelude::*;

pub mod resource;
pub mod response;

//pub mod local;
pub mod network;

pub use self::resource::*;
pub use self::response::Response as ResResponse;

#[derive_actor_trait]
pub trait FileSource {
    fn can_handle(&self, res: Resource) -> bool;
    fn get(&mut self, res: Resource) ->::act::ProxyLocal<ResResponse, Error>;
}

#[derive(Actor)]
pub struct DefaultFileSource {
    sources: Vec<Box<FileSourceAddr + Send + 'static>>
}

#[actor_impl]
impl FileSource for DefaultFileSource {
    fn can_handle(&self, res: Resource) -> bool {
        return true;
    }

    fn get(&mut self, res: Resource) -> ::act::ProxyLocal<ResResponse, Error> {
        for a in self.sources.iter() {
            println!("Checking URL compatibility");
            if a.can_handle(res.clone()).wait().unwrap() {
                return ProxyLocal::new(a.get(res).flatten());
            }
        }
        panic!()
    }

}

//#[actor_impl]
/*
impl DefaultFileSource {
    fn get(&mut self, res: Resource) -> Box<Future<Item=ResResponse, Error=Error>> {
        for a in self.sources.iter() {
            println!("Checking URL compatibility");
            if a.can_handle(res.clone()).wait().unwrap() {
                let outer: Box<Future<Item=_, Error=MailboxError>> = a.get(res);
                return Box::new(outer.flatten());
            }
        }
        unimplemented!()
    }
}
*/

impl DefaultFileSource {
    pub fn new() -> Self {
        DefaultFileSource {
            sources: vec![
                //  Box::new(local::LocalFileSource::spawn()),
                 Box::new(network::NetworkFileSource::spawn()),
            ]
        }
    }

    pub fn spawn() -> DefaultFileSourceAddr {
        DefaultFileSourceAddr {
            addr: start_in_thread::<DefaultFileSource, _>(|| DefaultFileSource::new())
        }
    }
}

