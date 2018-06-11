pub use std::{
    collections::BTreeMap,
    fmt::Debug,
    result::Result as StdResult,
    str::FromStr,
    string::ToString,
    ops::{Deref, DerefMut},
};
pub use gl;
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

pub use serde::{
    self
};


pub use futures::prelude::*;

pub use actix::{
    self,
    SystemRunner,
    prelude::*
};


pub use failure::{Error, Fail};
pub type Result<T> = StdResult<T,Error>;

pub type BoxFuture<T,E> = Box<Future<Item=T,Error=E>>;
