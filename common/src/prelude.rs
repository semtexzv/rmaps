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