use ::prelude::*;

pub mod raster;
pub mod vector;

use map::{
    MapViewImpl,
    storage::{
        ResourceRequest, ResourceResponse, DefaultFileSource,
    },
    style::{
        SourceType, StyleSource,
    },
    tiles::data::{
        self, TileReady, DecodedTileData, DecodeTile,
    },
};

pub struct TileRequest {
    coords: Vec<TileCoords>,
    cb: Recipient<TileReady>,
}

impl Message for TileRequest {
    type Result = ();
}

pub trait Source: Actor + Handler<TileRequest> {
    fn id(&self) -> &str;
}

pub trait SourceExt: Source {
    const STYLE_TYPE: SourceType;

    fn from_style(id: String, style: &Rc<StyleSource>, file_source: Addr<DefaultFileSource>) -> Self;
}

pub struct SourceHandle {
    addr: Recipient<TileRequest>
}

impl SourceHandle {}
