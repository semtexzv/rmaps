pub use syn::*;
pub use syn::token::Comma;
pub use std::ops::Deref;
pub use proc_macro2::{Ident, Span};
pub use quote::ToTokens;

pub use syn::punctuated::*;
pub use syn::spanned::Spanned;

use proc_macro2;

pub fn actor_handle_name(actor_name: &impl ::std::fmt::Display) -> Ident {
    let handle_name = Ident::new(&format!("{}Addr", &actor_name), Span::call_site());
    handle_name
}

pub fn fn_msg_name(fn_name: &impl ::std::fmt::Display, base_actor_name: &impl ::std::fmt::Display) -> Ident {
    Ident::new(&format!("Msg_{}_{}", base_actor_name, fn_name), Span::call_site())
}

pub fn make_path(x: &impl ::std::fmt::Display) -> Ident {
    Ident::new(&format!("{}", x), Span::call_site())
}

pub fn return_type(sig: &MethodSig) -> proc_macro2::TokenStream {
    let mut msg_return_type: proc_macro2::TokenStream = match &sig.decl.output {
        a @ ReturnType::Default => {
            quote!(())
        }
        ReturnType::Type(_, t) => {
            quote!(#t)
        }
    };

    msg_return_type

}


pub fn get_args_except_first(args: &Punctuated<FnArg, Comma>) -> Punctuated<FnArg, Comma> {
    let first_arg_self = match args.first() {
        Some(punctuated::Pair::End(a)) | Some(punctuated::Pair::Punctuated(a, _)) => {
            if let FnArg::SelfRef(_) = a {
                true
            } else if let FnArg::SelfValue(_) = a {
                panic!("Cant have methods that take self by value")
            } else {
                false
            }
        }
        _ => false
    };

    let msg_struct_args = if first_arg_self {
        let data: punctuated::Punctuated<FnArg, token::Comma> = args.clone().into_pairs().into_iter().skip(1).collect();
        data
    } else {
        args.clone()
    };

    msg_struct_args
}

pub fn split_arg_names_and_types(args: &Punctuated<FnArg, Comma>) -> (Vec<Ident>, Vec<Type>) {
    let mut arg_names = vec![];
    let mut arg_types = vec![];
    for a in args.iter() {
        match a {
            FnArg::Captured(c) => {
                if let Pat::Ident(ref id) = c.pat {
                    arg_names.push(id.ident.clone());
                    arg_types.push(c.ty.clone());
                }
            }
            _ => {}
        }
    }

    (arg_names, arg_types)
}
