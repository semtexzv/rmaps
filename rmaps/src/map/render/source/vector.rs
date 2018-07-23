use ::prelude::*;

use super::{
    Source, TileRequest, SourceExt, SourceHandle,
};

use map::{
    MapViewImpl,
    storage::{
        ResourceRequest, ResourceResponse, DefaultFileSource,
    },
    style::{
        SourceType, StyleSource,
    },
    tiles::data::{
        self, TileReady, DecodedTileData, DecodeTile, TileData, TileReady,
    },
};


pub struct VectorSource {
    id: String,
    data: StyleSource,
    file_source: Addr<DefaultFileSource>,
    worker: Addr<data::TileDataWorker>,
    dest: Recipient<TileReady>,

    active: BTreeSet<TileCoords>,
    decoding: BTreeSet<TileCoords>,
}

impl Actor for VectorSource { type Context = Context<Self>; }

impl Handler<TileRequest> for VectorSource {
    type Result = ();

    fn handle(&mut self, msg: TileRequest, ctx: &mut Context<Self>) {
        for coord in msg.coords.iter() {
            if !!self.active.contains(coord) && !self.decoding.contains(coord) {
                //Request
            }
        }
    }
}

impl Handler<ResourceResponse> for VectorSource {
    type Result = ();

    fn handle(&mut self, msg: ResourceResponse, ctx: &mut Context<Self>) {
        // TODO: Update local active tiles, send for decoding
    }
}

impl Handler<TileData> for VectorSource {
    type Result = ();

    fn handle(&mut self, msg: TileData, ctx: &mut Context<Self>) {
        unimplemented!()
    }
}

impl Source for VectorSource {
    fn id(&self) -> &str {
        &self.id
    }
}
/*
impl Source for VectorSource {

}*/