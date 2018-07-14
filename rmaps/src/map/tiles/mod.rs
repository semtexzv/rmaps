use prelude::*;

pub mod data;

use std::collections::BTreeSet;

// TODO, impl this

pub trait TileObserver {
    fn tile_changed();
}

pub struct TileStorage {
    pub in_flight: BTreeSet<TileCoords>,
    pub available: BTreeSet<TileCoords>,
}

impl TileStorage {
    pub fn new() -> Self {
        TileStorage {
            in_flight: BTreeSet::new(),
            available: BTreeSet::new(),
        }
    }
    pub fn needed_tiles(&self) -> Vec<TileCoords> {
        if self.available.is_empty() && self.in_flight.is_empty() {
            return vec![TileCoords::new(0, 0, 0)];
        }

        return self.available
            .iter()
            .filter(|x| x.z < 4)
            .flat_map(|x| Vec::from(&x.children()[..]))
            .filter(|t| !self.in_flight.contains(&t))
            .filter(|t| !self.available.contains(&t))
            .take(4).collect();
    }
    pub fn requested_tile(&mut self, coords: &TileCoords) {
        self.in_flight.insert(coords.clone());
    }
    pub fn finished_tile(&mut self, coords: &TileCoords) {
        self.in_flight.remove(coords);
        self.available.insert(coords.clone());
    }
}