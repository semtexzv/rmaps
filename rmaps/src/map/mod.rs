use prelude::*;

pub mod render;
pub mod layers;
pub mod style;
pub mod storage;

use map::layers::Layer;


pub struct MapView {
    addr: Addr<Unsync, MapViewImpl>,
    sys: SystemRunner,
}

impl MapView {
    pub fn new<F: glium::backend::Facade + Clone + 'static>(f: &F) -> Self {
        let mut sys = System::new("Map");
        let _impl = MapViewImpl::new(f);

        let addr = _impl.start();

        return MapView {
            sys,
            addr,
        };
    }

    pub fn do_run<R>(&mut self, f: impl FnOnce(Addr<Unsync, MapViewImpl>) -> R) -> R {
        let addr = self.addr.clone();
        let res = self.sys.run_until_complete(::common::futures::future::lazy(|| {
            Ok::<R, !>(f(addr))
        }));
        self.sys.pulse();
        res.unwrap()
    }

    pub fn render(&mut self, surface: glium::Frame) {
        self.do_run(|add| {
            add.do_send(MapMethodArgs::Render(surface));
        });
    }

    pub fn set_style_url(&mut self, url: &str) {
        self.do_run(|add| {
            add.do_send(MapMethodArgs::SetStyleUrl(url.into()));
        });
    }
}

#[actor_handle]
pub struct MapViewImpl {
    addr: Option<MapViewImplAddr>,
    facade: Box<glium::backend::Facade>,
    style: Option<style::Style>,
    layers: Vec<layers::LayerHolder>,
    source: storage::DefaultFileSourceAddr,
}

impl Actor for MapViewImpl {
    type Context = Context<MapViewImpl>;

    fn started(&mut self, ctx: &mut <Self as Actor>::Context) {
        self.addr = Some(MapViewImplAddr {
            addr: ctx.address(),
        })
    }
}

impl MapViewImpl {
    pub fn new<F: glium::backend::Facade + Clone + 'static>(f: &F) -> Self {
        let src_add = storage::DefaultFileSource::spawn();

        let m = MapViewImpl {
            addr: None,
            facade: Box::new((*f).clone()),
            style: None,
            layers: vec![],
            source: src_add,
        };


        return m;
    }

    pub fn set_style(&mut self, style: style::Style) {
        self.layers.clear();
        self.layers = layers::parse_style_layers(self.facade.deref(), &style);
        println!("Layers : {:?}", style);
        self.style = Some(style);
    }

    pub fn set_style_url(&mut self, url: &str) {
        println!("Setting style url : {:?}", url);

        let resource = storage::Resource::style(url.into());

        actix::Arbiter::handle().spawn(
            self.source.get(resource).flatten()
                .then(|s| {
                    panic!("DATA: {:?}", s);
                    Ok(())
                }));
    }
    pub fn render(&mut self, target: &mut glium::Frame) {
        for l in self.layers.iter_mut() {
            l.render(target);
        }
    }
}

use self::storage::{*};


pub enum MapMethodArgs {
    Render(glium::Frame),
    SetStyleUrl(String),
}

impl Message for MapMethodArgs {
    type Result = ();
}

impl Handler<MapMethodArgs> for MapViewImpl {
    type Result = ();

    fn handle(&mut self, mut msg: MapMethodArgs, ctx: &mut Self::Context) -> () {
        match msg {
            MapMethodArgs::Render(mut frame) => {
                self.render(&mut frame);
                frame.finish();
            }
            MapMethodArgs::SetStyleUrl(url) => {
                self.set_style_url(&url)
            }
        };
    }
}