#![feature(associated_type_defaults)]
#![feature(specialization)]

extern crate futures;
extern crate actix;

use futures::prelude::*;

/*
pub struct Proxy<I: 'static, E: From<::futures::sync::oneshot::Canceled> + 'static>(::futures::sync::oneshot::Receiver<Result<I, E>>);

impl<I, E: From<::futures::sync::oneshot::Canceled>> ::futures::Future for Proxy<I, E> {
    type Item = I;
    type Error = E;

    fn poll(&mut self) -> Result<Async<I>, E> {
        return match self.0.poll() {
            Ok(Async::Ready(Ok(it))) => {
                Ok(Async::Ready(it))
            }
            Ok(Async::Ready(Err(e))) => {
                Err(e)
            }
            Ok(Async::NotReady) => {
                Ok(Async::NotReady)
            }
            Err(e) => {
                Err(e.into())
            }
        };
    }
}

impl<I: 'static, E: From<::futures::sync::oneshot::Canceled> + 'static> Proxy<I, E> {
    fn new() -> (::futures::sync::oneshot::Sender<Result<I, E>>, Proxy<I, E>) {
        let (tx, rx) = ::futures::sync::oneshot::channel();
        (tx, Proxy(rx))
    }
}

use actix::prelude::*;
use actix::dev::*;

impl<A, E, I, M> MessageResponse<A, M> for Proxy<I, E>
    where A: Actor,
          M: Message<Result=Proxy<I, E>> + 'static,
          E: From<::futures::sync::oneshot::Canceled> + 'static
{
    fn handle<R: ResponseChannel<M>>(self, ctx: &mut <A as Actor>::Context, tx: Option<R>) {
        /* ctx.spawn(self.then(move |res| {
             if let Some(tx) = tx {
                 tx.send(res);
             }
             fut::ok(())
         }));
         */

        if let Some(tx) = tx {
            tx.send(self);
        }
    }
}
*/

use futures::sync::oneshot::*;
use std::cell::RefCell;

use std::rc::Rc;


unsafe impl<I: 'static, E: From<::futures::sync::oneshot::Canceled> + 'static> Send for ProxyLocal<I, E> {}


pub struct ProxyLocal<I: 'static, E: From<::futures::sync::oneshot::Canceled> + 'static> {
    local: Option<Box<Future<Item=I, Error=E>>>,
    sender: Option<Sender<Result<I, E>>>,
    recvr: Receiver<Result<I, E>>,
}

impl<I: 'static, E: From<::futures::sync::oneshot::Canceled> + 'static> ProxyLocal<I, E> {
    pub fn new<F: Future<Item=I, Error=E> + 'static>(f: F) -> Self {
        let (tx, rx) = ::futures::sync::oneshot::channel();
        ProxyLocal {
            local: Some(Box::new(f)),
            sender: Some(tx),
            recvr: rx,
        }
    }
}


impl<I, E: From<::futures::sync::oneshot::Canceled>> ::futures::Future for ProxyLocal<I, E> {
    type Item = I;
    type Error = E;

    fn poll(&mut self) -> Result<Async<I>, E> {
        return match self.recvr.poll() {
            Ok(Async::Ready(Ok(it))) => {
                Ok(Async::Ready(it))
            }
            Ok(Async::Ready(Err(e))) => {
                Err(e)
            }
            Ok(Async::NotReady) => {
                Ok(Async::NotReady)
            }
            Err(e) => {
                Err(e.into())
            }
        };
    }
}


use actix::prelude::*;
use actix::dev::*;

impl<A, E, I, M> MessageResponse<A, M> for ProxyLocal<I, E>
    where A: Actor<Context=Context<A>>,
          M: Message<Result=ProxyLocal<I, E>> + 'static,
          E: From<::futures::sync::oneshot::Canceled> + 'static
{
    fn handle<R: ResponseChannel<M>>(mut self, ctx: &mut <A as Actor>::Context, tx: Option<R>) {
        let fut = self.local.take().unwrap();
        let sender = self.sender.take().unwrap();

        Arbiter::handle().spawn(
            fut.then(move |res| {
                sender.send(res);
                Ok(())
            })
        );

        if let Some(tx) = tx {
            tx.send(self);
        }
    }
}