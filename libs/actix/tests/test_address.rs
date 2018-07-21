#[macro_use]
extern crate actix;
extern crate futures;
extern crate tokio_core;

use actix::prelude::*;
use futures::{future, Future};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio_core::reactor::Timeout;

#[derive(Message, Debug)]
struct Ping(usize);

use std::marker::PhantomData;
use std::rc::Rc;

#[derive(Message, Debug)]
struct UnsyncPing(usize, PhantomData<Rc<bool>>);

struct MyActor(Arc<AtomicUsize>);

impl Actor for MyActor {
    type Context = actix::Context<Self>;
}

impl actix::Handler<Ping> for MyActor {
    type Result = ();

    fn handle(&mut self, _: Ping, _: &mut Self::Context) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

impl actix::Handler<UnsyncPing> for MyActor {
    type Result = ();

    fn handle(&mut self, _: UnsyncPing, _: &mut Self::Context) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

struct MyActor3;

impl Actor for MyActor3 {
    type Context = Context<Self>;
}

impl actix::Handler<Ping> for MyActor3 {
    type Result = ();

    fn handle(&mut self, _: Ping, _: &mut actix::Context<MyActor3>) -> Self::Result {
        Arbiter::system().do_send(actix::msgs::SystemExit(0));
    }
}

#[test]
fn test_address() {
    let sys = System::new("test");
    let count = Arc::new(AtomicUsize::new(0));

    let addr: Addr<Unsync, _> = MyActor(Arc::clone(&count)).start();
    let addr2 = addr.clone();
    addr.do_send(Ping(0));

    Arbiter::handle().spawn_fn(move || {
        addr2.do_send(Ping(1));

        Timeout::new(Duration::new(0, 100), Arbiter::handle())
            .unwrap()
            .then(move |_| {
                addr2.do_send(Ping(2));
                Arbiter::system().do_send(actix::msgs::SystemExit(0));
                future::result(Ok(()))
            })
    });

    sys.run();
    assert_eq!(count.load(Ordering::Relaxed), 3);
}

#[test]
fn test_recipient_call() {
    let sys = System::new("test");
    let count = Arc::new(AtomicUsize::new(0));

    let addr: Addr<Unsync, _> = MyActor(Arc::clone(&count)).start();
    let addr2 = addr.clone().recipient();
    addr.do_send(UnsyncPing(0, PhantomData));

    Arbiter::handle().spawn(addr2.send(Ping(1)).then(move |_| {
        addr2.send(Ping(2)).then(|_| {
            Arbiter::system().do_send(actix::msgs::SystemExit(0));
            Ok(())
        })
    }));

    sys.run();
    assert_eq!(count.load(Ordering::Relaxed), 3);
}

#[test]
fn test_sync_address() {
    let sys = System::new("test");
    let count = Arc::new(AtomicUsize::new(0));
    let arbiter = Arbiter::new("sync-test");

    let addr: Addr<Syn, _> = MyActor(Arc::clone(&count)).start();
    let addr2 = addr.clone();
    let addr3 = addr.clone();
    addr.do_send(Ping(1));

    arbiter.do_send(actix::msgs::Execute::new(move || -> Result<(), ()> {
        addr3.do_send(Ping(2));
        Ok(())
    }));

    Arbiter::handle().spawn_fn(move || {
        addr2.do_send(Ping(3));

        Timeout::new(Duration::new(0, 100), Arbiter::handle())
            .unwrap()
            .then(move |_| {
                addr2.do_send(Ping(4));

                Arbiter::handle().spawn_fn(move || {
                    Timeout::new(Duration::new(0, 1000), Arbiter::handle())
                        .unwrap()
                        .then(move |_| {
                            Arbiter::system().do_send(actix::msgs::SystemExit(0));
                            future::result(Ok(()))
                        })
                });
                future::result(Ok(()))
            })
    });

    sys.run();
    thread::sleep(Duration::from_millis(200));

    assert_eq!(count.load(Ordering::Relaxed), 4);
}

#[test]
fn test_sync_recipient_call() {
    let sys = System::new("test");
    let count = Arc::new(AtomicUsize::new(0));

    let addr: Addr<Syn, _> = MyActor(Arc::clone(&count)).start();
    let addr2 = addr.clone().recipient();
    addr.do_send(Ping(0));

    Arbiter::handle().spawn(addr2.send(Ping(1)).then(move |_| {
        addr2.send(Ping(2)).then(|_| {
            Arbiter::system().do_send(actix::msgs::SystemExit(0));
            Ok(())
        })
    }));

    sys.run();
    assert_eq!(count.load(Ordering::Relaxed), 3);
}

#[test]
fn test_error_result() {
    let sys = System::new("test");

    let addr: Addr<Unsync, _> = MyActor3.start();

    Arbiter::handle().spawn_fn(move || {
        addr.send(Ping(0)).then(|res| {
            match res {
                Ok(_) => (),
                _ => panic!("Should not happen"),
            }
            futures::future::result(Ok(()))
        })
    });

    sys.run();
}

struct TimeoutActor;

impl Actor for TimeoutActor {
    type Context = actix::Context<Self>;
}

impl Handler<Ping> for TimeoutActor {
    type Result = ();

    fn handle(&mut self, _: Ping, ctx: &mut Self::Context) {
        Timeout::new(Duration::new(0, 5_000_000), Arbiter::handle())
            .unwrap()
            .map_err(|_| ())
            .into_actor(self)
            .wait(ctx);
    }
}

#[test]
fn test_message_timeout() {
    let sys = System::new("test");

    let addr: Addr<Unsync, _> = TimeoutActor.start();
    let count = Arc::new(AtomicUsize::new(0));
    let count2 = Arc::clone(&count);

    Arbiter::handle().spawn_fn(move || {
        addr.do_send(Ping(0));
        addr.send(Ping(0))
            .timeout(Duration::new(0, 1_000))
            .then(move |res| {
                match res {
                    Ok(_) => panic!("Should not happen"),
                    Err(MailboxError::Timeout) => {
                        count2.fetch_add(1, Ordering::Relaxed);
                    }
                    _ => panic!("Should not happen"),
                }
                Arbiter::system().do_send(actix::msgs::SystemExit(0));
                futures::future::result(Ok(()))
            })
    });

    sys.run();
    assert_eq!(count.load(Ordering::Relaxed), 1);
}

#[test]
fn test_sync_message_timeout() {
    let sys = System::new("test");

    let addr: Addr<Syn, _> = TimeoutActor.start();
    let count = Arc::new(AtomicUsize::new(0));
    let count2 = Arc::clone(&count);

    Arbiter::handle().spawn_fn(move || {
        addr.do_send(Ping(0));
        addr.send(Ping(0))
            .timeout(Duration::new(0, 1_000))
            .then(move |res| {
                match res {
                    Ok(_) => panic!("Should not happen"),
                    Err(MailboxError::Timeout) => {
                        count2.fetch_add(1, Ordering::Relaxed);
                    }
                    _ => panic!("Should not happen"),
                }
                Arbiter::system().do_send(actix::msgs::SystemExit(0));
                futures::future::result(Ok(()))
            })
    });

    sys.run();
    assert_eq!(count.load(Ordering::Relaxed), 1);
}

struct TimeoutActor2(Addr<Unsync, TimeoutActor>, Arc<AtomicUsize>);

impl Actor for TimeoutActor2 {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.0.do_send(Ping(0));
        self.0
            .send(Ping(0))
            .timeout(Duration::new(0, 1_000))
            .into_actor(self)
            .then(move |res, act, _| {
                match res {
                    Ok(_) => panic!("Should not happen"),
                    Err(MailboxError::Timeout) => {
                        act.1.fetch_add(1, Ordering::Relaxed);
                    }
                    _ => panic!("Should not happen"),
                }
                Arbiter::system().do_send(actix::msgs::SystemExit(0));
                actix::fut::ok(())
            })
            .wait(ctx)
    }
}

#[test]
fn test_call_message_timeout() {
    let sys = System::new("test");
    let addr: Addr<Unsync, _> = TimeoutActor.start();

    let count = Arc::new(AtomicUsize::new(0));
    let count2 = Arc::clone(&count);
    let _addr2: Addr<Unsync, _> = TimeoutActor2(addr, count2).start();

    sys.run();
    assert_eq!(count.load(Ordering::Relaxed), 1);
}

struct TimeoutActor3(Addr<Syn, TimeoutActor>, Arc<AtomicUsize>);

impl Actor for TimeoutActor3 {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.0.do_send(Ping(0));
        self.0
            .send(Ping(0))
            .timeout(Duration::new(0, 1_000))
            .into_actor(self)
            .then(move |res, act, _| {
                match res {
                    Ok(_) => panic!("Should not happen"),
                    Err(MailboxError::Timeout) => {
                        act.1.fetch_add(1, Ordering::Relaxed);
                    }
                    _ => panic!("Should not happen"),
                }
                Arbiter::system().do_send(actix::msgs::SystemExit(0));
                actix::fut::ok(())
            })
            .wait(ctx)
    }
}

#[test]
fn test_sync_call_message_timeout() {
    let sys = System::new("test");
    let addr: Addr<Syn, _> = TimeoutActor.start();

    let count = Arc::new(AtomicUsize::new(0));
    let count2 = Arc::clone(&count);
    let _addr2: Addr<Unsync, _> = TimeoutActor3(addr, count2).start();

    sys.run();
    assert_eq!(count.load(Ordering::Relaxed), 1);
}
