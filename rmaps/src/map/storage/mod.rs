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
    pub callback: Recipient<ResourceResponse>,
}

impl ResourceRequest {
    pub fn new(req: Request, callback: Recipient<ResourceResponse>) -> Self {
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
    local: Addr<local::LocalFileSource>,
    network: Addr<network::NetworkFileSource>,

    requests: BTreeMap<String, Recipient<ResourceResponse>>,
}

impl Actor for DefaultFileSource {
    type Context = Context<DefaultFileSource>;
}

impl Handler<ResourceRequest> for DefaultFileSource {
    type Result = ();

    fn handle(&mut self, msg: ResourceRequest, _ctx: &mut Context<Self>) {
        let url = { msg.request.url().to_string() };
        println!("Req : {:?}", url);
        if let Some(res) = self.cache.get(&msg.request).unwrap() {
            println!("Resp DB");
            spawn(msg.callback.send(ResourceResponse {
                request: msg.request,
                result: Ok(res),
            }));
            return;
        }


        let addr: Addr<_> = _ctx.address();
        let mut recipient: Recipient<_> = addr.recipient();


        if url.starts_with("file://") || url.starts_with("local://") {
            println!("Req local");
            self.requests.insert(msg.request.url(), msg.callback);
            spawn(self.local.send(ResourceRequest { request: msg.request, callback: recipient }));
        } else if url.starts_with("http://") || url.starts_with("https://") {
            println!("Req network");
            self.requests.insert(msg.request.url(), msg.callback);
            spawn(self.network.send(ResourceRequest { request: msg.request, callback: recipient }));
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

    pub fn spawn() -> Addr<Self> {
        start_in_thread::<DefaultFileSource, _>(|| DefaultFileSource::new())
    }
}

