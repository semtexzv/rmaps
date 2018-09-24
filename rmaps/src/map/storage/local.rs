use prelude::*;

use super::*;
use std::io::Read;

use super::{Request, Resource, ResourceError};

pub struct LocalFileSource {}

impl Actor for LocalFileSource {
    type Context = Context<Self>;
}

impl Handler<Request> for LocalFileSource {
    type Result = StdResult<Resource, ResourceError>;

    fn handle(&mut self, mut msg: Request, _ctx: &mut Context<Self>) -> Self::Result {
        trace!("LocalFileSource: Processing : {:?}", msg);
        let req = &msg;
        let url = {
            req.url().to_string()
        };
        let pos = url.find("://");
        let path = url.split_at(pos.unwrap() + 3).1;

        let mut f = ::std::fs::File::open(path).map_err(|e| ResourceError::Other(e.into()))?;
        let mut data = vec![];
        f.read_to_end(&mut data).map_err(|e| ResourceError::Other(e.into()))?;
        let resource = Resource {
            req: req.clone(),
            cache_until: 0,
            data,
        };
        return Ok(resource);
    }
}

impl LocalFileSource {
    pub fn new() -> LocalFileSource {
        LocalFileSource {}
    }
    pub fn spawn() -> Addr<Self> {
        start_in_thread(|| Self::new())
    }
}

