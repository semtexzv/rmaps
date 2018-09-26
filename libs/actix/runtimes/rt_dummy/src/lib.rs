extern crate futures;

use futures::prelude::*;

pub struct Runtime {

}

impl Runtime {
    pub fn new() -> std::io::Result<Runtime> {
        unimplemented!()
    }
    pub fn block_on<F>(&mut self, f: F) -> Result<F::Item, F::Error>
        where F: Future
    {
        unimplemented!()
    }
    pub fn spawn<F>(&mut self, future: F) -> &mut Self
        where F: Future<Item = (), Error = ()> + 'static,
    {
        unimplemented!()
    }

}