use ::prelude::*;
use ::common::futures::Future;
use ::common::http;

use map::storage;

pub trait HttpClient: 'static + Send + Sized {
    fn new() -> Result<Self>;
    fn execute(&mut self, request: http::Request<bytes::Bytes>) -> BoxFuture<http::Response<bytes::Bytes>, http::Error>;
}

pub trait OfflineCache: 'static + Send + Sized {
    fn new() -> Result<Self>;
    fn get(&self, req: &storage::Request) -> Result<Option<storage::Resource>>;
    fn put(&self, res: &storage::Resource) -> Result<()>;
}

pub trait Platform: 'static + Send {
    type HttpClientType: HttpClient;
    type OfflineCacheType: OfflineCache;

    fn spawn_actor<A, F>(create: F) -> Addr<A>
        where A: Actor<Context=Context<A>> + Send + 'static,
              F: FnOnce() -> A + Send + 'static {
        let (tx, rx) = ::std::sync::mpsc::channel();
        ::std::thread::spawn(move || {
            let sys = System::new("");

            let actor = create();
            let addr = actor.start();
            let _ = tx.send(addr);
            let _ = sys.run();
        });

        rx.recv().unwrap()
    }
}