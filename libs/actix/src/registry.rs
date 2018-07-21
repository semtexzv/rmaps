//! Actors registry
//!
//! Actor can register itself as a service. Service can be defined as
//! `ArbiterService` which is unique per arbiter or `SystemService` which is
//! unique per system.
use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::default::Default;
use std::sync::{Arc, Mutex};

use actor::{Actor, Supervised};
use address::{Addr, Syn, Unsync};
use arbiter::Arbiter;
use context::Context;
use supervisor::Supervisor;

/// Actors registry
///
/// Actor can register itself as a service. Service can be defined as
/// `ArbiterService` which is unique per arbiter or `SystemService` which is
/// unique per system.
///
/// # Example
///
/// ```rust
/// # #[macro_use] extern crate actix;
/// use actix::prelude::*;
///
/// #[derive(Message)]
/// struct Ping;
///
/// #[derive(Default)]
/// struct MyActor1;
///
/// impl Actor for MyActor1 {
///     type Context = Context<Self>;
/// }
/// impl actix::Supervised for MyActor1 {}
///
/// impl ArbiterService for MyActor1 {
///    fn service_started(&mut self, ctx: &mut Context<Self>) {
///       println!("Service started");
///    }
/// }
///
/// impl Handler<Ping> for MyActor1 {
///    type Result = ();
///
///    fn handle(&mut self, _: Ping, ctx: &mut Context<Self>) {
///       println!("ping");
/// #     Arbiter::system().do_send(actix::msgs::SystemExit(0));
///    }
/// }
///
/// struct MyActor2;
///
/// impl Actor for MyActor2 {
///    type Context = Context<Self>;
///
///    fn started(&mut self, _: &mut Context<Self>) {
///       let act = Arbiter::registry().get::<MyActor1>();
///       act.do_send(Ping);
///    }
/// }
///
///
/// fn main() {
///    // initialize system
///    let sys = System::new("test");
///
///    // Start MyActor1
///    let _:() = MyActor1.start();
///
///    // Start MyActor2
///    let _:() = MyActor2.start();
///
///    // Run system, this function blocks current thread
///    let code = sys.run();
/// #  std::process::exit(code);
/// }
/// ```
pub struct Registry {
    registry: RefCell<HashMap<TypeId, Box<Any>>>,
}

/// Trait defines arbiter's service.
#[allow(unused_variables)]
pub trait ArbiterService: Actor<Context = Context<Self>> + Supervised + Default {
    /// Method is called during service initialization.
    fn service_started(&mut self, ctx: &mut Context<Self>) {}

    /// Get actor's address from arbiter registry
    fn from_registry() -> Addr<Unsync, Self> {
        Arbiter::registry().get::<Self>()
    }

    /// Create an actor in the Arbiter with a closure
    fn init_actor<F>(f: F) -> Addr<Unsync, Self> where F: FnOnce(&mut Self::Context) -> Self + 'static{
        Arbiter::registry().init_actor::<Self, F>(f)
    }
}

/// Trait defines system's service.
#[allow(unused_variables)]
pub trait SystemService: Actor<Context = Context<Self>> + Supervised + Default {
    /// Method is called during service initialization.
    fn service_started(&mut self, ctx: &mut Context<Self>) {}

    /// Get actor's address from system registry
    fn from_registry() -> Addr<Syn, Self> {
        Arbiter::system_registry().get::<Self>()
    }

    /// Create an SystemService with a closure
    fn init_actor<F>(f: F) -> Addr<Syn, Self> where F: FnOnce(&mut Self::Context) -> Self + Send + 'static{
        Arbiter::system_registry().init_actor::<Self, F>(f)
    }
}

impl Registry {
    pub(crate) fn new() -> Self {
        Registry {
            registry: RefCell::new(HashMap::new()),
        }
    }

    /// Query registry for specific actor. Returns address of the actor.
    /// If actor is not registered, starts new actor and
    /// return address of newly created actor.
    pub fn get<A: ArbiterService + Actor<Context=Context<A>>>(&self) -> Addr<Unsync, A> {
        let id = TypeId::of::<A>();
        if let Some(addr) = self.registry.borrow().get(&id) {
            if let Some(addr) = addr.downcast_ref::<Addr<Unsync, A>>() {
                return addr.clone()
            }
        }
        let addr: Addr<Unsync, A> = Supervisor::start(|ctx| {
            let mut act = A::default();
            act.service_started(ctx);
            act
        });

        self.registry.borrow_mut().insert(id, Box::new(addr.clone()));
        addr
    }

    /// Add new actor to the registry using the initialization function provided, panic if actor
    /// is already running
    pub fn init_actor<A: ArbiterService + Actor<Context=Context<A>>, F>(&self, with: F) -> Addr<Unsync, A> where F: FnOnce(&mut A::Context) -> A + 'static {
        let id = TypeId::of::<A>();
        if let Some(addr) = self.registry.borrow().get(&id) {
            if let Some(addr) = addr.downcast_ref::<Addr<Unsync, A>>() {
                return addr.clone()
            }
        }
        let addr: Addr<Unsync, A> = Supervisor::start(|ctx| {
            let mut act = with(ctx);
            act.service_started(ctx);
            act
        });

        self.registry.borrow_mut().insert(id, Box::new(addr.clone()));
        addr
    }

    /// Add new actor to the registry using the initialization function provided, panic if actor
    /// is already running
    pub fn set<A: ArbiterService + Actor<Context=Context<A>>>(&self, addr: Addr<Unsync, A>) {
        let id = TypeId::of::<A>();
        if let Some(addr) = self.registry.borrow().get(&id) {
             match addr.downcast_ref::<Addr<Unsync, A>>() {
                Some(_) => {
                    panic!("Actor already started")
                },
                None => {},
            }
        }

        self.registry.borrow_mut().insert(id, Box::new(addr.clone()));
    }
}

// TODO: Remove lock
/// System wide actors registry
///
/// System registry serves same purpose as [Registry](struct.Registry.html), except
/// it is shared across all arbiters.
pub struct SystemRegistry {
    registry: Arc<Mutex<HashMap<TypeId, Box<Any>>>>,
}

unsafe impl Send for SystemRegistry {}

impl SystemRegistry {
    pub(crate) fn new() -> Self {
        SystemRegistry{registry: Arc::new(Mutex::new(HashMap::new()))}
    }

    /// Return address of the service. If service actor is not running
    /// it get started in system arbiter.
    pub fn get<A: SystemService + Actor<Context=Context<A>>>(&self) -> Addr<Syn,A> {
        {
            if let Ok(hm) = self.registry.lock() {
                if let Some(addr) = hm.get(&TypeId::of::<A>()) {
                    match addr.downcast_ref::<Addr<Syn, A>>() {
                        Some(addr) => {
                            return addr.clone()
                        },
                        None => error!("Got unknown value: {:?}", addr),
                    }
                }
            } else { panic!("System registry lock is poisoned"); }
        }

        let addr = Supervisor::start_in(&Arbiter::system_arbiter(), |ctx| {
            let mut act = A::default();
            act.service_started(ctx);
            act
        });
        if let Ok(mut hm) = self.registry.lock() {
            hm.insert(TypeId::of::<A>(), Box::new(addr.clone()));
            return addr
        }
        panic!("System registry lock is poisoned");
    }

    /// Initialize a SystemService, panic if already started
    pub fn init_actor<A: SystemService + Actor<Context=Context<A>>, F>(&self, with: F) -> Addr<Syn,A> where F: FnOnce(&mut A::Context) -> A + Send + 'static {
        let addr = Supervisor::start_in(&Arbiter::system_arbiter(), |ctx| {
            let mut act = with(ctx);
            act.service_started(ctx);
            act
        });

        self.set(addr.clone());
        addr
    }

    /// Initialize a SystemService to a actor address (can be used to start SyncActors, panic if already started
    pub fn set<A: SystemService + Actor<Context=Context<A>>>(&self, addr: Addr<Syn,A>) {
        {
            if let Ok(hm) = self.registry.lock() {
                if let Some(addr) = hm.get(&TypeId::of::<A>()) {
                    match addr.downcast_ref::<Addr<Syn, A>>() {
                        Some(_) => {
                            panic!("Actor already started")
                        },
                        None => {},
                    }
                }
            } else { panic!("System registry lock is poisoned"); }
        }

        if let Ok(mut hm) = self.registry.lock() {
            hm.insert(TypeId::of::<A>(), Box::new(addr.clone()));
        } else {
            panic!("System registry lock is poisoned");
        }
    }
}

impl Clone for SystemRegistry {
    fn clone(&self) -> Self {
        SystemRegistry{registry: Arc::clone(&self.registry)}
    }
}
