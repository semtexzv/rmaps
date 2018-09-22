pub use std::{
    collections::{BTreeSet, BTreeMap},
    fmt::{
        self,
        Debug,
    },
    marker::PhantomData,
    result::Result as StdResult,
    str::FromStr,
    string::ToString,
    ops::{Deref, DerefMut},
    borrow::Cow,
    rc::Rc,
    sync::Arc,
    cell::{
        self,
        RefCell,
    },
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


pub use std::time::Instant;

pub use time::Duration;

#[derive(Debug, PartialOrd, Ord, PartialEq, Hash, Eq, Copy, Clone)]
pub struct PreciseTime(u64);

impl PreciseTime {
    /// Returns a `PreciseTime` representing the current moment in time.
    pub fn now() -> PreciseTime {
        PreciseTime(::time::precise_time_ns())
    }

    /// Returns a `Duration` representing the span of time from the value of
    /// `self` to the value of `later`.
    ///
    /// # Notes
    ///
    /// If `later` represents a time before `self`, the result of this method
    /// is unspecified.
    ///
    /// If `later` represents a time more than 293 years after `self`, the
    /// result of this method is unspecified.
    #[inline]
    pub fn to(&self, later: PreciseTime) -> Duration {
        // NB: even if later is less than self due to overflow, this will work
        // since the subtraction will underflow properly as well.
        //
        // We could deal with the overflow when casting to an i64, but all that
        // gets us is the ability to handle intervals of up to 584 years, which
        // seems not very useful :)
        Duration::nanoseconds((later.0 - self.0) as i64)
    }
}

pub use rayon::{
    self,
    iter::{IntoParallelRefMutIterator, ParallelIterator},
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
    Actor,
    Context,
    Handler,
    Message,
    System,
    Arbiter,
    Addr,
    Recipient,
    AsyncContext,
};

pub use geo;
pub use failure::{Error, Fail, bail};


pub type Result<T> = StdResult<T, Error>;

pub type BoxFuture<T, E> = Box<Future<Item=T, Error=E>>;

pub const EXTENT: f32 = 8192.0;

use std::sync::Mutex;


pub struct ForceSend<T>(pub T);

unsafe impl<T> Send for ForceSend<T> {}

unsafe impl<T> Sync for ForceSend<T> {}

pub struct Invoke<A, F, R>
    where A: Actor,
          F: FnOnce(&mut A, &mut A::Context) -> R,
          R: 'static

{
    pub f: F,
    _a: ::std::marker::PhantomData<ForceSend<A>>,
    _r: ::std::marker::PhantomData<R>,
}

impl<A, F, R> Invoke<A, F, R>
    where A: Actor,
          F: FnOnce(&mut A, &mut A::Context) -> R,
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
          F: FnOnce(&mut A, &mut A::Context) -> R,
          R: 'static
{
    fn from(f: F) -> Self {
        Self::new(f)
    }
}

impl<A, F, R> Message for Invoke<A, F, R>
    where A: Actor,
          F: FnOnce(&mut A, &mut A::Context) -> R,
          R: 'static
{
    type Result = Result<R>;
}

use actix::dev::*;

pub fn spawn<E: Into<Error>>(fut: impl Future<Item=(), Error=E> + 'static) {
    actix::spawn(fut.map_err(|e| {
        error!("Error occured: {}", e.into());
        ()
    }));
    /*Arbiter::handle().spawn(fut.map_err(|e| {
        error!("Error occured: {}", e.into());
        ()
    }));
    */
}


#[macro_export]
macro_rules! impl_invoke_handler {
    ($ty:ty) => {
        impl<F, R> Handler<Invoke<$ty, F, R>> for $ty
        where F: FnOnce(&mut $ty, &mut <$ty as Actor>::Context) -> R,
              R: 'static
        {
            type Result = Result<R>;

            fn handle(&mut self, msg: Invoke<$ty, F, R>, _ctx: &mut Context<Self>) -> Result<R> {
                Ok((msg.f)(self,_ctx))
            }
        }

    };
}