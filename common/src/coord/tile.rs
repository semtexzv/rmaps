use ::prelude::*;
use super::point::*;

use util::{LatLng, LatLngBounds};


#[derive(Debug, Clone, Copy, Hash, PartialOrd, PartialEq, Ord, Eq)]
pub struct TileCoords {
    pub z: i32,
    pub y: i32,
    pub x: i32,
}

impl TileCoords {
    pub fn new(x: impl Into<i32>, y: impl Into<i32>, z: impl Into<i32>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    pub fn parent(&self) -> Option<Self> {
        if self.z > 0 {
            return Some(Self::new(
                self.x / 2,
                self.y / 2,
                self.z - 1,
            ));
        } else {
            return None;
        }
    }

    pub fn id(&self) -> u8 {
        let x = (self.x & 0xFF) as u8;
        let y = (self.y & 0xFF) as u8;
        let z = (self.z & 0xFF) as u8;

        return z & 0b11 << 6 | x & 0b111 << 3 | y & 0b111;
    }


    pub fn children(&self) -> [Self; 4] {
        return [
            Self::new(self.x * 2 + 0, self.y * 2 + 0, self.z + 1),
            Self::new(self.x * 2 + 1, self.y * 2 + 0, self.z + 1),
            Self::new(self.x * 2 + 1, self.y * 2 + 1, self.z + 1),
            Self::new(self.x * 2 + 0, self.y * 2 + 1, self.z + 1),
        ];
    }

    pub fn bounds(&self) -> LatLngBounds {
        use std::f32::consts::PI;
        let n = num::pow(2.0f32, self.z as usize);

        fn lon_d(x: f32, n: f32) -> f32 {
            x / n * 360. - 180.
        }
        fn lat_d(y: f32, n: f32) -> f32 {
            let rad = f32::atan(f32::sinh((PI * (1. - 2. * y / n))));

            rad * 180. / PI
        }

        let x = self.x as f32;
        let y = self.y as f32;

        let nw = LatLng {
            lat: lat_d(y, n) as f64,
            lng: lon_d(x, n) as f64,
        };
        let se = LatLng {
            lat: lat_d(y + 1., n) as f64,
            lng: lon_d(x + 1., n) as f64,
        };

        return LatLngBounds {
            nw,
            se,
        };
    }
}

impl Into<UnwrappedTileCoords> for TileCoords {
    fn into(self) -> UnwrappedTileCoords {
        UnwrappedTileCoords {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialOrd, PartialEq, Ord, Eq)]
pub struct UnwrappedTileCoords {
    pub z: i32,
    pub y: i32,
    pub x: i32,
}

impl UnwrappedTileCoords {
    pub fn new(x: impl Into<i32>, y: impl Into<i32>, z: impl Into<i32>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    pub fn parent(&self) -> Option<Self> {
        if self.z > 0 {
            let x = if self.x < 0 {
                self.x / 2 - 1
            } else {
                self.x / 2
            };

            return Some(Self::new(
                x,
                self.y / 2,
                self.z - 1,
            ));
        } else {
            return None;
        }
    }

    pub fn id(&self) -> u8 {
        let x = (self.x & 0xFF) as u8;
        let y = (self.y & 0xFF) as u8;
        let z = (self.z & 0xFF) as u8;

        return z & 0b11 << 6 | x & 0b111 << 3 | y & 0b111;
    }


    pub fn children(&self) -> [Self; 4] {
        return [
            Self::new(self.x * 2 + 0, self.y * 2 + 0, self.z + 1),
            Self::new(self.x * 2 + 1, self.y * 2 + 0, self.z + 1),
            Self::new(self.x * 2 + 1, self.y * 2 + 1, self.z + 1),
            Self::new(self.x * 2 + 0, self.y * 2 + 1, self.z + 1),
        ];
    }

    pub fn bounds(&self) -> LatLngBounds {
        use std::f32::consts::PI;
        let n = num::pow(2.0f32, self.z as usize);

        fn lon_d(x: f32, n: f32) -> f32 {
            x / n * 360. - 180.
        }
        fn lat_d(y: f32, n: f32) -> f32 {
            let rad = f32::atan(f32::sinh((PI * (1. - 2. * y / n))));

            rad * 180. / PI
        }

        let x = self.x as f32;
        let y = self.y as f32;

        let nw = LatLng {
            lat: lat_d(y, n) as f64,
            lng: lon_d(x, n) as f64,
        };
        let se = LatLng {
            lat: lat_d(y + 1., n) as f64,
            lng: lon_d(x + 1., n) as f64,
        };

        return LatLngBounds {
            nw,
            se,
        };
    }

    pub fn wrap(&self) -> TileCoords {
        let x = self.x;
        let tiles = 1 << self.z;

        let wrap = (if x < 0 { x - tiles + 1 } else { x }) / tiles;
        let cx = self.x - wrap * tiles;

        return TileCoords::new(cx, self.y % tiles, self.z);
    }
}