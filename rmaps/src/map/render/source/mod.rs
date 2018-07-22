use ::prelude::*;

pub mod raster;
pub mod vector;

pub trait Source: Actor {}
/*
pub enum SourceHandle {
    //Raster(Addr<raster::RasterSource>),
    Vector(Addr<vector::VectorSource>),
}*/