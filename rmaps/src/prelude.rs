pub use ::common::export::*;
pub use common::failure;
pub use rmaps_derive::*;


pub use geometry;

pub use imgui;
pub use imgui_glium_renderer;
/*
pub fn start_in_thread<A: Actor<Context=Context<A>> + Send + 'static, F: FnOnce() -> A + Send + 'static>(a: F) -> Addr< A> {
    let (tx, rx) = ::std::sync::mpsc::channel();

    ::std::thread::spawn(move || {
        System::run(move || {

            let actor = a();
            let addr = actor.start();
            let _ = tx.send(addr);
        });
        //let sys = System::new("aa");

      //  let _ = sys.run();
    });

    rx.recv().unwrap()
}
*/
pub const HELKP: u32 = 3;

pub fn start_in_thread<A: Actor<Context=Context<A>> + Send + 'static, F: FnOnce() -> A + Send + 'static>(a: F) -> Addr<A> {
    let (tx, rx) = ::std::sync::mpsc::channel();
    format!("");
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

/*
impl<A, M, I: 'static, E: 'static> actix::dev::MessageResponse<A, M> for Box<Future<Item=I,Error=E>>
    where
        A: Actor,
        M: Message<Result=Box<Future<Item=I,Error=E>>>,
{
    fn handle<R: actix::dev::ResponseChannel<M>>(self, ctx : &mut A::Context, tx: Option<R>) {
        if let Some(tx) = tx {
            tx.send(self);
        }
    }
}
*/