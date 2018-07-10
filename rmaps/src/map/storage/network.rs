use prelude::*;

use actix_web::client;
use common::actix_web::HttpMessage;

use super::*;

pub struct NetworkFileSource {}

impl Actor for NetworkFileSource {
    type Context = Context<Self>;
}

impl Handler<super::ResourceRequest> for NetworkFileSource {
    type Result = ();

    fn handle(&mut self, msg: ResourceRequest, _ctx: &mut Context<Self>) {
        //println!("Getting: {:?}", msg.0.url());

        let fut = client::get(msg.0.url().clone())   // <- Create request builder
            .timeout(::std::time::Duration::from_secs(15))
            .finish().unwrap()
            .send()                               // <- Send http request
            .timeout(::std::time::Duration::from_secs(15))
            .map_err(|x| {
                println!("Retrieval failed: {}", x);
                x.into()
            })
            .and_then(move |res| res.body().map_err(|e| e.into()))
            .then(move |body| {
               // println!("THEN : {:?}", body);
                match body {
                    Ok(data) => {
                        let resource = super::Resource {
                            req : msg.0.clone(),
                            data: data.to_vec()// vec![] //data,
                        };
                        msg.1.do_send(super::ResourceCallback(Ok(resource))).unwrap();
                    }
                    Err(e) => {
                        msg.1.do_send(super::ResourceCallback(Err(e))).unwrap()
                    }
                }
                Ok(())
            });
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