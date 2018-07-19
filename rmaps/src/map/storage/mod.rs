use prelude::*;

pub mod resource;

pub mod local;
pub mod network;
pub mod offline_cache;
mod url;

pub use self::resource::*;

#[derive(Debug, Fail)]
pub enum ResourceError {
    #[fail(display = "Resource not found")]
    NotFound,
    #[fail(display = "Too many requests")]
    RateLimited,
    #[fail(display = "Other errror: {}", 0)]
    Other(Error),
}

pub type ResourceResult = StdResult<Resource, ResourceError>;

#[derive(Debug)]
pub struct ResourceResponse {
    pub request: Request,
    pub result: StdResult<Resource, ResourceError>,
}

impl Message for ResourceResponse {
    type Result = ();
}

pub struct ResourceRequest {
    pub request: Request,
    pub callback: Recipient<Syn, ResourceResponse>,
}

impl ResourceRequest {
    pub fn new(req: Request, callback: Recipient<Syn, ResourceResponse>) -> Self {
        ResourceRequest {
            request: req,
            callback,
        }
    }
}

impl Message for ResourceRequest {
    type Result = ();
}

pub struct DefaultFileSource {
    cache: offline_cache::OfflineCache,
    local: SyncAddr<local::LocalFileSource>,
    network: SyncAddr<network::NetworkFileSource>,

    requests: BTreeMap<String, Recipient<Syn, ResourceResponse>>,
}

impl Actor for DefaultFileSource {
    type Context = Context<DefaultFileSource>;
}

impl Handler<ResourceRequest> for DefaultFileSource {
    type Result = ();

    fn handle(&mut self, msg: ResourceRequest, _ctx: &mut Context<Self>) {
        let url = { msg.request.url().to_string() };

        if let Some(res) = self.cache.get(&msg.request).unwrap() {
            spawn(msg.callback.send(ResourceResponse {
                request: msg.request,
                result: Ok(res),
            }));
            return;
        }


        let mut recipient = _ctx.sync_address().recipient();


        if url.starts_with("file://") || url.starts_with("local://") {
            self.requests.insert(msg.request.url(), msg.callback);
            self.local.send(ResourceRequest { request: msg.request, callback: recipient }).wait().unwrap();
        } else if url.starts_with("http://") || url.starts_with("https://") {
            self.requests.insert(msg.request.url(), msg.callback);
            self.network.send(ResourceRequest { request: msg.request, callback: recipient }).wait().unwrap();
        } else {
            panic!("No data source available for {:?}", url);
        }
    }
}

impl Handler<ResourceResponse> for DefaultFileSource {
    type Result = ();

    fn handle(&mut self, msg: ResourceResponse, _ctx: &mut Context<Self>) {
        if let Some(cb) = self.requests.remove(&msg.request.url()) {
            {
                if let Ok(ref resource) = msg.result {
                    self.cache.put(resource).unwrap();
                }
            }
            cb.send(msg).wait().unwrap();
        }
    }
}


impl DefaultFileSource {
    pub fn new() -> Self {
        DefaultFileSource {
            // TODO: Better location selection
            cache: offline_cache::OfflineCache::new("./tile-data/cache.db").unwrap(),
            local: local::LocalFileSource::spawn(),
            network: network::NetworkFileSource::spawn(),
            requests: BTreeMap::new(),
        }
    }

    pub fn spawn() -> Addr<Syn, Self> {
        start_in_thread::<DefaultFileSource, _>(|| DefaultFileSource::new())
    }
}

