extern crate futures;
extern crate tokio;

use futures::prelude::*;

pub struct Runtime {
    rt: tokio::runtime::current_thread::Runtime,
}

impl Runtime {
    pub fn new() -> std::io::Result<Runtime> {
        Ok(Runtime {
            rt: tokio::runtime::current_thread::Runtime::new()?
        })
    }
    pub fn block_on<F>(&mut self, f: F) -> Result<F::Item, F::Error>
        where F: Future
    {
        self.rt.block_on(f)
    }
    pub fn spawn<F>(&mut self, future: F) -> &mut Self
        where F: Future<Item=(), Error=()> + 'static
    {
        self.rt.spawn(future);
        self
    }
}