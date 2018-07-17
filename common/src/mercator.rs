use ::prelude::*;

use util::{LatLng, LatLngBounds,Camera};
use coord::*;



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