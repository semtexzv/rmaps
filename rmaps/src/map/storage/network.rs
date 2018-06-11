use prelude::*;

use actix_web::client;

use super::*;

#[derive(Actor)]
pub struct NetworkFileSource {}
#[actor_impl]
impl FileSource for NetworkFileSource {
    fn can_handle(&self, res: Resource) -> bool {
        let url = res.url();
        return url.starts_with("http://" ) || url.starts_with("https://")
    }

    fn get(&mut self, res: Resource) ->::act::ProxyLocal<ResResponse, Error> {
        let fut = client::get(res.url().clone())   // <- Create request builder
            .finish().unwrap()
            .send()                               // <- Send http request
            .map_err(|x| {
                panic!("Retrieval failed {}", x);
                x.into()
            })
            .map(move |data| {                // <- server http response
                println!("Response: {:?}", data);
                super::ResResponse {
                    resource: res,
                    data: vec![],
                }
            });

        ::act::ProxyLocal::new(fut)
    }


    /*
    fn get(&mut self, res: Resource, accept_addr: Box<FileAcceptorAddr + Send + 'static>) -> Result<()> {

        println!("Network Loading URL");
        let fut = client::get(res.url().clone())   // <- Create request builder
            .finish().unwrap()
            .send()                               // <- Send http request
            .map_err(|x| {
                panic!("Retrieval failed {}", x);
                ()
            })
            .and_then(move |data| {                // <- server http response
                println!("Response: {:?}", data);
                accept_addr.resource_ready_async(super::ResResponse {
                    resource: res,
                    data: vec![],
                });
                Ok(())
            });

        println!("Network Loading URL");
        actix::Arbiter::handle().spawn({
            fut
        });
        println!("Network Loading URL Finished");
        Ok(())
    }
    */
}

impl NetworkFileSource {
    fn new() -> Self {
        return NetworkFileSource {};
    }
    pub fn spawn() -> NetworkFileSourceAddr {
        NetworkFileSourceAddr {
            addr: start_in_thread(|| NetworkFileSource::new())
        }
    }
}