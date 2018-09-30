use ::prelude::*;

use common::export::*;

use common::http;

pub struct DesktopHttpClient;

pub struct DesktopTypes;

pub struct SqliteCache {}

use rmaps::map::{
    pal,
    storage,
};


impl pal::Platform for DesktopTypes {
    type HttpClientType = DesktopHttpClient;
    type OfflineCacheType = SqliteCache;

    fn spawn_actor<A, F>(create: F) -> Addr<A> where A: Actor<Context=Context<A>> + Send + 'static,
                                                     F: FnOnce() -> A + Send + 'static {
        use actix::Context;

        let ctx = Context::<A>::new();
        let act = create();

        return ctx.run(act);
    }
}

/* Spawn actor into thread

let (tx, rx) = ::std::sync::mpsc::channel();
        ::std::thread::spawn(move || {
            let sys = System::new("");

            let actor = create();
            let addr = actor.start();
            let _ = tx.send(addr);
            let _ = sys.run();
        });

        rx.recv().unwrap()

        */

use actix_web::{
    client,
    HttpMessage,
};


impl pal::HttpClient for DesktopHttpClient {
    fn new() -> Result<Self> {
        Ok(DesktopHttpClient)
    }

    fn execute(&mut self, request: http::Request<bytes::Bytes>) -> Box<Future<Item=http::Response<bytes::Bytes>, Error=http::Error>> {
        use actix_web::HttpMessage;

        let req = client::get(request.uri().to_string()).body(request.into_body()).unwrap();
        let sent = req.send();
        let resp = sent.then(|resp| {
            let r = resp.unwrap();
            let status = r.status();
            r.body().then(move |body|
                http::Response::builder()
                    .status(status)
                    .body(body.unwrap().into())
            )
        });


        Box::new(resp)
    }
}


impl pal::OfflineCache for SqliteCache {
    fn new() -> Result<Self> {
        Ok(SqliteCache {})
    }

    fn get(&self, req: &storage::Request) -> Result<Option<storage::Resource>> {
        Ok(None)
    }

    fn put(&self, res: &storage::Resource) -> Result<()> {
        Ok(())
    }
}