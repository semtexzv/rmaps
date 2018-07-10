use prelude::*;

use cgmath::SquareMatrix;

#[derive(Debug, Clone, Copy, Hash, PartialOrd, PartialEq, Ord, Eq)]
pub struct TileCoords {
    pub z: i32,
    pub x: i32,
    pub y: i32,
}

impl TileCoords {
    pub fn new(x: impl Into<i32>, y: impl Into<i32>, z: impl Into<i32>) -> Self {
        TileCoords {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }
    pub fn parent(&self) -> Option<TileCoords> {
        if self.z > 0 {
            return Some(TileCoords::new(
                self.x / 2,
                self.y / 2,
                self.z - 1,
            ));
        } else {
            return None;
        }
    }

    pub fn children(&self) -> [TileCoords; 4] {
        return [
            TileCoords::new(self.x * 2 + 0, self.y * 2 + 0, self.z + 1),
            TileCoords::new(self.x * 2 + 1, self.y * 2 + 0, self.z + 1),
            TileCoords::new(self.x * 2 + 1, self.y * 2 + 1, self.z + 1),
            TileCoords::new(self.x * 2 + 0, self.y * 2 + 1, self.z + 1),
        ];
    }
    /// Generates matrix that transforms 0-8192 x 0-8192 coordinates to -180-180 x -85 - 85 coordinates
    ///of mercator projection
    pub fn matrix(&self) -> ::cgmath::Matrix4<f32> {
        let bounds = self.bounds();

        let w = bounds.width();
        let h = bounds.height();

        let center = bounds.center();

        //println!("{:?} - c: {:?} , B: {:?}", self, center, bounds);

        let translate = ::cgmath::Matrix4::from_translation((center.lng as f32, center.lat as f32, 0.).into());
        let scale = ::cgmath::Matrix4::from_nonuniform_scale(w / EXTENT, h / EXTENT, 1.);
        let center_m = ::cgmath::Matrix4::from_translation((-8192. / 2., -8192. / 2., 0.).into());
        return translate * scale * center_m;
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


#[derive(Debug, Default, Clone, Copy)]
pub struct LatLng {
    pub lat: f64,
    pub lng: f64,
}

impl LatLng {
    pub fn new(lat: impl Into<f64>, lng: impl Into<f64>) -> Self {
        LatLng {
            lat: lat.into(),
            lng: lng.into(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct LatLngBounds {
    pub nw: LatLng,
    pub se: LatLng,
}

impl LatLngBounds {
    fn center(&self) -> LatLng {
        return LatLng {
            lat: (self.nw.lat + self.se.lat) / 2.,
            lng: (self.nw.lng + self.se.lng) / 2.,
        };
    }

    fn width(&self) -> f32 {
        return (self.se.lng - self.nw.lng) as f32;
    }
    fn height(&self) -> f32 {
        return (self.nw.lat - self.se.lat) as f32;
    }
    fn lat_span(&self) -> (f32, f32) {
        return (self.se.lat as _, self.nw.lat as _);
    }
    fn lng_span(&self) -> (f32, f32) {
        return (self.se.lng as _, self.nw.lng as _);
    }
}


#[derive(Default,Clone)]
pub struct Camera {
    /// Camera position in 0-1 scale,
    pub pos: (f32, f32),
    pub zoom: f32,
}

impl Camera {
    pub fn lat_lng(&self) -> LatLng {
        return Mercator::point_to_latlng(self.pos.clone());
    }
    pub fn zoom(&self) -> f32 {
        return self.zoom;
    }
}


pub const MERCATOR_WIDTH: f32 = 360.;
pub const MERCATOR_HEIGHT: f32 = 170.10225;

use std::f32::consts::PI;

pub struct Mercator;

impl Mercator {
    /// Create a matrix that converts coordinates from internal tile coordinates : 8192x 8192
    /// into 1x1 square with 0,0 in top left corner
    pub fn tile_to_internal_matrix(coord: &TileCoords) -> ::cgmath::Matrix4<f32> {
        let n = num::pow(2.0f32, coord.z as usize);
        let w = 1. / n;
        let h = 1. / n;

        let cx = ((coord.x as f32 + 0.5) / n);
        let cy = ((n - (coord.y as f32 + 0.5)) / n);
        //println!("{:?} - c: {:?} , B: {:?}", coord, (cx, cy), (w, h));

        let translate = ::cgmath::Matrix4::from_translation((cx, cy, 0.).into());
        let scale = ::cgmath::Matrix4::from_nonuniform_scale(w / EXTENT, -h / EXTENT, 1.);
        let center_m = ::cgmath::Matrix4::from_translation((-8192. / 2., -8192. / 2., 0.).into());
        return translate * scale * center_m;
    }

    /// Matrix that converts coordinates from linear 1x1 coordinate system
    /// Into screen coordinate system
    /// This matrix must take camera position,zoom and projection into account
    pub fn internal_to_screen_matrix(camera: &Camera) -> ::cgmath::Matrix4<f32> {

        let s = f32::powf(2.0, camera.zoom);
        return
            ::cgmath::Matrix4::from_nonuniform_scale(s, s, 1.) *
                ::cgmath::Matrix4::from_translation((0.5 - camera.pos.0, -0.5 + camera.pos.1, 0.).into())
                * ::cgmath::Matrix4::from_translation((-0.5, -0.5, 0.).into());
    }
    pub fn point_to_latlng(pos: (f32, f32)) -> LatLng {
        unimplemented!()
    }
    pub fn latlng_to_point(pos: LatLng) -> (f32, f32) {
        let DEG2RAD = PI / 180.0;
        let x = ((pos.lng as f32 + 180.) / 360.);
        let y = (1. - f32::ln(f32::tan(pos.lat as f32 * DEG2RAD) + (1. / f32::cos(pos.lat as f32 * DEG2RAD))) / PI) / 2.;
        return (x, y);
    }
}

#[test]
fn test_mercator() {
    assert_eq!((1., 1.), Mercator::latlng_to_point(LatLng::new(0, 0)));
}