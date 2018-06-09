pub use act::*;
pub use syn::*;
pub use syn::token::Comma;
pub use std::ops::Deref;
pub use proc_macro2::{Ident, Span};
pub use quote::ToTokens;

pub use syn::punctuated::*;
pub use syn::spanned::Spanned;

pub fn actor_handle_name(actor_name: &impl ::std::fmt::Display) -> Ident {
    let handle_name = Ident::new(&format!("{}Handle", &actor_name), Span::call_site());
    handle_name
}

pub fn fn_msg_name(fn_name: &impl ::std::fmt::Display, base_actor_name: &impl ::std::fmt::Display) -> Ident {
    Ident::new(&format!("Msg_{}_{}", base_actor_name, fn_name), Span::call_site())
}

pub fn make_ident(x: &impl ::std::fmt::Display) -> Ident {
    Ident::new(&format!("{}", x), Span::call_site())
}