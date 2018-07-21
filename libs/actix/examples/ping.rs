extern crate actix;
extern crate futures;

use actix::prelude::*;
use futures::Future;

/// Define `Ping` message
struct Ping(usize);

impl Message for Ping {
    type Result = usize;
}

/// Actor
struct MyActor {
    count: usize,
}

/// Declare actor and it's context
impl Actor for MyActor {
    type Context = Context<Self>;
}

/// Handler for `Ping` message
impl Handler<Ping> for MyActor {
    type Result = usize;

    fn handle(&mut self, msg: Ping, _: &mut Context<Self>) -> Self::Result {
        self.count += msg.0;
        self.count
    }
}

fn main() {
    // start system, this is required step
    let system = System::new("test");

    // start new actor
    let addr: Addr<Unsync, _> = MyActor { count: 10 }.start();

    // send message and get future for result
    let res = addr.send(Ping(10));

    // handle() returns tokio handle
    Arbiter::handle().spawn(res.map(|res| {
        println!("RESULT: {}", res == 20);

        // stop system and exit
        Arbiter::system().do_send(actix::msgs::SystemExit(0));
    }).map_err(|_| ()));

    system.run();
}
