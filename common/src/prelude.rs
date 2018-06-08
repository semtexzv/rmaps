pub use std::{
    collections::{
        BTreeMap
    },
    fmt::{
        Debug
    },
    result::Result as StdResult,
};
pub use gl;
pub use glium::{
    self,
    implement_vertex,
    vertex::{
        VertexBuffer
    },
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



use error_chain::{
    error_chain, error_chain_processing,
    impl_error_chain_processed,impl_extract_backtrace,impl_error_chain_kind
};




error_chain! {
    foreign_links {
        GliumVertexCreate(glium::vertex::BufferCreationError);
        GliumIndexCreate(glium::index::BufferCreationError);
        ProgramCreate(glium::program::ProgramCreationError);
        ProgramCreateChooser(glium::program::ProgramChooserCreationError);
    }
}