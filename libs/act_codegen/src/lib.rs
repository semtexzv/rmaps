#![feature(proc_macro)]
#![recursion_limit = "128"]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate syn;

#[macro_use]
extern crate quote;

#[macro_use]
extern crate ordermap;


mod prelude;
mod sigs;
mod actor;
mod actor_fns;
mod msg;
mod addr;

use syn::synom::Parser;
use syn::synom::Synom;
use syn::parse;

#[proc_macro_derive(Actor)]
pub fn derive_actor(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    actor::do_derive_actor(input)
}

/*
#[proc_macro_attribute]
pub fn actor_impls(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let expected_id: proc_macro2::Ident = syn::parse(attr).unwrap();

    //actor_fns::do_actor_mod(expected_id, item)
}
*/
#[proc_macro_attribute]
pub fn actor_impl(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let a  : proc_macro2::TokenStream = item.clone().into();
    let _impl : syn::ItemImpl = parse(item).unwrap();
    let res = if let  Some(ref x) = _impl.trait_ {
        actor_fns::actor_impl_traited(&_impl)
    } else {
        actor_fns::actor_impl_simple(&_impl)
    };

    quote!(#a #res).into()
}

#[proc_macro_attribute]
pub fn derive_actor_trait(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    actor::derive_actor_trait(item)
}

use prelude::*;

