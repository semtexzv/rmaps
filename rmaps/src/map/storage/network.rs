use prelude::*;

/*
use common::reqwest::{
    unstable::async::*,
};

*/
use common::actix_web::{
    http::{
        StatusCode,
        header::{
            self, Header, HeaderName,
            LOCATION, CONTENT_LOCATION,
        },
    },
    client::{
        self,
        ClientConnector, ClientRequest, ClientResponse,
    },
    Body,
    HttpMessage,
    HttpResponse,
};

use super::*;

pub struct NetworkFileSource {}

impl Actor for NetworkFileSource {
    type Context = Context<Self>;
}

use common::futures::future::*;

impl Handler<super::ResourceRequest> for NetworkFileSource {
    type Result = ();

    fn handle(&mut self, msg: ResourceRequest, _ctx: &mut Context<Self>) {
        println!("Getting: {:?}", msg.request.url());

        fn get(url: &str, msg: ResourceRequest, allowed_redirect_count: usize) -> Box<dyn Future<Item=(), Error=ResourceError>> {
            let request: Box<dyn Future<Item=_, Error=ResourceError>> = Box::new(client::get(url)
                .timeout(::std::time::Duration::from_secs(15))
                .finish().unwrap()
                .send()
                .timeout(::std::time::Duration::from_secs(15))
                .map_err(|e| ResourceError::Other(e.into())));

            let parse_response = move |response: ClientResponse| -> Box<dyn Future<Item=(), Error=ResourceError>> {
                if response.status().is_redirection() {
                    if let Some(location) = response.headers().get("Location") {
                        if allowed_redirect_count > 0 {
                            return box get(location.to_str().unwrap(), msg, allowed_redirect_count - 1);
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
                                    let cb = super::ResourceResponse {
                                        result: Ok(super::Resource {
                                            req: msg.request.clone(),
                                            cache_until : u64::max_value(),
                                            data: data.to_vec(),
                                        }),
                                        request: msg.request,
                                    };

                                    spawn(msg.callback.send(cb));
                                }
                                Err(e) => {
                                    let cb = super::ResourceResponse {
                                        result: Err(ResourceError::Other(e.into())),
                                        request: msg.request,
                                    };
                                    spawn(msg.callback.send(cb));
                                }
                            };

                            Ok(())
                        });
                } else if response.status().is_client_error() {
                    let cb = super::ResourceResponse {
                        result: Err(ResourceError::NotFound),
                        request: msg.request,
                    };
                    spawn(msg.callback.send(cb));
                    return box ok(());
                }

                error!("Failed to retrieve : {:?}", response);
                return box ok(());
            };

            let next_action: Box<dyn Future<Item=_, Error=ResourceError>> = box request
                .map_err(|e| ResourceError::Other(e.into()))
                .and_then(parse_response);


            return next_action;
        }

        let fut = get(&msg.request.url(), msg, 3);
        spawn(fut.map_err(|e| {
            format_err!("Aasddsdasadsa : {:?}", e)
        }));
//        Arbiter::handle().spawn(fut);
        //::actix::Arbiter::spawn(fut);
    }
}

impl NetworkFileSource {
    pub fn new() -> Self {
        return NetworkFileSource {};
    }
    pub fn spawn() -> Addr<Self> {
        start_in_thread(|| NetworkFileSource::new())
    }
}