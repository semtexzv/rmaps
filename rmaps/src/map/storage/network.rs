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

use map::interop::HttpClient;

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

use common::futures::future::*;

pub struct NetworkFileSource<I: interop::Types> {
    client: I::HttpClientType
}

impl<I: interop::Types> Actor for NetworkFileSource<I> {
    type Context = Context<Self>;
}


impl<I: interop::Types> Handler<Request> for NetworkFileSource<I> {
    type Result = ResponseActFuture<Self, Resource, ResourceError>;

    fn handle(&mut self, msg: Request, _ctx: &mut Context<Self>) -> Self::Result {
        println!("NetworkFileSource : requesting {:?}", msg.url());
        return unimplemented!();
        /*
        fn get<I: interop::Types>(url: &str, msg: Request, allowed_redirect_count: usize) -> Box<dyn Future<Item=Resource, Error=ResourceError>> {
            let request = http::Request::get(url).body(()).unwrap();
            let resp_fut = I::HttpClientType::new().execute(request);
        }

        let request: Box<dyn Future<Item=_, Error=ResourceError>> = Box::new(client::get(url)
            .timeout(::std::time::Duration::from_secs(15))
            .finish().unwrap()
            .send()
            .timeout(::std::time::Duration::from_secs(15))
            .map_err(|e| ResourceError::Other(e.into())));

        let parse_response = move |response: ClientResponse| -> Box<dyn Future<Item=Resource, Error=ResourceError>> {
            if response.status().is_redirection() {
                if let Some(location) = response.headers().get("Location") {
                    if allowed_redirect_count > 0 {
                        return get(location.to_str().unwrap(), msg, allowed_redirect_count - 1);
                    }
                    panic!("Too many redirects")
                }
            }

            if response.status().is_success() {
                return box response.body()
                    .limit(1024 * 1024 * 32)
                    .then(move |body| {
                        match body {
                            Ok(data) => {
                                return Ok(super::Resource {
                                    req: msg.clone(),
                                    cache_until: u64::max_value(),
                                    data: data.to_vec(),
                                });
                            }
                            Err(e) => {
                                return Err(ResourceError::Other(e.into()));
                            }
                        };
                    });
            } else if response.status().is_client_error() {
                return box err(ResourceError::NotFound);
            }

            return box err(ResourceError::NotFound);
        };

        let next_action: Box<dyn Future<Item=_, Error=ResourceError>> = box request
            .map_err(|e| ResourceError::Other(e.into()))
            .and_then(parse_response);


        return next_action;
    }

        let fut = get::<I>(&msg.url(), msg, 3);
        return box WrapFuture::into_actor(fut, self);
    */
    }
}

impl<I: interop::Types> NetworkFileSource<I> {
    pub fn new() -> Self {
        return NetworkFileSource {
            client: I::HttpClientType::new().unwrap()
        };
    }
    pub fn spawn() -> Addr<Self> {
        start_in_thread(|| NetworkFileSource::new())
    }
}