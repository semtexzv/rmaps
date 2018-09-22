pub use ::common::export::*;
pub use common::failure;


pub use std::convert::{TryFrom, From, Into};
pub use rmaps_derive::*;


use common::actix::prelude::*;
use common::actix::{ResponseFuture, ResponseActFuture};

pub use geometry;

pub use imgui;
pub use imgui_glium_renderer;

pub fn start_in_thread<A: Actor<Context=Context<A>> + Send + 'static, F: FnOnce() -> A + Send + 'static>(a: F) -> Addr<A> {
    let (tx, rx) = ::std::sync::mpsc::channel();
    ::std::thread::spawn(move || {
        let sys = System::new("aaaasaas");

        let actor = a();
        let addr = actor.start();
        let _ = tx.send(addr);
        let _ = sys.run();
    });

    rx.recv().unwrap()
}

use common::glium::uniforms::{Uniforms, UniformValue};

pub struct MergeUniforms<'u, A: Uniforms + 'u, B: Uniforms + 'u> (pub &'u A, pub &'u B);

impl<'u, A: Uniforms + 'u, B: Uniforms + 'u> Uniforms for MergeUniforms<'u, A, B> {
    fn visit_values<'a, F: FnMut(&str, UniformValue<'a>)>(&'a self, mut f: F) {
        self.0.visit_values(&mut f);
        self.1.visit_values(f);
    }
}

