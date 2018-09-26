use ::prelude::*;
use ::common::futures::Future;
use ::common::http;

use map::storage;

pub trait HttpClient: 'static + Send + Sized {
    fn new() -> Result<Self>;
    fn execute<T>(&mut self, request: http::Request<T>) -> BoxFuture<http::Response<T>, http::Error>;
}

pub trait OfflineCache: 'static + Send + Sized {
    fn new() -> Result<Self>;
    fn get(&self, req: &storage::Request) -> Result<Option<storage::Resource>>;
    fn put(&self, res: &storage::Resource) -> Result<()>;
}

pub trait Types: 'static + Send {
    type HttpClientType: HttpClient;
    type OfflineCacheType: OfflineCache;
}