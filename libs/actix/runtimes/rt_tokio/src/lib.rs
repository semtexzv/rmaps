extern crate futures;
extern crate tokio;
extern crate tokio_current_thread;

use futures::prelude::*;

pub fn spawn<F>(future: F) where F: Future<Item=(), Error=()> + 'static
{
    tokio_current_thread::spawn(future);
}

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