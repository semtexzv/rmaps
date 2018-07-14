use prelude::*;

use ::map::style;

#[derive(Debug, Clone)]
pub struct RasterTileData {
    pub image: Vec<u8>,
    pub dims: (u32, u32),
}

#[derive(Debug, Clone)]
pub struct VectorTileData {
    pub layers : Vec<::mvt::Layer>

}

#[derive(Debug, Clone)]
pub enum DecodedTileData {
    Vector(VectorTileData),
    Raster(RasterTileData),
}


#[derive(Debug, Clone)]
pub struct TileData {
    pub coord: TileCoords,
    pub source: String,
    pub data: DecodedTileData,
}

pub struct TileDataWorker();

impl Actor for TileDataWorker {
    type Context = Context<Self>;
}


pub struct DecodeTile {
    pub res: ::map::storage::Resource,
    pub source_name: String,
    pub source: ::map::style::StyleSource,
    pub cb: Recipient<Syn, TileReady>,
}

#[derive(Debug, Clone)]
pub struct TileReady {
    pub data: TileData
}


impl Message for DecodeTile {
    type Result = ();
}

impl Message for TileReady {
    type Result = ();
}

impl Handler<DecodeTile> for TileDataWorker {
    type Result = ();

    fn handle(&mut self, msg: DecodeTile, ctx: &mut Context<Self>) {
        let data = match msg.source {
            ::map::style::StyleSource::Raster(_) => {
                let data = &msg.res.data;
                let format = ::image::guess_format(data).unwrap();
                let decoded = ::image::load_from_memory_with_format(data, format).unwrap().to_rgba();
                let dims = decoded.dimensions();
                let data = RasterTileData {
                    image: decoded.into_raw(),
                    dims,
                };

                TileData {
                    coord: msg.res.req.tile_data().unwrap().coords,
                    data: DecodedTileData::Raster(data),
                    source: msg.source_name,
                }
            }
            ::map::style::StyleSource::Vector(_) => {
                use mvt::prost::Message;

                let data = &msg.res.data;
                let vt = ::mvt::vector_tile::Tile::decode(data).unwrap();
                let tile = ::mvt::Tile::from(vt);


                TileData {
                    coord: msg.res.req.tile_data().unwrap().coords,
                    data: DecodedTileData::Vector(VectorTileData {
                        layers : tile.layers,
                    }),
                    source: msg.source_name,
                }
            }
            _ => {
                panic!("Unhandled source decoding error")
            }
        };

        msg.cb.do_send(TileReady {
            data: data,
        }).unwrap();
    }
}


impl TileDataWorker {
    pub fn new() -> Self {
        return TileDataWorker();
    }
    pub fn spawn() -> Addr<Syn, TileDataWorker> {
        start_in_thread(|| {
            Self::new()
        })
    }
}

