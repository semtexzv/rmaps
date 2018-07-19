use prelude::*;

pub mod data;
pub mod source;


use std::collections::BTreeSet;
use super::storage;
use super::style;


pub struct TileLoader {
    pub source: Addr< storage::DefaultFileSource>,
    pub worker: Addr< data::TileDataWorker>,
    pub map: Option<Addr< super::MapViewImpl>>,

    pub active: BTreeMap<(Cow<'static, str>, TileCoords), Arc<style::StyleSource>>,
    pub decoding: BTreeSet<(Cow<'static, str>, TileCoords)>,

    pub addr: Option<Addr< TileLoader>>,
    pub count_resources: usize,
    pub count_requests: usize,
}

impl TileLoader {
    pub fn addr(&self) -> Addr< Self> {
        return self.addr.as_ref().map(|x| x.clone()).unwrap();
    }
    pub fn map(&self) -> Addr< super::MapViewImpl> {
        return self.map.as_ref().map(|x| x.clone()).unwrap();
    }

    pub fn should_request(&self, name: &str, coords: TileCoords) -> bool {
        let pair = (Cow::Borrowed(name), coords);
        return !self.active.contains_key(&pair) && !self.decoding.contains(&pair);
    }
    pub fn new(file_source: Addr< storage::DefaultFileSource>) -> Self {
        let worker = data::TileDataWorker::spawn();
        TileLoader {
            source: file_source,
            worker: worker,
            map: None,
            active: BTreeMap::new(),
            decoding: BTreeSet::new(),
            addr: None,
            count_resources: 0,
            count_requests: 0,
        }
    }
    pub fn spawn(source: Addr< storage::DefaultFileSource>) -> Addr< TileLoader> {
        start_in_thread(|| {
            Self::new(source)
        })
    }

    pub fn request_tile(&mut self, name: &str, source: &Arc<style::StyleSource>, coords: TileCoords) {
        if !self.should_request(&name, coords) {
            return;
        }
        self.count_requests += 1;

        let key = (Cow::Owned(name.to_string()), coords);
        let template = (&source.tile_urls()[0]).to_string();
        let req = storage::resource::Request::tile(name.to_string(), template, coords);
        spawn(
            self.source.send(storage::ResourceRequest::new(req, self.addr().recipient()))
        );

        self.active.insert(key, source.clone());
    }

    pub fn tile_arrived(&mut self, data: &storage::TileRequestData, result: storage::ResourceResult) {
        match result {
            Ok(res) => {
                self.count_resources += 1;
                let key = (Cow::Owned(data.source.to_string()), data.coords);
                let source = self.active.remove(&key).unwrap();

                self.decoding.insert(key);

                let decode = data::DecodeTile {
                    source,
                    source_name: data.source.clone(),
                    cb: self.addr().recipient(),
                    res,
                };

                spawn(self.worker.send(decode));
                println!("req : {:?} \t resp : {:?}", self.count_requests, self.count_resources);
            }
            Err(storage::ResourceError::NotFound) => {
                println!("Not Found : {:?}", result);
            }
            Err(storage::ResourceError::RateLimited) => {
                println!("Rate limited : {:?}", result);
            }
            Err(storage::ResourceError::Other(e)) => {
                error!("Resource Error : {}", e);
            }
        }
    }
}

impl Actor for TileLoader {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut <Self as Actor>::Context) {
        self.addr = Some(ctx.address());
    }
}

impl_invoke_handler!(TileLoader);


impl Handler<storage::ResourceResponse> for TileLoader {
    type Result = ();

    fn handle(&mut self, msg: storage::ResourceResponse, _ctx: &mut Context<Self>) {
        let req_data = msg.request.tile_data().unwrap();
        self.tile_arrived(req_data,msg.result);

    }
}

impl Handler<data::TileReady> for TileLoader {
    type Result = ();

    fn handle(&mut self, msg: data::TileReady, _ctx: &mut Context<Self>) {
        spawn(self.map().send(msg));
        //self.tile_decoded(msg.data);
    }
}