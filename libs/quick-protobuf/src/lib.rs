//! A library to read binary protobuf files
//!
//! This reader is developed similarly to a pull reader

#![deny(missing_docs)]
#![allow(dead_code)]

extern crate byteorder;
extern crate failure;
#[macro_use]
extern crate failure_derive;

pub mod errors;
pub mod message;
pub mod reader;
pub mod sizeofs;
pub mod writer;

pub use errors::{Error, Result};
pub use message::{MessageRead, MessageWrite};
pub use reader::{deserialize_from_slice, BytesReader, Reader};
pub use writer::{serialize_into_vec, Writer};

/// Simple helper
pub struct BufRefAccess<'acc> {
    /// Internal reader
    pub r: &'acc mut BytesReader,
    /// Internal Buffer
    pub buf: &'acc [u8],
}
impl<'acc> BufRefAccess<'acc> {
    /// Read nested message
    pub fn read_nested<F: FnMut(&mut BufRefAccess) -> Result<()>>(&mut self, mut fun: F) -> Result<()> {
        self.r.read_len_varint(self.buf, |r, buf| {
            fun(&mut BufRefAccess { r, buf })
        })
    }
}