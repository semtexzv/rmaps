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
    pub indices: Vec<u16>,
}

#[derive(Debug, Clone)]
pub struct VectorTileLayer {
    pub layer: ::mvt::Layer,
    pub pre_tesselated: BTreeMap<usize, FeatureGeometry>,
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

impl DecodedTileData {
    pub fn unwrap_vector(&self) -> &VectorTileData {
        if let DecodedTileData::Vector(v) = self {
            return v;
        }
        panic!("Not a Vector")
    }

    pub fn unwrap_raster(&self) -> &RasterTileData {
        if let DecodedTileData::Raster(r) = self {
            return r;
        }
        panic!("Not a raster")
    }
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
                let data = &msg.res.data;
                let tile = ::mvt::decode(&data).unwrap();

                let layers = tile.layers.into_iter().map(|mut l| {
                    let mut pretess = BTreeMap::new();

                    let mult = EXTENT as f32 / l.extent as f32;

                    for (idx, f) in l.features.iter_mut().enumerate() {
                        match f.typ {
                            ::mvt::GeomType::Polygon => {
                                let g = &f.geometry;


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
                                    pretess.insert(idx, g);
                                }
                            }
                            ::mvt::GeomType::LineString => {
                                for l in f.geometry.iter_mut() {
                                    for v in l.iter_mut() {
                                        v[0] = v[0] * mult;
                                        v[1] = v[1] * mult;
                                    }
                                }
                            }
                            _ => {

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
    pub fn spawn<P: hal::Platform>() -> Addr<TileDataWorker> {
        P::spawn_actor(|| {
            Self::new()
        })
    }
}

