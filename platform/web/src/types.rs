use ::prelude::*;

pub struct WebHttpClient;

pub struct WebTypes;

pub struct WebCache {}


impl hal::Platform for WebTypes {
    type HttpClientType = WebHttpClient;
    type OfflineCacheType = WebCache;

    fn spawn_actor<A, F>(create: F) -> Addr<A> where A: Actor<Context=Context<A>> + Send + 'static,
                                                     F: FnOnce() -> A + Send + 'static {
        unimplemented!()
        /*
        actix::system::System::current()
        actix::Arbiter::current().send(actix::msgs::StartActor(box || {
            create()
        }))
        */
    }
}

impl hal::HttpClient for WebHttpClient {
    fn new() -> Result<Self> {
        Ok(WebHttpClient)
    }

    fn execute(&mut self, request: http::Request<bytes::Bytes>) -> BoxFuture<http::Response<bytes::Bytes>, http::Error> {
        unimplemented!()
    }
}


impl hal::OfflineCache for WebCache {
    fn new() -> Result<Self> {
        Ok(WebCache {})
    }

    fn get(&self, req: &storage::Request) -> Result<Option<storage::Resource>> {
        Ok(None)
    }

    fn put(&self, res: &storage::Resource) -> Result<()> {
        Ok(())
    }
}