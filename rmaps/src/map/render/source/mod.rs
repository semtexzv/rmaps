use ::prelude::*;

pub mod raster;
pub mod vector;

use map::{
    MapViewImpl,
    storage::{
        ResourceError, DefaultFileSource, Resource, ResourceResult,
    },
    style::{
        SourceType, StyleSource, Style,
    },
    tiles::{
        self, TileData, DecodedTileData, DecodeTile, TileDataWorker,
    },
};

use common::actix::prelude::*;
use common::actix::fut::*;


pub struct TileRequest {
    pub coords: TileCoords,
}

#[derive(Debug)]
pub enum TileError {
    LoadingTileset,
    Downloading,
    Decoding,
    Resource(ResourceError),
    Error(Error),
}

impl From<Error> for TileError {
    fn from(e: Error) -> Self {
        TileError::Error(e)
    }
}

impl Message for TileRequest {
    type Result = StdResult<TileData, TileError>;
}

pub trait Source: Actor + Handler<TileRequest> {
    fn id(&self) -> &str;

    const STYLE_TYPE: SourceType;

    fn from_style(id: String, style: &Rc<StyleSource>, file_source: Recipient<::map::storage::Request>) -> Self;
}

pub fn parse_sources<P: pal::Platform>(style: &::map::style::Style, file_source: Recipient<::map::storage::Request>) -> BTreeMap<String, Addr<BaseSource>> {
    let mut res = BTreeMap::new();
    for (kk, v) in style.sources.iter() {
        let k: String = kk.clone();
        let v: StyleSource = v.deref().clone();
        let src = BaseSource::new::<P>(k, v, file_source.clone());
        let src = P::spawn_actor(|| { src });
        res.insert(kk.clone(), src);
    }
    res
}

use map::storage::{
    self, Request,
};

pub struct BaseSource {
    id: String,
    style: StyleSource,
    file_source: Recipient<storage::Request>,
    worker: Addr<TileDataWorker>,
    downloading: BTreeSet<TileCoords>,
    decoding: BTreeSet<TileCoords>,
}

impl Actor for BaseSource {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        if let (Some(ref url), None) = (&self.style.url, &self.style.tilejson.tiles.as_ref().and_then(|x| x.first())) {

            // Load tilejson
            let req = Request::source(self.id.clone(), url.to_string());

            trace!("BaseSource: Loading TileJson");
            let fut = wrap_future(self.file_source.send(req))
                .from_err::<Error>()
                .and_then(|res, this: &mut Self, ctx| {
                    if let Ok(data) = res {
                        let tile_json = json::from_slice(&data.data).unwrap();
                        this.style.tilejson = tile_json;
                        trace!("BaseSource: TileJSON loaded");
                    }
                    ok(())
                });

            ctx.spawn(fut.drop_err());
        }
    }
}

impl BaseSource {
    fn new<P: pal::Platform>(id: String, style: StyleSource, file_source: Recipient<storage::Request>) -> Self {
        let worker = P::spawn_actor(|| TileDataWorker::new());
        BaseSource {
            id,
            style: style,
            file_source,
            worker,
            downloading: BTreeSet::new(),
            decoding: BTreeSet::new(),
        }
    }
    fn resource_arrived(&mut self, res: ResourceResult, ctx: &mut Context<Self>) -> ResponseActFuture<Self, TileData, TileError> {
        use map::storage::Request;
        match res {
            Ok(res) => {
                let tile_data = res.req.tile_data().unwrap();
                self.decoding.insert(tile_data.coords);

                let msg = DecodeTile {
                    source_type: self.style.typ.clone(),
                    source_name: self.id.clone(),
                    res: res.clone(),
                };
                let decode = wrap_future(self.worker.send(msg));

                return box decode.from_err::<Error>().from_err::<TileError>()
                    .and_then(|result, this: &mut BaseSource, ctx| {
                        match result {
                            Ok(decoded) => {
                                this.decoding.remove(&decoded.coord);

                                return ok(decoded);
                            }
                            Err(e) => {
                                return err(e.into());
                            }
                        }
                    });
            }
            Ok(_) => {
                return box err(format_err!("Invalid resource arrived").into());
            }
            Err(e) => {
                return box err(TileError::Resource(e));
            }
        }
    }
}


impl Handler<TileRequest> for BaseSource {
    type Result = ResponseActFuture<Self, TileData, TileError>;

    fn handle(&mut self, msg: TileRequest, ctx: &mut Context<Self>) -> Self::Result {
        let t = msg.coords;
        if self.downloading.contains(&t) {
            return box err(TileError::Downloading);
        }
        if self.decoding.contains(&t) {
            return box err(TileError::Decoding);
        }

        if let Some(url) = self.style.tilejson.tiles.as_ref().and_then(|f| f.first().clone()) {
            let req = ::map::storage::Request::tile(self.id.clone(), url.to_string(), t);
            let fut = wrap_future(self.file_source.send(req));
            self.downloading.insert(t);
            box fut.from_err::<Error>().from_err::<TileError>()
                .and_then(|res, this: &mut BaseSource, ctx| {
                    this.resource_arrived(res, ctx)
                })
        } else {
            return box err(TileError::LoadingTileset);
        }
    }
}