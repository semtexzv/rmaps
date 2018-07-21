//! Sync actors support
//!
//! Sync actors could be used for cpu bound load. Only one sync actor
//! runs within arbiter's thread. Sync actor process one message at a time.
//! Sync arbiter can start multiple threads with separate instance of actor in
//! each. Note on actor `stopping` lifecycle event, sync actor can not prevent
//! stopping by returning `false` from `stopping` method.
//! Multi consumer queue is used as a communication channel queue.
//! To be able to start sync actor via `SyncArbiter`
//! Actor has to use `SyncContext` as an execution context.
//!
//! ## Example
//!
//! ```rust
//! # extern crate actix;
//! # extern crate futures;
//! use actix::prelude::*;
//!
//! struct Fibonacci(pub u32);
//!
//! impl Message for Fibonacci {
//!     type Result = Result<u64, ()>;
//! }
//!
//! struct SyncActor;
//!
//! impl Actor for SyncActor {
//!     type Context = SyncContext<Self>;
//! }
//!
//! impl Handler<Fibonacci> for SyncActor {
//!     type Result = Result<u64, ()>;
//!
//!     fn handle(&mut self, msg: Fibonacci, _: &mut Self::Context) -> Self::Result {
//!         if msg.0 == 0 {
//!             Err(())
//!         } else if msg.0 == 1 {
//!             Ok(1)
//!         } else {
//!             let mut i = 0;
//!             let mut sum = 0;
//!             let mut last = 0;
//!             let mut curr = 1;
//!             while i < msg.0 - 1 {
//!                 sum = last + curr;
//!                 last = curr;
//!                 curr = sum;
//!                 i += 1;
//!             }
//!             Ok(sum)
//!         }
//!    }
//! }
//!
//! fn main() {
//!     let sys = System::new("test");
//!
//!     // start sync arbiter with 2 threads
//!     let addr = SyncArbiter::start(2, || SyncActor);
//!
//!     // send 5 messages
//!     for n in 5..10 {
//!         addr.do_send(Fibonacci(n));
//!     }
//!
//!     Arbiter::handle().spawn_fn(|| {
//! #        Arbiter::system().do_send(actix::msgs::SystemExit(0));
//!         futures::future::result(Ok(()))
//!     });
//!
//!     sys.run();
//! }
//! ```
use std::marker::PhantomData;
use std::sync::Arc;
use std::{mem, thread};

use crossbeam_channel as channel;
use futures::sync::oneshot::Sender as SyncSender;
use futures::{Async, Future, Poll, Stream};

use actor::{Actor, ActorContext, ActorState, Running};
use address::sync_channel;
use address::{Addr, EnvelopeProxy, Syn, SyncAddressReceiver, SyncEnvelope, ToEnvelope};
use arbiter::Arbiter;
use context::Context;
use handler::{Handler, Message, MessageResponse};

/// Sync arbiter
pub struct SyncArbiter<A>
where
    A: Actor<Context = SyncContext<A>>,
{
    queue: channel::Sender<SyncContextProtocol<A>>,
    msgs: SyncAddressReceiver<A>,
    threads: usize,
}

impl<A> SyncArbiter<A>
where
    A: Actor<Context = SyncContext<A>> + Send,
{
    /// Start new sync arbiter with specified number of worker threads.
    /// Returns address of the started actor.
    pub fn start<F>(threads: usize, factory: F) -> Addr<Syn, A>
    where
        F: Fn() -> A + Send + Sync + 'static,
    {
        let factory = Arc::new(factory);
        let (sender, receiver) = channel::unbounded();

        for _ in 0..threads {
            let f = Arc::clone(&factory);
            let actor_queue = receiver.clone();

            thread::spawn(move || SyncContext::new(f, actor_queue).run());
        }

        let (tx, rx) = sync_channel::channel(0);
        Arbiter::handle().spawn(SyncArbiter {
            queue: sender,
            msgs: rx,
            threads,
        });

        Addr::new(tx)
    }
}

impl<A> Actor for SyncArbiter<A>
where
    A: Actor<Context = SyncContext<A>>,
{
    type Context = Context<Self>;
}

#[doc(hidden)]
impl<A> Future for SyncArbiter<A>
where
    A: Actor<Context = SyncContext<A>>,
{
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            match self.msgs.poll() {
                Ok(Async::Ready(Some(msg))) => self.queue
                    .send(SyncContextProtocol::Envelope(msg))
                    .expect("Should not fail"),
                Ok(Async::NotReady) => break,
                Ok(Async::Ready(None)) | Err(_) => unreachable!(),
            }
        }

        // stop condition
        if self.msgs.connected() {
            Ok(Async::NotReady)
        } else {
            // stop sync arbiters
            for _ in 0..self.threads {
                let _ = self.queue.send(SyncContextProtocol::Stop);
            }
            Ok(Async::Ready(()))
        }
    }
}

impl<A, M> ToEnvelope<Syn, A, M> for SyncContext<A>
where
    A: Actor<Context = SyncContext<A>> + Handler<M>,
    M: Message + Send + 'static,
    M::Result: Send,
{
    fn pack(msg: M, tx: Option<SyncSender<M::Result>>) -> SyncEnvelope<A> {
        SyncEnvelope::with_proxy(Box::new(SyncContextEnvelope::new(msg, tx)))
    }
}

enum SyncContextProtocol<A>
where
    A: Actor<Context = SyncContext<A>>,
{
    Stop,
    Envelope(SyncEnvelope<A>),
}

/// Sync actor execution context
pub struct SyncContext<A>
where
    A: Actor<Context = SyncContext<A>>,
{
    act: A,
    queue: channel::Receiver<SyncContextProtocol<A>>,
    stopping: bool,
    state: ActorState,
    factory: Arc<Fn() -> A>,
}

impl<A> SyncContext<A>
where
    A: Actor<Context = Self>,
{
    /// Create new SyncContext
    fn new(
        factory: Arc<Fn() -> A>,
        queue: channel::Receiver<SyncContextProtocol<A>>,
    ) -> Self {
        let act = factory();
        SyncContext {
            act,
            queue,
            factory,
            stopping: false,
            state: ActorState::Started,
        }
    }

    fn run(&mut self) {
        let ctx: &mut SyncContext<A> =
            unsafe { mem::transmute(self as &mut SyncContext<A>) };

        // started
        A::started(&mut self.act, ctx);
        self.state = ActorState::Running;

        loop {
            match self.queue.recv() {
                Ok(SyncContextProtocol::Stop) => {
                    self.state = ActorState::Stopping;
                    if A::stopping(&mut self.act, ctx) != Running::Stop {
                        warn!("stopping method is not supported for sync actors");
                    }
                    self.state = ActorState::Stopped;
                    A::stopped(&mut self.act, ctx);
                    return;
                }
                Ok(SyncContextProtocol::Envelope(mut env)) => {
                    env.handle(&mut self.act, ctx);
                }
                Err(_) => {
                    self.state = ActorState::Stopping;
                    if A::stopping(&mut self.act, ctx) != Running::Stop {
                        warn!("stopping method is not supported for sync actors");
                    }
                    self.state = ActorState::Stopped;
                    A::stopped(&mut self.act, ctx);
                    return;
                }
            }

            if self.stopping {
                self.stopping = false;

                // stop old actor
                A::stopping(&mut self.act, ctx);
                self.state = ActorState::Stopped;
                A::stopped(&mut self.act, ctx);

                // start new actor
                self.state = ActorState::Started;
                self.act = (*self.factory)();
                A::started(&mut self.act, ctx);
                self.state = ActorState::Running;
            }
        }
    }
}

impl<A> ActorContext for SyncContext<A>
where
    A: Actor<Context = Self>,
{
    /// Stop current actor. SyncContext creates and starts new actor.
    fn stop(&mut self) {
        self.stopping = true;
        self.state = ActorState::Stopping;
    }

    /// Terminate actor execution. SyncContext creates and starts new actor.
    fn terminate(&mut self) {
        self.stopping = true;
        self.state = ActorState::Stopping;
    }

    /// Actor execution state
    fn state(&self) -> ActorState {
        self.state
    }
}

pub(crate) struct SyncContextEnvelope<A, M>
where
    A: Actor<Context = SyncContext<A>> + Handler<M>,
    M: Message + Send,
{
    msg: Option<M>,
    tx: Option<SyncSender<M::Result>>,
    actor: PhantomData<A>,
}

unsafe impl<A, M> Send for SyncContextEnvelope<A, M>
where
    A: Actor<Context = SyncContext<A>> + Handler<M>,
    M: Message + Send,
{
}

impl<A, M> SyncContextEnvelope<A, M>
where
    A: Actor<Context = SyncContext<A>> + Handler<M>,
    M: Message + Send,
    M::Result: Send,
{
    pub fn new(msg: M, tx: Option<SyncSender<M::Result>>) -> Self {
        SyncContextEnvelope {
            tx,
            msg: Some(msg),
            actor: PhantomData,
        }
    }
}

impl<A, M> EnvelopeProxy for SyncContextEnvelope<A, M>
where
    M: Message + Send + 'static,
    A: Actor<Context = SyncContext<A>> + Handler<M>,
{
    type Actor = A;

    fn handle(&mut self, act: &mut A, ctx: &mut A::Context) {
        let tx = self.tx.take();
        if tx.is_some() && tx.as_ref().unwrap().is_canceled() {
            return;
        }

        if let Some(msg) = self.msg.take() {
            <A as Handler<M>>::handle(act, msg, ctx).handle(ctx, tx)
        }
    }
}
