use prelude::*;

use super::*;

#[derive(Actor)]
pub struct LocalFileSource {

}

impl LocalFileSource {
    pub fn new() -> LocalFileSource {
        LocalFileSource{

        }
    }
    pub fn spawn() -> LocalFileSourceAddr {
        LocalFileSourceAddr {
            addr: start_in_thread(|| Self::new())
        }
    }
}
#[actor_impl]
impl FileSource for LocalFileSource{
    fn can_handle(&self, res : Resource) -> bool {
        return true;
       // return url.starts_with("local://") || url.starts_with("file://");
    }

    fn get(&mut self, res: Resource, accept_addr: Box<FileAcceptorAddr + Send + 'static>) {

    }
}
#[actor_impl]
impl LocalFileSource{
    fn test(&mut self) -> usize {
        0
    }
}

