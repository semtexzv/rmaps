extern crate futures;
extern crate stdweb;
extern crate tokio_current_thread;


use futures::executor;
use futures::future::Future;
use futures::{Stream, Sink, Async};
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use stdweb::web::set_timeout;

pub struct Runtime {

}

impl Runtime {
    pub fn new() -> std::io::Result<Runtime> {
        Ok(Runtime {})
    }
    pub fn block_on<F>(&mut self, f: F) -> Result<F::Item, F::Error>
        where F: Future
    {
        tokio_current_thread::block_on_all(f)
    }
    pub fn spawn<F>(&mut self, future: F) -> &mut Self
        where F: Future<Item = (), Error = ()> + 'static,
    {
        tokio_current_thread::spawn(future);
        self
    }

}