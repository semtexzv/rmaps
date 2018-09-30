use ::prelude::*;


use rt_wasm::{self, *};
use stdweb::unstable::TryInto;


pub struct WebHttpClient;

pub struct WebTypes;

pub struct WebCache {}
impl pal::Platform for WebTypes {
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

impl pal::HttpClient for WebHttpClient {
    fn new() -> Result<Self> {
        Ok(WebHttpClient)
    }

    #[allow(unused_must_use)]
    fn execute(&mut self, request: http::Request<bytes::Bytes>) -> BoxFuture<http::Response<bytes::Bytes>, http::Error> {
        use stdweb::web::{
            self,
            IEventTarget,
            XmlHttpRequest,
            XhrReadyState,
        };

        let req = stdweb::web::XmlHttpRequest::new();
        req.open("GET", &request.uri().to_string()).unwrap();
        for (h, v) in request.headers().iter() {
            req.set_request_header(h.as_str(), v.to_str().unwrap());
        }
        let tmp = req.clone();

        js! ( @(no_return) @{tmp}.responseType = "arraybuffer"; );

        req.send_with_bytes(request.into_body().deref()).unwrap();

        let (tx, rx) = futures::sync::oneshot::channel();
        let mut sender = Some(tx);

        let _sent = req.clone();
        _sent.add_event_listener(move |e: web::event::LoadEndEvent| {
            if req.ready_state() == XhrReadyState::Done {
                let tmp = req.clone();
                let data: stdweb::web::ArrayBuffer = js!( return @{tmp}.response; ).try_into().unwrap();
                let data: Vec<u8> = data.into();
                println!("Received something");


                let resp = http::Response::builder().body(bytes::Bytes::from(data));
                if let Some(t) = sender.take() {
                    t.send(resp).unwrap()
                }
            }
        });

        return Box::new(rx.map_err(|_| panic!()).map(|v| v.unwrap()));
    }
}


impl pal::OfflineCache for WebCache {
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