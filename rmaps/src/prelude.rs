pub use ::common::export::*;
pub use ::act_codegen::*;

pub fn start_in_thread<A: Actor<Context=Context<A>> + Send + 'static, F: FnOnce() -> A + Send + 'static>(a: F) -> Addr<Syn,A> {
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