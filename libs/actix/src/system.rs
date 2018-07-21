use futures::Future;
use futures::sync::oneshot::{channel, Receiver, Sender};
use std::collections::HashMap;
use tokio_core::reactor::{Core, Handle};

use actor::Actor;
use address::{Addr, Syn};
use arbiter::Arbiter;
use context::Context;
use handler::{Handler, Message};
use msgs::{StopArbiter, SystemExit};

/// System is an actor which manages process.
///
/// Before starting any actix's actors, `System` actor has to be created
/// with `System::new()` call. This method creates new `Arbiter` in current
/// thread and starts `System` actor.
///
/// # Examples
///
/// ```rust
/// extern crate actix;
///
/// use actix::prelude::*;
/// use std::time::Duration;
///
/// struct Timer {
///     dur: Duration,
/// }
///
/// impl Actor for Timer {
///     type Context = Context<Self>;
///
///     // stop system after `self.dur` seconds
///     fn started(&mut self, ctx: &mut Context<Self>) {
///         ctx.run_later(self.dur, |act, ctx| {
///             // send `SystemExit` to `System` actor.
///             Arbiter::system().do_send(actix::msgs::SystemExit(0));
///         });
///     }
/// }
///
/// fn main() {
///     // initialize system
///     let sys = System::new("test");
///
///     // Start `Timer` actor
///     let _: () = Timer {
///         dur: Duration::new(0, 1),
///     }.start();
///
///     // Run system, this function blocks current thread
///     let code = sys.run();
///     std::process::exit(code);
/// }
/// ```
pub struct System {
    stop: Option<Sender<i32>>,
    arbiters: HashMap<String, Addr<Syn, Arbiter>>,
}

impl Actor for System {
    type Context = Context<Self>;
}

impl System {
    #[cfg_attr(feature = "cargo-clippy", allow(new_ret_no_self))]
    /// Create new system
    pub fn new<T: Into<String>>(name: T) -> SystemRunner {
        let name = name.into();
        let core = Arbiter::new_system(name.clone());
        let (stop_tx, stop) = channel();

        // start system
        let sys = System {
            arbiters: HashMap::new(),
            stop: Some(stop_tx),
        }.start();
        Arbiter::set_system(sys, name);

        SystemRunner { core, stop }
    }
}

/// Helper object that runs System's event loop
#[must_use = "SystemRunner must be run"]
pub struct SystemRunner {
    core: Core,
    stop: Receiver<i32>,
}

impl SystemRunner {
    /// Returns handle to the current event loop.
    pub fn handle(&self) -> &Handle {
        Arbiter::handle()
    }

    /// This function will start event loop and will finish once the
    /// `SystemExit` message get received.
    pub fn run(self) -> i32 {
        let SystemRunner {
            mut core, stop, ..
        } = self;

        // run loop
        match core.run(stop) {
            Ok(code) => code,
            Err(_) => 1,
        }
    }

    pub fn run_until_complete<F, I, E>(&mut self, fut: F) -> Result<I, E>
    where
        F: Future<Item = I, Error = E>,
    {
        self.core.run(fut)
    }
}

impl Handler<SystemExit> for System {
    type Result = ();

    fn handle(&mut self, msg: SystemExit, _: &mut Context<Self>) {
        // stop arbiters
        for addr in self.arbiters.values() {
            addr.do_send(StopArbiter(msg.0));
        }
        // stop event loop
        if let Some(stop) = self.stop.take() {
            let _ = stop.send(msg.0);
        }
    }
}

/// Register Arbiter within system
pub(crate) struct RegisterArbiter(pub String, pub Addr<Syn, Arbiter>);

#[doc(hidden)]
impl Message for RegisterArbiter {
    type Result = ();
}

#[doc(hidden)]
impl Handler<RegisterArbiter> for System {
    type Result = ();

    fn handle(&mut self, msg: RegisterArbiter, _: &mut Context<Self>) {
        self.arbiters.insert(msg.0, msg.1);
    }
}

/// Unregister Arbiter
pub(crate) struct UnregisterArbiter(pub String);

#[doc(hidden)]
impl Message for UnregisterArbiter {
    type Result = ();
}

#[doc(hidden)]
impl Handler<UnregisterArbiter> for System {
    type Result = ();

    fn handle(&mut self, msg: UnregisterArbiter, _: &mut Context<Self>) {
        self.arbiters.remove(&msg.0);
    }
}
