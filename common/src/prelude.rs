pub use std::{
    collections::{BTreeSet, BTreeMap},
    fmt::{
        self,
        Debug,
    },
    result::Result as StdResult,
    str::FromStr,
    string::ToString,
    ops::{Deref, DerefMut},
    borrow::Cow,
    rc::Rc,
    sync::Arc,
    mem,
};
pub use glium::{
    self,
    implement_vertex,
    vertex::VertexBuffer,
    index::{
        IndexBuffer,
        PrimitiveType,
    },
    Display,
    vertex::BufferCreationError,
    program::{
        Program,
        ProgramCreationInput,
    },
};
pub use cgmath::{SquareMatrix, Matrix4, Vector4};


pub use time::{
    Duration,
    PreciseTime,
};

pub use rayon::{
    self,
    prelude::*,
};

pub use regex::{
    self,
    Regex,
    RegexBuilder,
};

pub use num;

pub use css_color_parser;

pub use serde::{
    self, Serialize, Deserialize, Serializer, Deserializer, ser, de,
};

pub use serde_derive::{
    Serialize, Deserialize,
};


pub use futures::prelude::*;

pub use actix::{
    self,
    SystemRunner,
    prelude::*,
};

pub use geo;
pub use failure::{Error, Fail, bail};

pub type Result<T> = StdResult<T, Error>;

pub type BoxFuture<T, E> = Box<Future<Item=T, Error=E>>;
pub type SyncAddr<A> = Addr<Syn, A>;

pub const EXTENT: f32 = 8192.0;


pub struct Invoke<A, F, R>
    where A: Actor,
          F: FnOnce(&mut A) -> R,
          R: 'static

{
    pub f: F,
    _a: ::std::marker::PhantomData<A>,
    _r: ::std::marker::PhantomData<R>,
}

impl<A, F, R> Invoke<A, F, R>
    where A: Actor,
          F: FnOnce(&mut A) -> R,
          R: 'static

{
    pub fn new(f: F) -> Self {
        Invoke {
            f: f,
            _a: ::std::marker::PhantomData,
            _r: ::std::marker::PhantomData,
        }
    }
}

impl<A, F, R> From<F> for Invoke<A, F, R>
    where A: Actor,
          F: FnOnce(&mut A) -> R,
          R: 'static
{
    fn from(f: F) -> Self {
        Self::new(f)
    }
}

impl<A, F, R> Message for Invoke<A, F, R>
    where A: Actor,
          F: FnOnce(&mut A) -> R,
          R: 'static
{
    type Result = Result<R>;
}

use actix::dev::*;

pub fn spawn<E: Into<Error>>(fut: impl Future<Item=(), Error=E> + 'static) {
    Arbiter::handle().spawn(fut.map_err(|e| {
        error!("Error occured: {}", e.into());
        ()
    }));
}


#[macro_export]
macro_rules! impl_invoke_handler {
    ($ty:ty) => {
        impl<F, R> Handler<Invoke<$ty, F, R>> for $ty
        where F: FnOnce(&mut $ty) -> R,
              R: 'static
        {
            type Result = Result<R>;

            fn handle(&mut self, msg: Invoke<$ty, F, R>, _ctx: &mut Context<Self>) -> Result<R> {
                Ok((msg.f)(self))
            }
        }

    };
}