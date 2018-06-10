use futures::sync::oneshot::{channel, Sender};
use std;
use std::cell::RefCell;
use std::thread;
use tokio_core::reactor::{Core, Handle};
use uuid::Uuid;

use actor::{Actor, AsyncContext};
use address::{sync_channel, Addr, Syn, Unsync};
use context::Context;
use handler::Handler;
use mailbox::DEFAULT_CAPACITY;
use msgs::{Execute, StartActor, StopArbiter};
use registry::{Registry, SystemRegistry};
use system::{RegisterArbiter, System, UnregisterArbiter};

thread_local!(
    static HND: RefCell<Option<Handle>> = RefCell::new(None);
    static STOP: RefCell<Option<Sender<i32>>> = RefCell::new(None);
    static ADDR: RefCell<Option<Addr<Unsync, Arbiter>>> = RefCell::new(None);
    static REG: RefCell<Option<Registry>> = RefCell::new(None);
    static NAME: RefCell<Option<String>> = RefCell::new(None);
    static SYS: RefCell<Option<Addr<Syn, System>>> = RefCell::new(None);
    static SYSARB: RefCell<Option<Addr<Syn, Arbiter>>> = RefCell::new(None);
    static SYSNAME: RefCell<Option<String>> = RefCell::new(None);
    static SYSREG: RefCell<Option<SystemRegistry>> = RefCell::new(None);
);

/// Event loop controller
///
/// Arbiter controls event loop in it's thread. Each arbiter runs in separate
/// thread. Arbiter provides several api for event loop access. Each arbiter
/// can belongs to specific `System` actor.
pub struct Arbiter {
    id: Uuid,
    sys: bool,
}

impl Actor for Arbiter {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        // register arbiter within system
        if !self.sys {
            Arbiter::system().do_send(RegisterArbiter(
                self.id.simple().to_string(),
                ctx.address(),
            ));
        }
    }
}

impl Arbiter {
    /// Spawn new thread and run event loop in spawned thread.
    /// Returns address of newly created arbiter.
    pub fn new<T: Into<String>>(name: T) -> Addr<Syn, Arbiter> {
        let (tx, rx) = std::sync::mpsc::channel();

        let id = Uuid::new_v4();
        let sys = Arbiter::system();
        let sys_name = Arbiter::system_name();
        let sys_arbiter = Arbiter::system_arbiter();
        let sys_registry = Arbiter::system_registry().clone();
        let name = format!(
            "arbiter:{:?}:{:?}",
            id.hyphenated().to_string(),
            name.into()
        );

        let _ = thread::Builder::new()
            .name(name.clone())
            .spawn(move || {
                let mut core = Core::new().unwrap();

                let (stop_tx, stop_rx) = channel();
                HND.with(|cell| *cell.borrow_mut() = Some(core.handle()));
                STOP.with(|cell| *cell.borrow_mut() = Some(stop_tx));
                NAME.with(|cell| *cell.borrow_mut() = Some(name));
                REG.with(|cell| *cell.borrow_mut() = Some(Registry::new()));

                // system
                SYS.with(|cell| *cell.borrow_mut() = Some(sys));
                SYSARB.with(|cell| *cell.borrow_mut() = Some(sys_arbiter));
                SYSNAME.with(|cell| *cell.borrow_mut() = Some(sys_name));
                SYSREG.with(|cell| *cell.borrow_mut() = Some(sys_registry));

                // start arbiter
                let (addr, saddr) = Actor::start(Arbiter { id, sys: false });
                ADDR.with(|cell| *cell.borrow_mut() = Some(addr));

                if tx.send(saddr).is_err() {
                    error!("Can not start Arbiter, remote side is dead");
                } else {
                    // run loop
                    let _ = match core.run(stop_rx) {
                        Ok(code) => code,
                        Err(_) => 1,
                    };
                }

                // unregister arbiter
                Arbiter::system().do_send(UnregisterArbiter(id.simple().to_string()));
            });

        rx.recv().unwrap()
    }

    pub(crate) fn new_system(name: String) -> Core {
        let core = Core::new().unwrap();
        HND.with(|cell| *cell.borrow_mut() = Some(core.handle()));
        REG.with(|cell| *cell.borrow_mut() = Some(Registry::new()));
        NAME.with(|cell| *cell.borrow_mut() = Some(name));
        SYSREG.with(|cell| *cell.borrow_mut() = Some(SystemRegistry::new()));

        // start arbiter
        let (addr, sys_addr) = Actor::start(Arbiter {
            sys: true,
            id: Uuid::new_v4(),
        });
        ADDR.with(|cell| *cell.borrow_mut() = Some(addr));
        SYSARB.with(|cell| *cell.borrow_mut() = Some(sys_addr));

        core
    }

    pub(crate) fn set_system(addr: Addr<Syn, System>, name: String) {
        SYS.with(|cell| *cell.borrow_mut() = Some(addr));
        SYSNAME.with(|cell| *cell.borrow_mut() = Some(name));
    }

    /// Returns current arbiter's address
    pub fn name() -> String {
        NAME.with(|cell| match *cell.borrow() {
            Some(ref name) => name.clone(),
            None => "Arbiter is not running".into(),
        })
    }

    /// Returns current arbiter's address
    pub fn arbiter() -> Addr<Unsync, Arbiter> {
        ADDR.with(|cell| match *cell.borrow() {
            Some(ref addr) => addr.clone(),
            None => panic!("Arbiter is not running"),
        })
    }

    /// This function returns system address,
    pub fn system() -> Addr<Syn, System> {
        SYS.with(|cell| match *cell.borrow() {
            Some(ref addr) => addr.clone(),
            None => panic!("System is not running"),
        })
    }

    /// This function returns system address,
    pub fn system_arbiter() -> Addr<Syn, Arbiter> {
        SYSARB.with(|cell| match *cell.borrow() {
            Some(ref addr) => addr.clone(),
            None => panic!("System is not running"),
        })
    }

    /// This function returns system name,
    pub fn system_name() -> String {
        SYSNAME.with(|cell| match *cell.borrow() {
            Some(ref name) => name.clone(),
            None => panic!("System is not running"),
        })
    }

    /// This function returns system registry,
    pub fn system_registry() -> &'static SystemRegistry {
        SYSREG.with(|cell| match *cell.borrow() {
            Some(ref reg) => unsafe { std::mem::transmute(reg) },
            None => panic!("System is not running"),
        })
    }

    /// This function returns current event loop's handle,
    pub fn handle() -> &'static Handle {
        HND.with(|cell| match *cell.borrow() {
            Some(ref h) => unsafe { std::mem::transmute(h) },
            None => panic!("Arbiter is not running"),
        })
    }

    /// This function returns arbiter's registry,
    pub fn registry() -> &'static Registry {
        REG.with(|cell| match *cell.borrow() {
            Some(ref reg) => unsafe { std::mem::transmute(reg) },
            None => panic!("System is not running: {}", Arbiter::name()),
        })
    }

    /// Start new arbiter and then start actor in created arbiter.
    /// Returns `Addr<Syn, A>` of created actor.
    pub fn start<A, F>(f: F) -> Addr<Syn, A>
    where
        A: Actor<Context = Context<A>>,
        F: FnOnce(&mut A::Context) -> A + Send + 'static,
    {
        let (stx, srx) = sync_channel::channel(DEFAULT_CAPACITY);

        // new arbiter
        let addr = Arbiter::new("actor");

        // create actor
        addr.do_send::<Execute>(Execute::new(move || {
            let mut ctx = Context::with_receiver(None, srx);
            let act = f(&mut ctx);
            ctx.set_actor(act);
            ctx.run(Arbiter::handle());
            Ok(())
        }));

        Addr::new(stx)
    }
}

impl Handler<StopArbiter> for Arbiter {
    type Result = ();

    fn handle(&mut self, msg: StopArbiter, _: &mut Context<Self>) {
        if self.sys {
            warn!(
                "System arbiter received `StopArbiter` message.
                  To shutdown system, `SystemExit` message should be
                  send to `Addr<Syn, System>`"
            );
        } else {
            STOP.with(|cell| {
                if let Some(stop) = cell.borrow_mut().take() {
                    let _ = stop.send(msg.0);
                }
            });
        }
    }
}

impl<A> Handler<StartActor<A>> for Arbiter
where
    A: Actor<Context = Context<A>>,
{
    type Result = Addr<Syn, A>;

    fn handle(&mut self, msg: StartActor<A>, _: &mut Context<Self>) -> Addr<Syn, A> {
        msg.call()
    }
}

/// Execute function in arbiter's thread
impl<I: Send, E: Send> Handler<Execute<I, E>> for Arbiter {
    type Result = Result<I, E>;

    fn handle(&mut self, msg: Execute<I, E>, _: &mut Context<Self>) -> Result<I, E> {
        msg.exec()
    }
}
