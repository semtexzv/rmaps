extern crate futures;

pub extern crate stdweb;

use futures::Future;

pub fn spawn<F>(future: F)
    where F: Future<Item=(), Error=()> + 'static
{
    stdweb::PromiseFuture::spawn(future);
}

pub struct Runtime {}

type LtFuture<'a> = Box<Future<Item=(), Error=()> + 'a>;
type BoxedFuture = Box<Future<Item=(), Error=()> + 'static>;


impl Runtime {
    pub fn new() -> std::io::Result<Runtime> {
        Ok(Runtime {})
    }
    pub fn block_on<F>(&mut self, f: F) -> Result<F::Item, F::Error>
        where F: Future
    {
        use std::{
            mem,
            sync::mpsc::*,
        };

        let (tx, rx) = channel();

        let fut = f.then(move |v| {
            tx.send(v).unwrap();
            futures::finished::<(), ()>(())
        });

        let time: LtFuture = Box::new(fut);


        unsafe {
            let act = mem::transmute::<LtFuture, BoxedFuture>(time);
            stdweb::PromiseFuture::spawn(act);

            let data = 'l: loop {
                stdweb::webcore::executor::EventLoop.drain();
                match rx.try_recv() {
                    Ok(o) => {
                        break 'l o;
                    }
                    Err(e) => {
                        println!("Not yet finished: {:?}", e);
                    }
                }
            };

            return data;
        }
    }

    pub fn spawn<F>(&mut self, future: F) -> &mut Self
        where F: Future<Item=(), Error=()> + 'static,
    {
        stdweb::PromiseFuture::spawn(future);
        self
    }
}