use prelude::*;

use super::*;
use std::io::Read;

#[derive(Actor)]
pub struct LocalFileSource {}

impl LocalFileSource {
    pub fn new() -> LocalFileSource {
        LocalFileSource {}
    }
    pub fn spawn() -> LocalFileSourceAddr {
        LocalFileSourceAddr {
            addr: start_in_thread(|| Self::new())
        }
    }
    /*
    fn get_int(&mut self, res: Resource, accept_addr: Box<FileAcceptorAddr + Send + 'static>) -> Result<()> {
        let url = {
            res.url().to_string()
        };
        let pos = url.find("://");
        let path = url.split_at(pos.unwrap()+3).1;
//        let path = format!("{}{}",url.domain(),url.path());
        println!("Local  Retrieving  {:?}", path);
        let mut f = ::std::fs::File::open(path)?;
        let mut data = vec![];
        f.read_to_end(&mut data)?;
        let resp = ResResponse {
            resource: res,
            data,
        };
        accept_addr.resource_ready_async(resp);
        Ok(())
    }
    */
}


#[actor_impl]
impl FileSource for LocalFileSource {
    fn can_handle(&self, res: Resource) -> bool {
        println!("Local can handle {:?}", res.url());
        return res.url().starts_with("local://") || res.url().starts_with("file://");
    }

    fn get(&mut self, res: Resource) -> ResponseFuture<ResResponse,Error> {
        Box::new(unimplemented!())

    }
}
