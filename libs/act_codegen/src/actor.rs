use prelude::*;
use proc_macro2;

use proc_macro;

fn generate_actor_impl(input: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &input.ident;
    let handle_name = actor_handle_name(&input.ident);

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        // The generated impl.
        impl #impl_generics ::act::Actor for #name #ty_generics #where_clause {
            type HandleType = #handle_name;
            fn process_messages(&mut self) {
                while let Ok(mut msg) = self.inbox.recvr.try_recv() {
                    ::act::world().handle_msg(self, msg);
                }
            }
            fn handle(&self) -> &Self::HandleType {
                &self.handle
            }
        }
    };
    expanded
}


fn generate_actor_handle(name : &Ident) -> proc_macro2::TokenStream {
    let handle_name = actor_handle_name(name);

    let res = quote! {
        #[derive(Clone,Debug)]
        pub struct #handle_name {
            chan : ::std::sync::mpsc::Sender<::act::MessageWrapper>
        }

        impl ::act::ActorHandle for #handle_name {


        }
        impl #handle_name {
            fn send<M: ::act::Message>(&self, m : M) {
               self.chan.send(act::MessageWrapper::new(m));
            }
            fn new_inbox_pair() -> (::act::Inbox, Self) {
                let (sendr, recvr) = ::std::sync::mpsc::channel();
                (::act::Inbox {
                    recvr
                }, #handle_name {  chan : sendr })
            }
        }
    };

    res.into()
}

pub fn check_has_self_handle(input: &DeriveInput) -> bool {
    match input.data {
        Data::Struct(ref x) => {
            match x.fields {
                Fields::Named(ref n) => {
                    for p in n.named.pairs() {
                        let val = p.value();
                        if format!("{}", val.ident.as_ref().unwrap()) == "handle" {
                            return true;
                        }
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }

    false
}

pub fn do_derive_actor(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = ::syn::parse(input).unwrap();
    if !check_has_self_handle(&input) {
        panic!("Actor must have a `handle` Field containing it's handle");
    }

    let actor_impl = generate_actor_impl(&input);
    let actor_handle = generate_actor_handle(&input.ident);

    let res = quote! {
        #actor_impl
        #actor_handle
    };

    res.into()
}


pub fn derive_actor_trait(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let _trait: ItemTrait = parse(item).unwrap();
    let handle_name = actor_handle_name(&_trait.ident);

    let actor_handle = generate_actor_handle(&_trait.ident);

    let mut msg_structs = vec![];
    let mut msg_impls : Vec<ItemImpl> = vec![];
    let mut handle_methods : Vec<_>  = vec![];

    for i in _trait.items.iter() {
        if let TraitItem::Method(ref m) = i {
            let mut msg_name = fn_msg_name(&m.sig.ident,&_trait.ident);
            msg_structs.push(::actor_fns::gen_msg_struct(&_trait.ident,&m.sig));
            msg_impls.push(parse_quote!(
                impl ::act::Message for #msg_name {

                }
            ));

            handle_methods.push(::actor_fns::gen_msg_handle_method(&handle_name,&_trait.ident,&m.sig));
        }
    }

    let res = quote!{
        #_trait
        #actor_handle

        #(#msg_structs)*
        #(#msg_impls)*

        impl #handle_name {
            #(#handle_methods);*
        }
    };
    //panic!("{:?}", handle_name);
    return res.into();
}