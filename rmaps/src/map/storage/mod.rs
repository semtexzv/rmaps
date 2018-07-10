use prelude::*;

pub mod resource;

pub mod local;
pub mod network;

pub use self::resource::*;

#[derive(Debug)]
pub struct ResourceCallback(pub Result<Resource>);

impl Message for ResourceCallback {
    type Result = ();
}

pub struct ResourceRequest(pub Request, pub Recipient<Syn, ResourceCallback>);

impl Message for ResourceRequest {
    type Result = ();
}

//#[derive(Actor)]
pub struct DefaultFileSource {
    local: SyncAddr<local::LocalFileSource>,
    network: SyncAddr<network::NetworkFileSource>,
}

impl Actor for DefaultFileSource {
    type Context = Context<DefaultFileSource>;
}

impl Handler<ResourceRequest> for DefaultFileSource {
    type Result = ();

    fn handle(&mut self, msg: ResourceRequest, _ctx: &mut Context<Self>)  {
        let url = { msg.0.url().to_string() };
        if url.starts_with("file://") || url.starts_with("local://") {
            self.local.do_send(msg);
        } else if url.starts_with("http://") || url.starts_with("https://")  {
            self.network.do_send(msg);
        } else {
            panic!("No data source available for {:?}", url);
        }
    }
}


impl DefaultFileSource {
    pub fn new() -> Self {
        DefaultFileSource {
            local: local::LocalFileSource::spawn(),
            network: network::NetworkFileSource::spawn(),
        }
    }

    pub fn spawn() -> Addr<Syn, Self> {
        start_in_thread::<DefaultFileSource, _>(|| DefaultFileSource::new())
    }
}

