use prelude::*;

pub mod data;
pub mod source;


use std::collections::BTreeSet;
use super::storage;

// TODO, impl this

pub trait TileObserver {
    fn tile_changed();
}

pub struct TileLoader {
    pub in_flight: BTreeSet<TileCoords>,
    pub available: BTreeSet<TileCoords>,
}

impl TileLoader {
    pub fn new() -> Self {
        TileLoader {
            in_flight: BTreeSet::new(),
            available: BTreeSet::new(),
        }
    }

    pub fn request_tile(&mut self, source : &source::TileSource, coord : TileCoords) {

    }
}