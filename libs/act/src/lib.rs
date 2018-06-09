#![allow(mutable_transmutes)]
#[macro_use]
extern crate lazy_static;

use std::any::{TypeId, Any};
use std::collections::{
    BTreeMap, HashMap,
};
use std::ops::DerefMut;


pub struct Inbox {
    pub recvr: ::std::sync::mpsc::Receiver<MessageWrapper>,
}

type ErasedAct = *mut ();
type ErasedMsg = *mut ();
type HandlerFn = Box<Fn(ErasedAct, ErasedMsg) + 'static + Sync>;

pub struct World {
    handlers: BTreeMap<(TypeId, TypeId), HandlerFn>,

}

impl World {
    pub fn register_handler<A: Actor + Sized, M: Message + Sized>(&mut self, handler: impl Fn(&mut A, Box<M>) + 'static + Send + Sync) {
        let at = TypeId::of::<A>();
        let mt = TypeId::of::<M>();
        self.handlers.insert((at, mt), Box::new(move |act: ErasedAct, msg: ErasedMsg| {
            unsafe {
                let msg = Box::from_raw(msg as _);

                handler(::std::mem::transmute(act as *mut A), msg);
            }
        }));
    }

    pub fn handle_msg<A: Actor>(&mut self, act: &mut A, mut msg: MessageWrapper) {
        let at = TypeId::of::<A>();
        let mt = msg.typ;

        let res = self.handlers.get(&(at, mt)).unwrap();

        res(act as *mut A as *mut (), Box::into_raw(msg.data) as _);
    }


}

impl Default for World {
    fn default() -> Self {
        World {
            handlers: BTreeMap::new(),
        }
    }


}

lazy_static! {
    static ref WORLD: World = {
        Default::default()
    };
}

pub fn world() -> &'static mut World {
    return unsafe { ::std::mem::transmute(&(*WORLD)) };
}


pub trait Actor: 'static {
    type HandleType: ActorHandle;

    fn process_messages(&mut self);
    fn handle(&self) -> &Self::HandleType;
}


pub trait Message: 'static + Send {}

pub struct MessageWrapper {
    data: Box<Message>,
    typ: TypeId,
}

impl MessageWrapper {
    pub fn new<M: Message>(m: M) -> MessageWrapper {
        MessageWrapper {
            typ: TypeId::of::<M>(),
            data: Box::new(m),
        }
    }
}

pub trait ActorHandle: Clone + Send {

}
