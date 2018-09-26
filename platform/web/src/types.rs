use ::prelude::*;

pub struct WebHttpClient;

pub struct WebTypes;

pub struct WebCache {}


impl interop::Types for WebTypes {
    type HttpClientType = WebHttpClient;
    type OfflineCacheType = WebCache;
}

impl interop::HttpClient for WebHttpClient {
    fn new() -> Result<Self> {
        Ok(WebHttpClient)
    }

    fn execute<T>(&mut self, request: http::Request<T>) -> Box<Future<Item=http::Response<T>, Error=http::Error>> {
        unimplemented!()
    }
}


impl interop::OfflineCache for WebCache {
    fn new() -> Result<Self> {
        unimplemented!()
    }

    fn get(&self, req: &storage::Request) -> Result<Option<storage::Resource>> {
        unimplemented!()
    }

    fn put(&self, res: &storage::Resource) -> Result<()> {
        unimplemented!()
    }
}