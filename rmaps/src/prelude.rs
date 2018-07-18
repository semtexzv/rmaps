pub use ::common::export::*;
pub use common::failure;
pub use rmaps_derive::*;

pub fn start_in_thread<A: Actor<Context=Context<A>> + Send + 'static, F: FnOnce() -> A + Send + 'static>(a: F) -> Addr<Syn, A> {
    let (tx, rx) = ::std::sync::mpsc::channel();

    ::std::thread::spawn(move || {
        let sys = System::new("aa");

        let actor = a();
        let addr = actor.start();
        let _ = tx.send(addr);
        let _ = sys.run();
    });

    rx.recv().unwrap()
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