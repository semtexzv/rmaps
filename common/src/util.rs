use prelude::*;
use coord::*;
use cgmath::{SquareMatrix, Matrix4, Vector4};

#[derive(Debug, Clone, Copy, Hash, PartialOrd, PartialEq, Ord, Eq)]
pub struct TileCoords {
    pub z: i32,
    pub y: i32,
    pub x: i32,
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

/// Structure holding information about Tile IDs needed to cover certain area at zoom level.
/// This structure is used to calculate which tiles need to be rendered
#[derive(Debug, Clone)]
pub struct TileCover {
    tiles: Vec<TileCoords>
}

impl TileCover {
    fn new_raw(tl: WorldPoint, tr: WorldPoint, br: WorldPoint, bl: WorldPoint, z: i32) -> Self {
        use geo::prelude::*;
        let ring: Vec<geo::Point<f64>> = vec![tl.into(), tr.into(), br.into(), bl.into(), tl.into()];
        let poly = geo::Polygon::<f64>::new(ring.into(), vec![]);
        let bbox: geo::Bbox<f64> = poly.bbox().unwrap();


        fn float_step<F: FnMut(f64)>(min: f64, max: f64, step: f64, mut op: F) {
            let mut v = min;
            while v <= max {
                (&mut op)(v);
                v += step
            }
        }

        let tiles: i32 = 1 << z;

        let center = poly.centroid().unwrap();
        // Expand polygon by 10%, so it will contain extreme screen coordinates
        let exp = poly.map_coords(&|c| {
            ((c.0 - center.x()) * 1.1 + center.x(),
             (c.1 - center.y()) * 1.1 + center.y())
        });


        let mut cover = vec![];

        float_step(bbox.xmin, bbox.xmax, (bbox.xmax - bbox.xmin) / 8., |x| {
            float_step(bbox.ymin, bbox.ymax, (bbox.ymax - bbox.ymin) / 8., |y| {
                let pt: geo::Point<_> = (x, y).into();
                if exp.contains(&pt) {
                    let xx = x * tiles as f64;
                    let yy = y * tiles as f64;
                    // Integer coordinates, we need to override rounding for negative numbers
                    // TODO: Verify whether floor operation on all shouldnt be better
                    let tx = xx.floor() as i32;
                    let ty = yy.floor() as i32;

                    //info!("Should add pt : {:?}, {:?}", (x * tiles as f64) as i32, (y * tiles as f64) as i32);
                    cover.push(TileCoords {
                        x: tx,
                        y: ty,
                        z: z,
                    });
                }
            });
        });

        { cover.retain(|&x| x.y >= 0 && x.y < tiles); }
        { cover.sort_by(|a, b| a.partial_cmp(b).unwrap_or(::std::cmp::Ordering::Less)); }
        { cover.dedup_by(|a, b| a.x == b.x && a.y == b.y); }

        println!("Cover: {:#?}", cover);

        TileCover {
            tiles: vec![]
        }
    }
    /*
    pub fn from_camera(camera: &Camera) -> Self {
        let mut size = camera.size();
        let z = camera.zoom.ceil() as i32;
        let w = size.w;
        let h = size.h;

        let mut tl = camera.window_to_world(PixelPoint::new(0, 0));
        let mut tr = camera.window_to_world(PixelPoint::new(w, 0));
        let mut br = camera.window_to_world(PixelPoint::new(w, h));
        let mut bl = camera.window_to_world(PixelPoint::new(0, h));
        let mut c = camera.window_to_world(PixelPoint::new(w / 2., h / 2.));

        tl.y = 1. - tl.y;
        tr.y = 1. - tr.y;
        br.y = 1. - br.y;
        bl.y = 1. - bl.y;
        c.y = 1. - c.y;
        //mem::swap(&mut tl.y, &mut bl.y);
        //mem::swap(&mut tr.y, &mut br.y);


        println!("TL : {:?}", tl);
        println!("TR : {:?}", tr);
        println!("BR : {:?}", br);
        println!("BL : {:?}", bl);

        println!("Camera x in : [{:?}  ===  {:?}]", tl.x,br.x);
        println!("Camera y in : [{:?}  ===  {:?}]", tl.y,br.y);

        let cover = TileCover::new_raw(tl, tr, br, bl, c, z);
        println!("Cover: {:#?}", cover);

        TileCover {
            tiles: vec![]
        }
    }
    */

    pub fn from_camera(camera: &Camera) -> Self {

        let mut size = camera.size();
        let z = camera.zoom.ceil() as i32;
        let w = size.w;
        let h = size.h;

        let mut tl = camera.window_to_world(PixelPoint::new(0, 0));
        let mut tr = camera.window_to_world(PixelPoint::new(w, 0));
        let mut br = camera.window_to_world(PixelPoint::new(w, h));
        let mut bl = camera.window_to_world(PixelPoint::new(0, h));

        TileCover::new_raw(tl, tr, br, bl, z)
    }
}


#[derive(Debug, Default, Clone, Copy, PartialOrd, PartialEq)]
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

    fn clamp_lon(&mut self) {
        if self.lng > MAX_LON {
            self.lng = MAX_LON;
        }

        if self.lng < MIN_LON {
            self.lng = MIN_LON
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

use std::cell::Cell;

#[derive(Debug, Clone)]
pub struct Camera {
    pub window_size: PixelSize,
    /// Camera position in 0-1 scale,
    pub pos: WorldPoint,

    pub zoom: f32,
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
    }

    pub fn size(&self) -> PixelSize {
        return self.window_size;
    }
    pub fn set_size(&mut self, s: PixelSize) {
        self.window_size = s;
    }

    pub fn pos(&self) -> WorldPoint {
        self.pos
    }
    pub fn set_pos(&mut self, pos: WorldPoint) {
        self.pos = pos;
    }

    pub fn projection(&self) -> Matrix4<f32> {
        Mercator::projection(&self)
    }
    pub fn view(&self) -> Matrix4<f32> {
        Mercator::world_to_screen(&self)
    }

    pub fn inverse_view_projection(&self) -> Matrix4<f32> {
        (self.projection() * self.view()).invert().unwrap()
    }
    #[inline]
    pub fn window_to_world(&self, point: PixelPoint) -> WorldPoint {
        let a = self.window_to_device(point);
        return self.device_to_world(a);
    }

    #[inline]
    pub fn window_to_device(&self, point: PixelPoint) -> DevicePoint {
        let multiplied = (point.x / self.size().w - 0.5, point.y / self.size().h - 0.5);

        DevicePoint::new(2. * multiplied.0 as f32, 2. * -multiplied.1 as f32)
    }

    #[inline]
    pub fn device_to_world(&self, point: DevicePoint) -> WorldPoint {
        let pt = self.inverse_view_projection() * Vector4::new(point.x as f32, point.y as f32, 0., 1.);

        WorldPoint::new(pt.x, pt.y)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            window_size: Default::default(),
            pos: WorldPoint::new(0.5, 0.5),
            zoom: 0.,
        }
    }
}


use std::f64::consts::PI;

pub const MERCATOR_WIDTH: f64 = 360.;
pub const MERCATOR_HEIGHT: f64 = 170.10225751;
//f32::atan(f32::sinh(PI)) * 180. / PI;

pub const MAX_LAT: f64 = MERCATOR_HEIGHT / 2.;
pub const MAX_LON: f64 = MERCATOR_WIDTH / 2.;

pub const MIN_LAT: f64 = -MERCATOR_HEIGHT / 2.;
pub const MIN_LON: f64 = -MERCATOR_WIDTH / 2.;

const DEG2RAD: f64 = PI / 180.0;
const RAD2DEG: f64 = 180.0 / PI;

pub struct Mercator;
/*

Coordinjate systems used:

TILE coords  : mapbox tile coordinates , 0,0 in top left corner, 8192,8192 in bottom right
WORLD/point coords : coordinates in internal world representation , 0,0 in top left, 1,1 in bottom right of tile 0,0,0

SCREEN : Coordinates in  max (-1,1) range, with smaller dimension(width or height) having smaller range, used to preserve
aspect-ratio rendering

DEVICE : Normalized device coordinates


*/
impl Mercator {
    /// Create a matrix that converts coordinates from internal tile coordinates : 8192x 8192
    /// into 1x1 square with 0,0 in top left corner
    pub fn tile_to_world(coord: &TileCoords) -> ::cgmath::Matrix4<f32> {
        let n = num::pow(2.0f32, coord.z as usize);
        let w = 1. / n;
        let h = 1. / n;

        let cx = ((coord.x as f32 + 0.5) / n);
        let cy = (((coord.y as f32 + 0.5)) / n);
        //println!("{:?} - c: {:?} , B: {:?}", coord, (cx, cy), (w, h));

        let translate = ::cgmath::Matrix4::from_translation((cx, cy, 0.).into());
        let scale = ::cgmath::Matrix4::from_nonuniform_scale(w / EXTENT, h / EXTENT, 1.);
        let center_m = ::cgmath::Matrix4::from_translation((-8192. / 2., -8192. / 2., 0.).into());
        return translate * scale * center_m;
    }

    /// Matrix that converts coordinates from linear 1x1 coordinate system
    /// Into screen coordinate system
    /// This matrix must take camera position,zoom into account, projection is separate
    pub fn world_to_screen(camera: &Camera) -> ::cgmath::Matrix4<f32> {
        let s = f32::powf(2.0, camera.zoom);

        // Internal coordinate system : 0,0 in top left corner of root tile
        // 1,1 in bottom right
        let trans = ::cgmath::Matrix4::from_translation((-camera.pos.x as f32, -camera.pos.y as f32, 0.).into());
        // after translation, coordinates are in Internal coordinate system scale, but are relative to camera.
        let scale = ::cgmath::Matrix4::from_nonuniform_scale(s, -s, 1.);
        // After scaling, we are in screen coordinates, 0.0 is center, zoom is applied

        return scale * trans;
    }

    pub fn screen_to_device(camera: &Camera) -> ::cgmath::Matrix4<f32> {
        let PixelSize { w, h } = camera.size();
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
        projection
    }

    pub fn projection(camera: &Camera) -> ::cgmath::Matrix4<f32> {
        Mercator::screen_to_device(camera)
    }

    pub fn world_to_latlng(pos: WorldPoint) -> LatLng {
        let lon_deg = pos.x * 360. - 180.;
        let lat_rad = f64::atan(f64::sinh(PI * (1. - 2. * pos.y)));
        let lat_deg = lat_rad * RAD2DEG;
        LatLng::new(lat_deg, lon_deg)
    }
    pub fn latlng_to_world(mut pos: LatLng) -> WorldPoint {
        pos.clamp_lon();
        let x = ((pos.lng as f64 + 180.) / 360.);
        let y = (1. - f64::ln(f64::tan(pos.lat as f64 * DEG2RAD) + (1. / f64::cos(pos.lat as f64 * DEG2RAD))) / PI) / 2.;
        return WorldPoint::new(x, y);
    }
}

#[test]
fn test_mercator() {
    fn test_mercator(lat: f64, lon: f64, x: f64, y: f64) {
        macro_rules! flt_eq {
            ($a:expr, $b:expr, $eps:expr) => {
                assert!(($a-$b).abs() < $eps, " {} != {}", $a , $b)
            }
        }
        let eps = 0.000000001;

        let c1 = LatLng::new(lat, lon);
        let center = Mercator::latlng_to_world(c1);
        flt_eq!(center.x,x,eps);
        flt_eq!(center.y,y,eps);
        let recenter = Mercator::world_to_latlng(center);

        flt_eq!(c1.lat, recenter.lat,eps);
        flt_eq!(c1.lng, recenter.lng,eps);
    }


    test_mercator(0., 0., 0.5, 0.5);
    // Bottom right,
    // latitude is up + , down -
    // Longitude is right + , left _
    test_mercator(MIN_LAT, MAX_LON, 1., 1.);
    test_mercator(MAX_LAT, MIN_LON, 0., 0.);

    // Test positive wrapping
    test_mercator(MIN_LAT, MAX_LON * 3., 2., 1.);
    // Test negative wrapping
    test_mercator(MIN_LAT, MIN_LON * 2., -0.5, 1.);
}