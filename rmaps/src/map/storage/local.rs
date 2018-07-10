use prelude::*;

use super::*;
use std::io::Read;

use common::actix_derive::*;

pub struct LocalFileSource {}
impl Actor for LocalFileSource{
    type Context = Context<Self>;
}

impl Handler<super::ResourceRequest> for LocalFileSource {
    type Result = ();

    fn handle(&mut self, msg: ResourceRequest, _ctx: &mut Context<Self>) {
        let req = &msg.0;
        let url = {
            req.url().to_string()
        };
        let pos = url.find("://");
        let path = url.split_at(pos.unwrap()+3).1;
//        let path = format!("{}{}",url.domain(),url.path());
        println!("Local  Retrieving  {:?}", path);
        let mut f = ::std::fs::File::open(path).unwrap();
        let mut data = vec![];
        f.read_to_end(&mut data).unwrap();
        let resp = Resource {
            req:req.clone(),
            data,
        };
        msg.1.do_send(super::ResourceCallback(Ok(resp))).unwrap();
    }
}
impl LocalFileSource {
    pub fn new() -> LocalFileSource {
        LocalFileSource {}
    }
    pub fn spawn() -> Addr<Syn,Self> {
        start_in_thread(|| Self::new())
    }
}

