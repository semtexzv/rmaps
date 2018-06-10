//#![feature(custom_attribute)]
#![feature(slice_patterns)]
#![feature(proc_macro)]
#![feature(proc_macro_mod)]
#![feature(never_type)]

#![allow(unused_imports)]
pub extern crate common;
pub extern crate mapbox_tiles;
pub extern crate css_color_parser;

pub extern crate act;
pub extern crate act_codegen;


/*
pub extern crate act;
#[macro_use]
pub extern crate act_codegen;
*/


pub mod prelude;
pub mod map;

use prelude::*;


#[derive_actor_trait]
pub trait TraitTest: Actor {
    fn a(&mut self, a: usize) -> Result<usize>;
    //fn a<T: Clone,_R : Future<Item=T,Error=actix::MailboxError>>(&mut self, a: T) -> ::act::Wrap<_R>;
}

#[derive(Actor)]
pub struct A {}

#[actor_impl]
impl TraitTest for A {
    fn a(&mut self, a: usize) -> Result<usize>{
        Ok(a.clone())
    }
}

#[actor_impl]
impl A {
    fn b(&mut self) -> Result<usize> {
        Ok(0)
    }
}
/*
mod t {
    use prelude::*;
    pub trait TraitTest: Actor {
        fn a(&mut self, a: usize) -> Result<usize>;
    }
    pub trait TraitTestAddr {
        type ActorType: ::actix::Actor<Context=::actix::Context<Self::ActorType>> + ::actix::Handler<Msg_TraitTest_a>;
        fn a(&self, a: usize) -> ::actix::dev::Request<::actix::Syn, Self::ActorType, Msg_TraitTest_a>;
        fn a_async(&self, a: usize);
    }
    struct Msg_TraitTest_a(usize);
    impl ::actix::Message for Msg_TraitTest_a {
        type Result = Result<usize>;
    }
    pub struct A {}
    impl ::actix::Actor for A {
        type Context = ::actix::Context<A>;
    }
    pub struct AAddr {
        pub addr: ::actix::Addr<Syn, A>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for AAddr {
        #[inline]
        fn clone(&self) -> AAddr {
            match *self {
                AAddr {
                    addr: ref __self_0_0,
                } => AAddr {
                    addr: ::std::clone::Clone::clone(&(*__self_0_0)),
                },
            }
        }
    }
    impl TraitTest for A {
        fn a(&mut self, a: usize) -> Result<usize> {
            Ok(a.clone())
        }
    }
    impl TraitTestAddr for AAddr {
        type ActorType = A;
        pub fn a(
            &self,
            a: usize,
        ) -> ::actix::dev::Request<::actix::Syn, Self::ActorType, Msg_TraitTest_a> {
            let data = Msg_TraitTest_a(a);
            self.addr.send(data)
        }
        pub fn a_async(&self, a: usize) {
            let data = Msg_TraitTest_a(a);
            self.addr.do_send(data)
        }
    }
    impl ::actix::Handler<Msg_TraitTest_a> for A {
        type Result = Result<usize>;
        fn handle(&mut self, msg: Msg_TraitTest_a, ctx: &mut Self::Context) -> Result<usize> {
            self.a(msg.0)
        }
    }
    impl A {
        fn b(&mut self) -> Result<usize> {
            Ok(0)
        }
    }
    struct Msg_A_b();
    impl ::actix::Message for Msg_A_b {
        type Result = Result<usize>;
    }
    impl AAddr {
        pub fn b(&self) -> ::actix::dev::Request<::actix::Syn, A, Msg_A_b> {
            let data = Msg_A_b();
            self.addr.send(data)
        }
        pub fn b_async(&self) {
            let data = Msg_A_b();
            self.addr.do_send(data)
        }
    }
    impl ::actix::Handler<Msg_A_b> for A {
        type Result = Result<usize>;
        fn handle(&mut self, msg: Msg_A_b, ctx: &mut Self::Context) -> Result<usize> {
            self.b()
        }
    }

}
*/


pub fn init() {
    use common::prelude::*;

    ::std::thread::spawn(move || {
        let sys = actix::System::new("test");
        sys.run();
    });
    //map::storage::actor_impls::setup();
    //map::storage::setup_FileSource();
}
