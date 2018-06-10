use prelude::*;
use proc_macro2;

use proc_macro;

fn generate_actor_impl(input: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &input.ident;
    let handle_name = actor_handle_name(&input.ident);

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        // The generated impl.
        impl #impl_generics ::actix::Actor for #name #ty_generics #where_clause {
            type Context = ::actix::Context<#name #ty_generics>;
        }
    };
    expanded
}


pub fn do_derive_actor(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = ::syn::parse(input).unwrap();

    let actor_impl = generate_actor_impl(&input);
    let actor_handle = ::addr::generate_actor_handle_struct(&input.ident);

    let res = quote! {
        #actor_impl
        #actor_handle
    };

    res.into()
}


pub fn derive_actor_trait(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let _trait: ItemTrait = parse(item).unwrap();
    let handle_name = actor_handle_name(&_trait.ident);

    if _trait.generics != parse_quote!() {
        panic!("Generic traits cannot be actors" );
    }

    let actor_handle_trait = ::addr::generate_actor_handle_trait(&_trait.ident,&_trait);

    let mut msg_structs = vec![];
    let mut msg_impls : Vec<ItemImpl> = vec![];
    for i in _trait.items.iter() {
        if let TraitItem::Method(ref m) = i {
            let mut msg_name = fn_msg_name(&m.sig.ident,&_trait.ident);
            msg_structs.push(::msg::gen_msg_struct(&_trait.ident,&m.sig));
            msg_impls.push(::msg::gen_msg_msg_impl(&_trait.ident,&m.sig));
        }
    }

    let res = quote!{
        #_trait
        #actor_handle_trait

        #(#msg_structs)*
        #(#msg_impls)*

    };
    //panic!("{:?}", handle_name);
    return res.into();
}