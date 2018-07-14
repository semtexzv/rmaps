use prelude::*;
use coord::*;

use cgmath::{SquareMatrix, Matrix4, Vector4};

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

    pub fn id(&self) -> u8 {
        let x = (self.x & 0xFF) as u8;
        let y = (self.y & 0xFF) as u8;
        let z = (self.z & 0xFF) as u8;

        return z & 0b11 << 6 | x & 0b111 << 3 | y & 0b111;
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

#[derive(Debug, Clone)]
pub struct Camera {
    pub window_size: PixelSize,
    /// Camera position in 0-1 scale,
    pub pos: WorldPoint,
    pub zoom: f32,

    projection: Option<::cgmath::Matrix4<f32>>,
    view: Option<::cgmath::Matrix4<f32>>,
}

impl Camera {
    pub fn lat_lng(&self) -> LatLng {
        return Mercator::world_to_latlng(self.pos.clone());
    }

    pub fn zoom(&self) -> f32 {
        return self.zoom;
    }
    pub fn set_zoom(&mut self, z: f32) {
        self.zoom = z;
        if self.zoom < 0. {
            self.zoom = 0.;
        }
        self.view = None;
    }

    pub fn size(&self) -> PixelSize {
        return self.window_size;
    }
    pub fn set_size(&mut self, s: PixelSize) {
        self.window_size = s;
        self.projection = None;
    }

    pub fn pos(&self) -> WorldPoint {
        self.pos
    }
    pub fn set_pos(&mut self, pos: WorldPoint) {
        self.pos = pos;
        trace!("Camera position changed to : {:?}", pos);
        self.view = None;
    }


    pub fn projection(&mut self) -> Matrix4<f32> {
        return if let Some(ref mut p) = self.projection {
            p.clone()
        } else {
            let PixelSize { w, h } = self.size();
            let scale = w as f32 / h as f32;

            let (xs, ys) = if scale <= 1. {
                (scale, 1.)
            } else {
                (1., 1. / scale)
            };
            let (wh, hh) = (xs / 2., ys / 2.);
            let projection = ::cgmath::ortho(
                -wh, wh,
                -hh, hh,
                -1., 100.);

            self.projection = Some(projection.clone());
            projection
        };
    }

    pub fn view(&mut self) -> Matrix4<f32> {
        return if let Some(ref mut p) = self.view {
            p.clone()
        } else {
            let view = Mercator::internal_to_screen_matrix(&self);
            self.view = Some(view.clone());
            view
        };
    }

    #[inline]
    pub fn window_to_world(&mut self, point: PixelPoint) -> WorldPoint {
        let a = self.window_to_device(point);
        return self.device_to_world(a);
    }

    #[inline]
    pub fn window_to_device(&mut self, point: PixelPoint) -> DevicePoint {
        let multiplied = (point.x / self.size().w - 0.5, point.y / self.size().h - 0.5);

        DevicePoint::new(2. * multiplied.0 as f32, 2. * -multiplied.1 as f32)
    }

    #[inline]
    pub fn device_to_world(&mut self, point: DevicePoint) -> WorldPoint {
        let pt = (self.projection() * self.view()).invert().unwrap() * Vector4::new(point.x as f32, point.y as f32, 0., 1.);

        WorldPoint::new(pt.x, pt.y)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            window_size: Default::default(),
            pos: WorldPoint::new(0.5, 0.5),
            zoom: 0.,
            projection: None,
            view: None,
        }
    }
}


use std::f64::consts::PI;

pub const MERCATOR_WIDTH: f64 = 360.;
pub const MERCATOR_HEIGHT: f64 = 170.10225751;
//f32::atan(f32::sinh(PI)) * 180. / PI;

pub const MAX_LAT: f64 = MERCATOR_HEIGHT / 2.;
pub const MAX_LON: f64 = MERCATOR_WIDTH / 2.;


pub struct Mercator;
/*

Coordinjate systems used:

TILE coords  : mapbox tile coordinates , 0,0 in top left corner, 8192,8192 in bottom right
WORLD/point coords : coordinates in internal world representation , 0,0 in top left


*/
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
    /// This matrix must take camera position,zoom into account, projection is separate
    pub fn internal_to_screen_matrix(camera: &Camera) -> ::cgmath::Matrix4<f32> {
        let s = f32::powf(2.0, camera.zoom);
        return
            ::cgmath::Matrix4::from_nonuniform_scale(s, s, 1.) *
                ::cgmath::Matrix4::from_translation((0.5 - camera.pos.x as f32, 0.5 - camera.pos.y as f32, 0.).into())
                * ::cgmath::Matrix4::from_translation((-0.5, -0.5, 0.).into());
    }

    pub fn world_to_latlng(pos: WorldPoint) -> LatLng {
        unimplemented!()
    }
    pub fn latlng_to_world(pos: LatLng) -> WorldPoint {
        let DEG2RAD = PI / 180.0;
        let x = ((pos.lng as f64 + 180.) / 360.);
        let y = (1. - f64::ln(f64::tan(pos.lat as f64 * DEG2RAD) + (1. / f64::cos(pos.lat as f64 * DEG2RAD))) / PI) / 2.;
        return WorldPoint::new(x, y);
    }
}

#[test]
fn test_mercator() {
    macro_rules! assert_float_eq {
        ($a:expr, $b:expr) => {
            assert_eq!(format!("{:?}",$a), format!("{:?}",$b));
        }
    }

    // CENTER
    assert_float_eq!((0.5f64, 0.5f64), Mercator::latlng_to_point(LatLng::new(0, 0)));

    // TOP LEFT
    // assert_eq!((0., 0.), Mercator::latlng_to_point(LatLng::new(MAX_LAT, -180)));

    // BOTTOM RIGHT
    assert_float_eq!((1f64, 1f64), Mercator::latlng_to_point(LatLng::new(-MAX_LAT, 180)));
}