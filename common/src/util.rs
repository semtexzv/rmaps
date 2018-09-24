use prelude::*;

use coord::*;
use mercator::*;

/// Structure holding information about Tile IDs needed to cover certain area at zoom level.
/// This structure is used to calculate which tiles need to be rendered
#[derive(Debug, Clone)]
pub struct TileCover(pub BTreeSet<UnwrappedTileCoords>);

impl TileCover {
    fn new_raw(tl: WorldPoint, tr: WorldPoint, br: WorldPoint, bl: WorldPoint, z: i32) -> Self {

        fn float_step<F: FnMut(f64)>(min: f64, max: f64, step: f64, mut op: F) {
            let mut v = min;
            while v <= max {
                (&mut op)(v);
                v += step
            }
        }

        let tiles: i32 = 1 << z;

        let mut cover = BTreeSet::new();
        const SAMPLES: f64 = 32.;

        float_step(tl.x, br.x, (br.x - tl.x) / SAMPLES, |x| {
            float_step(tl.y, br.y, (br.y - tl.y) / SAMPLES, |y| {
                //let pt: geo::Point<_> = (x, y).into();
                let tile = WorldPoint::new(x, y).tile_at_zoom(z);
                let xx = x * tiles as f64;
                let yy = y * tiles as f64;
                let tx = xx.floor() as i32; //if xx < 0. { xx.ceil() } else { xx.floor() }  as i32;
                let ty = yy.floor() as i32;

                cover.insert(UnwrappedTileCoords {
                    x: tx,
                    y: ty,
                    z: z,
                });
            });
        });

        let mut cover = cover.into_iter()
            .filter(|t| t.y >= 0 && t.y < tiles)
            .collect();


        TileCover(cover)
    }

    pub fn from_bounds(bounds: &LatLngBounds, zoom: i32) -> Self {
        use mercator::Mercator;
        let mut tl = Mercator::latlng_to_world(bounds.nw());
        let mut tr = Mercator::latlng_to_world(bounds.ne());
        let mut br = Mercator::latlng_to_world(bounds.se());
        let mut bl = Mercator::latlng_to_world(bounds.sw());

        TileCover::new_raw(tl, tr, br, bl, zoom)
    }

    pub fn from_camera(camera: &Camera) -> Self {
        let mut size = camera.size();
        let z = camera.zoom_int();
        let w = size.w;
        let h = size.h;

        let mut tl = camera.window_to_world(PixelPoint::new(0, 0));
        let mut tr = camera.window_to_world(PixelPoint::new(w, 0));
        let mut br = camera.window_to_world(PixelPoint::new(w, h));
        let mut bl = camera.window_to_world(PixelPoint::new(0, h));

        TileCover::new_raw(tl, tr, br, bl, z)
    }

    pub fn tiles(&self) -> BTreeSet<UnwrappedTileCoords> {
        self.0.clone()
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

    pub fn clamp_lon(&mut self) {
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

    pub fn nw(&self) -> LatLng {
        self.nw
    }

    pub fn ne(&self) -> LatLng {
        LatLng::new(self.nw.lat, self.se.lng)
    }

    pub fn se(&self) -> LatLng {
        self.se
    }

    pub fn sw(&self) -> LatLng {
        LatLng::new(self.se.lat, self.nw.lng)
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
    pub bearing: f32,

    pub zoom: f32,
}

impl Camera {
    pub fn lat_lng(&self) -> LatLng {
        return Mercator::world_to_latlng(self.pos.clone());
    }

    pub fn zoom(&self) -> f32 {
        return self.zoom;
    }
    pub fn zoom_int(&self) -> i32 {
        (self.zoom.round() + 1.) as i32
    }
    pub fn set_zoom(&mut self, z: f32) {
        self.zoom = z;
        if self.zoom < 0. {
            self.zoom = 0.;
        }
    }

    pub fn bearing(&self) -> f32 {
        self.bearing
    }

    pub fn set_bearing(&mut self, v: f32) {
        self.bearing = v
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
        self.pos.y = f64::min(f64::max(0., self.pos.y), 1.);
    }

    pub fn projection(&self) -> Matrix4<f32> {
        Mercator::projection(&self) * Matrix4::from_angle_z(::cgmath::Rad(self.bearing))
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
            bearing: 0.,
            zoom: 0.,
        }
    }
}

