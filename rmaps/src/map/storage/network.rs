use prelude::*;

/*
use common::reqwest::{
    unstable::async::*,
};

*/

use common::http::{
    self,
    StatusCode,
    header::{
        self, HeaderName,
        LOCATION, CONTENT_LOCATION,
    },
};

use map::hal::HttpClient;

/*
use common::actix_web::{
    client::{
        self,
        ClientConnector, ClientRequest, ClientResponse,
    },
    Body,
    HttpMessage,
    HttpResponse,
};
*/

use super::{Request, Resource, ResourceError};


use common::actix::prelude::{ResponseActFuture, ResponseFuture, ActorFuture};
use common::actix::fut::{
    IntoActorFuture,
    WrapFuture,
};

pub struct NetworkFileSource<I: hal::Platform> {
    client: I::HttpClientType
}

impl<I: hal::Platform> Actor for NetworkFileSource<I> {
    type Context = Context<Self>;
}


impl<I: hal::Platform> Handler<Request> for NetworkFileSource<I> {
    type Result = ResponseActFuture<Self, Resource, ResourceError>;

    fn handle(&mut self, msg: Request, _ctx: &mut Context<Self>) -> Self::Result {
        use map::hal::HttpClient;

        fn get<I: hal::Platform>(url: &str, msg: Request, allowed_redirect_count: usize) -> BoxFuture<Resource, ResourceError> {
            let request = http::Request::get(url).body(bytes::Bytes::new()).unwrap();
            let resp_fut = I::HttpClientType::new().unwrap().execute(request);
            box resp_fut.then(move |res: StdResult<_, _>| {
                let response = res.unwrap();
                if response.status().is_success() {
                    let data = response.body();
                    return Ok(super::Resource {
                        req: msg.clone(),
                        cache_until: u64::max_value(),
                        data: data.to_vec(),
                    });
                }
                return Err(ResourceError::NotFound);
            })
        }

        let mut fut =  get::<I>(&msg.url(), msg, 3);
        return box WrapFuture::into_actor(fut, self);
    }
}

impl<I: hal::Platform> NetworkFileSource<I> {
    pub fn new() -> Self {
        return NetworkFileSource {
            client: I::HttpClientType::new().unwrap()
        };
    }
    pub fn spawn() -> Addr<Self> {
        I::spawn_actor(|| NetworkFileSource::new())
    }
}