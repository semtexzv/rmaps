use prelude::*;

use ::map::style::{
    self, SourceType,
};

#[derive(Debug, Clone)]
pub struct RasterTileData {
    pub image: Vec<u8>,
    pub dims: (u32, u32),
}

#[derive(Debug, Clone)]
pub struct FeatureGeometry {
    pub vertices: Vec<[f32; 2]>,
    pub indices: Vec<i16>,
}

#[derive(Debug, Clone)]
pub struct VectorTileLayer {
    pub layer: ::mvt::Layer,
    pub pre_tesselated: BTreeMap<u64, FeatureGeometry>,
}

#[derive(Debug, Clone)]
pub struct VectorTileData {
    pub layers: Vec<VectorTileLayer>
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
    pub source_type: SourceType,
}


impl Message for DecodeTile {
    type Result = Result<TileData>;
}


use common::actix::prelude::*;
use common::actix::fut::*;

impl Handler<DecodeTile> for TileDataWorker {
    type Result = ResponseActFuture<Self, TileData, Error>;

    fn handle(&mut self, msg: DecodeTile, ctx: &mut Context<Self>) -> Self::Result {
        use map::style::SourceType;

        match msg.source_type {
            SourceType::Raster => {
                let data = &msg.res.data;
                let format = ::image::guess_format(data).unwrap();
                let decoded = ::image::load_from_memory_with_format(data, format).unwrap().to_rgba();
                let dims = decoded.dimensions();
                let data = RasterTileData {
                    image: decoded.into_raw(),
                    dims,
                };

                return box ok(TileData {
                    coord: msg.res.req.tile_data().unwrap().coords,
                    data: DecodedTileData::Raster(data),
                    source: msg.source_name,
                });
            }
            SourceType::Vector => {
                use mvt::prost::Message;

                let data = &msg.res.data;
                let vt = match ::mvt::vector_tile::Tile::decode(data.clone()) {
                    Ok(t) => t,
                    Err(e) => {
                        panic!("Decoding error : {:?} on {:?}", e, msg.res);
                    }
                };

                let tile = ::mvt::Tile::from(vt);

                let layers = tile.layers.into_iter().map(|l| {
                    let mut pretess = BTreeMap::new();

                    let mult = EXTENT as f32 / l.extent as f32;

                    for f in l.features.iter() {
                        if f.typ == ::mvt::GeomType::Polygon {
                            let geom = &f.geom;

                            let g: Vec<Vec<[f32; 2]>> = f.geom.iter().map(|r| r.iter().map(|p| [p[0] as _, p[1] as _]).collect()).collect();

                            let zer = 0 as _;
                            let ext = l.extent as _;
                            let sq = vec![vec![
                                [zer, zer],
                                [ext, zer],
                                [ext, ext],
                                [zer, ext],
                            ]];


                            if let Ok(res) = ::tess2::intersect(&g, &sq) {
                                let mut g: FeatureGeometry = FeatureGeometry {
                                    vertices: res.vertices.into_iter().map(|[x, y]| [x * mult, y * mult]).collect(),
                                    indices: res.indices.into_iter().map(|x| x as _).collect(),
                                };
                                pretess.insert(f.id, g);
                            }
                        }
                    }

                    VectorTileLayer {
                        pre_tesselated: pretess,
                        layer: l,
                    }
                }).collect();

                return box ok(TileData {
                    coord: msg.res.req.tile_data().unwrap().coords,
                    data: DecodedTileData::Vector(VectorTileData {
                        layers: layers,
                    }),
                    source: msg.source_name,
                });
            }
            _ => {
                panic!("Unhandled source decoding error")
            }
        };
    }
}


impl TileDataWorker {
    pub fn new() -> Self {
        return TileDataWorker();
    }
    pub fn spawn() -> Addr<TileDataWorker> {
        start_in_thread(|| {
            Self::new()
        })
    }
}

