use prelude::*;

pub mod resource;

pub mod local;
pub mod network;
pub mod offline_cache;
mod url;

pub use self::resource::*;

use common::actix::prelude::{ResponseActFuture, ResponseFuture, ActorFuture};
use common::actix::fut::{
    wrap_future, ok, err, result,
};

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


pub struct DefaultFileSource {
    cache: offline_cache::OfflineCache,
    local: Addr<local::LocalFileSource>,
    network: Addr<network::NetworkFileSource>,
}

impl Actor for DefaultFileSource {
    type Context = Context<DefaultFileSource>;
}


impl Handler<Request> for DefaultFileSource {
    type Result = ResponseActFuture<Self, Resource, ResourceError>;

    fn handle(&mut self, msg: Request, _ctx: &mut Context<Self>) -> Self::Result {
        let url = msg.url().to_string();
        trace!("DefaultFileSource: Requesting {:?}", url);

        if let Ok(Some(res)) = self.cache.get(&msg) {
            trace!("DefaultFileSource: Returning from cache");

            return box ok(res);
        }

        use common::actix::fut::FutureWrap;
        let sent: FutureWrap<Box<Future<Item=_, Error=_>>, Self> = if url.starts_with("file://") || url.starts_with("local://") {
            wrap_future(box self.local.send(msg.clone()))
        } else if url.starts_with("http://") || url.starts_with("https://") {
            wrap_future(box self.network.send(msg.clone()))
        } else {
            panic!("No data source available for {:?}", url);
        };

        let proxy_future = box sent
            .map_err(|e, _, _| ResourceError::Other(e.into()))
            .and_then(|res, this: &mut Self, ctx| {
                match res {
                    Ok(data) => {
                        this.cache.put(&data).unwrap();
                        trace!("DefaultFileSource: Returning after caching");
                        return ok(data);
                    }
                    Err(e) => {
                        return err(e);
                    }
                }
            });

        let p = box proxy_future;
        return p;
    }
}


impl DefaultFileSource {
    pub fn new() -> Self {
        DefaultFileSource {
            // TODO: Better location selection
            cache: offline_cache::OfflineCache::new("./tile-data/cache.db").unwrap(),
            local: local::LocalFileSource::spawn(),
            network: network::NetworkFileSource::spawn(),
        }
    }

    pub fn spawn() -> Addr<Self> {
        start_in_thread::<DefaultFileSource, _>(|| DefaultFileSource::new())
    }
}

