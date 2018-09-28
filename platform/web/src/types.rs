use ::prelude::*;

pub struct WebHttpClient;

pub struct WebTypes;

pub struct WebCache {}


use rt_wasm::{
    self, *,
};


impl hal::Platform for WebTypes {
    type HttpClientType = WebHttpClient;
    type OfflineCacheType = WebCache;

    fn spawn_actor<A, F>(create: F) -> Addr<A> where A: Actor<Context=Context<A>> + Send + 'static,
                                                     F: FnOnce() -> A + Send + 'static {
        use actix::Context;

        let ctx = Context::<A>::new();
        let act = create();

        return ctx.run(act);
    }
}

impl hal::HttpClient for WebHttpClient {
    fn new() -> Result<Self> {
        Ok(WebHttpClient)
    }

    fn execute(&mut self, request: http::Request<bytes::Bytes>) -> BoxFuture<http::Response<bytes::Bytes>, http::Error> {
        use stdweb::web::{
            self,
            IEventTarget,
            XmlHttpRequest,
            XhrReadyState,
        };

        let req = stdweb::web::XmlHttpRequest::new();
        req.open("GET", &request.uri().to_string()).unwrap();
        req.send_with_bytes(request.into_body().deref()).unwrap();

        let (tx, rx) = futures::sync::oneshot::channel();
        let mut ot = Some(tx);

        let req2 = req.clone();
        req2.add_event_listener(move |e: web::event::LoadEndEvent| {
            if req.ready_state() == XhrReadyState::Done {
                println!("Received something : Encoding  : {:?}", req.get_response_header("Content-Encoding:"));
                let txt = req.response_text().unwrap().unwrap();

                let resp = http::Response::builder().body(bytes::Bytes::from(txt));
                if let Some(t) = ot.take() {
                    t.send(resp).unwrap()
                }
            }
        });

        return Box::new(rx.map_err(|_| panic!()).map(|v| v.unwrap()));
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