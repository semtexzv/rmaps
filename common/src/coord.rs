use prelude::*;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialOrd, PartialEq)]
pub struct PixelSize {
    pub w: f64,
    pub h: f64,
}

impl PixelSize {
    pub fn new(w: impl Into<f64>, h: impl Into<f64>) -> Self {
        PixelSize {
            w: w.into(),
            h: h.into(),
        }
    }
}


#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialOrd, PartialEq, Add, Sub)]
pub struct PixelPoint {
    pub x: f64,
    pub y: f64,
}

impl PixelPoint {
    pub fn new(x: impl Into<f64>, y: impl Into<f64>) -> Self {
        PixelPoint {
            x: x.into(),
            y: y.into(),
        }
    }

}


#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialOrd, PartialEq, Add, Sub)]
pub struct WorldPoint {
    pub x: f64,
    pub y: f64,
}

impl WorldPoint {
    pub fn new(x: impl Into<f64>, y: impl Into<f64>) -> Self {
        WorldPoint {
            x: x.into(),
            y: y.into(),
        }
    }
    pub fn scaled(&mut self, f : f64) -> WorldPoint{
        WorldPoint::new(self.x *f , self.y * f)
    }
}


#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialOrd, PartialEq, Add, Sub)]
pub struct DevicePoint {
    pub x: f64,
    pub y: f64,
}

impl DevicePoint {
    pub fn new(x: impl Into<f64>, y: impl Into<f64>) -> Self {
        DevicePoint {
            x: x.into(),
            y: y.into(),
        }
    }
}
