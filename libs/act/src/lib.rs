pub extern crate futures;
use futures::prelude::*;

pub struct Wrap<F: futures::Future>( pub F);

impl <F : Future> ::futures::Future for  Wrap<F>{
    type Item = F::Item;
    type Error = F::Error;

    fn poll(&mut self) -> Result<Async<<Self as Future>::Item>, <Self as Future>::Error> {
        self.0.poll()
    }
}