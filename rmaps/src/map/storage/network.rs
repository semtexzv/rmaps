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

        fn get(url: &str, msg: ResourceRequest, allowed_redirect_count: usize) -> Box<dyn Future<Item=(), Error=()>> {

            let request = Box::new(client::get(url)
                .timeout(::std::time::Duration::from_secs(15))
                .finish().unwrap()
                .send());


            let next_action: Box<dyn Future<Item=(), Error=()>> = Box::new(request
                .map_err(|e| ())
                .and_then(move |response: ClientResponse| -> Box<dyn Future<Item=(), Error=()>> {
                    if response.status().is_redirection() {
                        if let Some(location) = response.headers().get("Location") {
                            if allowed_redirect_count > 0 {
                                return box get(location.to_str().unwrap(), msg, allowed_redirect_count - 1).then(|_| Ok(()));
                            }
                            panic!("Too many redirects")
                        }
                    }

                    if response.status().is_success() {
                        return box response.body()
                            .then(move |body| {
                                match body {
                                    Ok(data) => {
                                        let cb = super::ResourceCallback {
                                            result: Ok(super::Resource {
                                                req: msg.request.clone(),
                                                data: data.to_vec(),
                                            }),
                                            request: msg.request,
                                        };

                                        msg.callback.send(cb).wait();
                                    }
                                    Err(e) => {
                                        let cb = super::ResourceCallback {
                                            result: Err(e.into()),
                                            request: msg.request,
                                        };
                                        msg.callback.send(cb).wait().unwrap();
                                    }
                                };

                                Ok(())
                            });
                    }

                    error!("Failed to retrieve : {:?}", response);
                    return box ok(());
                }));


            return next_action;
        }

        let fut = get(&msg.request.url(), msg, 3);
        Arbiter::handle().spawn(fut);

    }
}

impl NetworkFileSource {
    pub fn new() -> Self {
        return NetworkFileSource {};
    }
    pub fn spawn() -> Addr<Syn, Self> {
        start_in_thread(|| NetworkFileSource::new())
    }
}