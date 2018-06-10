use prelude::*;
use proc_macro2;
use proc_macro;

pub fn generate_actor_handle_struct(name: &Ident) -> proc_macro2::TokenStream {
    let handle_name = actor_handle_name(name);

    let res = quote! {
        #[derive(Clone)]
        pub struct #handle_name {
            pub addr : ::actix::Addr<Syn,#name>,
        }
    };

    res.into()
}

pub fn generate_actor_handle_trait(name: &Ident, actor_trait: &ItemTrait) -> ItemTrait {
    let handle_name = actor_handle_name(name);

    let mut others = quote!();


    let mut handle_sigs: Vec<_> = vec![];
    let mut msg_types = vec![];
    let mut msg_ty_generics = vec![];


    for i in actor_trait.items.iter() {
        if let TraitItem::Method(ref m) = i {
            msg_types.push(fn_msg_name(&m.sig.ident, &actor_trait.ident));
            msg_ty_generics.push(::msg::msg_generics(&actor_trait.ident,&m.sig));
            handle_sigs.push(::actor_fns::gen_msg_handle_method(&handle_name, &actor_trait.ident, &m.sig,true).sig);
            handle_sigs.push(::actor_fns::gen_msg_handle_method_async(&handle_name, &actor_trait.ident, &m.sig).sig);
        } else {
            others = quote!(#others #i);
        }
    }
    //panic!("{}", quote!(#(#msg_types)*));

    let res = quote! {
        pub trait #handle_name {
            type ActorType : ::actix::Actor<Context=::actix::Context<Self::ActorType>> + #(::actix::Handler<#msg_types #msg_ty_generics>)+*;

            #others
            #(#handle_sigs;)*
        }
    };


    //panic!("{}",res);
    parse2(res).unwrap()
}

pub fn generate_actor_handle_impl_simple(item: &ItemImpl) -> ItemImpl {
    let actor_name: Ident =  if let &Type::Path(ref p) = item.self_ty.deref() {
        let a = make_ident(&quote!( #p));
        a
    } else {
        panic!("Unsupported impl target")
    };

    let handle_name = actor_handle_name(&actor_name);

    let mut handle_methods: Vec<_> = vec![];

    for i in item.items.iter() {
        if let ImplItem::Method(ref m) = i {
            let mut msg_name = fn_msg_name(&m.sig.ident, &actor_name);
            let mut return_type = return_type(&m.sig);
            handle_methods.push(::actor_fns::gen_msg_handle_method(&handle_name, &actor_name, &m.sig,false));
            handle_methods.push(::actor_fns::gen_msg_handle_method_async(&handle_name, &actor_name, &m.sig));
        }
    }
    let res = parse_quote! {
        impl #handle_name {

            #(#handle_methods)*
        }
    };

    res
}

pub fn generate_actor_handle_impl_traited(item: &ItemImpl) -> ItemImpl {
    let (actor_name, _trait_name): (Ident, Ident) = if let &Type::Path(ref p) = item.self_ty.deref() {
        let trait_path: Option<Path> = if let Some((_, ref _p, _)) = item.trait_ {
            Some(_p.clone())
        } else {
            None
        };

        let a = make_ident(&quote!( #p));
        let b = make_ident(&quote!( #trait_path ));
        (a, b)
    } else {
        panic!("Unsupported impl target")
    };

    let handle_name = actor_handle_name(&actor_name);
    let trait_handle_name = actor_handle_name(&_trait_name);


    let mut handle_methods: Vec<_> = vec![];

    for i in item.items.iter() {
        if let ImplItem::Method(ref m) = i {
            let mut msg_name = fn_msg_name(&m.sig.ident, &_trait_name);
            let mut return_type = return_type(&m.sig);
            handle_methods.push(::actor_fns::gen_msg_handle_method(&handle_name, &_trait_name, &m.sig,true));
            handle_methods.push(::actor_fns::gen_msg_handle_method_async(&handle_name, &_trait_name, &m.sig));
        }
    }
    let res = parse_quote! {
        impl #trait_handle_name for #handle_name {
            type ActorType = #actor_name;
            #(#handle_methods)*
        }
    };

    res
}