use ::prelude::*;
use ::common::rusqlite::{Connection,Statement};

pub struct OfflineCache {
    db : Connection
}

impl OfflineCache {
    pub fn new(path : &str) -> Result<Self> {
        Ok(OfflineCache {
            db : Connection::open(path)?,
        })
    }
}