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


pub use failure::{Error, Fail, bail};

pub type Result<T> = StdResult<T, Error>;

pub type BoxFuture<T, E> = Box<Future<Item=T, Error=E>>;
pub type SyncAddr<A> = Addr<Syn, A>;

pub const EXTENT: f32 = 8192.0;