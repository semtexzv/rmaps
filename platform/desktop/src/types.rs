use ::prelude::*;

use common::export::*;

use common::http;

pub struct DesktopHttpClient;

pub struct DesktopTypes;

pub struct SqliteCache {}

use rmaps::map::{
    interop,
    storage,
};


impl interop::Types for DesktopTypes {
    type HttpClientType = DesktopHttpClient;
    type OfflineCacheType = SqliteCache;
}

impl interop::HttpClient for DesktopHttpClient {
    fn new() -> Result<Self> {
        Ok(DesktopHttpClient)
    }

    fn execute<T>(&mut self, request: http::Request<T>) -> Box<Future<Item=http::Response<T>, Error=http::Error>> {
        unimplemented!()
    }
}


impl interop::OfflineCache for SqliteCache {
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