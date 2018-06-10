use prelude::*;
use proc_macro2;
use proc_macro;

pub fn gen_msg_handle_method(handle_name: &Ident, base_actor_name: &Ident, sig: &MethodSig, traited : bool) -> ImplItemMethod {
    let method_ident = &sig.ident;
    let msg_ident = fn_msg_name(method_ident, base_actor_name);
    let mut return_type = return_type(&sig);

    let args = &get_args_except_first(&sig.decl.inputs);
    let (ref arg_names, _) = split_arg_names_and_types(&args);

    let mut sig = ::sigs::gen_msg_handle_sig(handle_name,base_actor_name,&sig, traited);

    let handle_method = parse_quote!{
        pub #sig {
            let data = #msg_ident(#(#arg_names),*);
            self.addr.send(data)
        }
    };
    handle_method
}

pub fn gen_msg_handle_method_async(handle_name: &Ident, base_actor_name: &Ident, sig: &MethodSig) -> ImplItemMethod {
    let method_ident = &sig.ident;
    let msg_ident = fn_msg_name(method_ident, base_actor_name);
    let mut return_type = return_type(&sig);

    let args = &get_args_except_first(&sig.decl.inputs);
    let (ref arg_names, _) = split_arg_names_and_types(&args);

    let mut sig = ::sigs::gen_msg_handle_sig(handle_name,base_actor_name,&sig,false);

    sig.decl.output =  ReturnType::Default;
    sig.ident = Ident::new(&format!("{}_async",sig.ident),Span::call_site());

    let handle_method = parse_quote!{
        pub #sig {
            let data = #msg_ident(#(#arg_names),*);
            self.addr.do_send(data)
        }
    };
    handle_method
}


pub fn gen_impl_handler(actor_name: &Ident, base_actor_name: &Ident, sig: &MethodSig) -> proc_macro2::TokenStream {
    let method_ident = &sig.ident;
    let msg_ident = fn_msg_name(method_ident, base_actor_name);
    let mut return_type = return_type(&sig);

    let (impl_generics, ty_generics, where_clause) = sig.decl.generics.split_for_impl();

    let mut args = get_args_except_first(&sig.decl.inputs);
    let (arg_names, arg_types) = split_arg_names_and_types(&args);
    let arg_indices: Vec<usize> = arg_names.iter().enumerate().map(|(x, y)| x).collect();

    let res = quote!(

        impl #impl_generics ::actix::Handler<#msg_ident #ty_generics> for #actor_name  #where_clause {
            type Result = #return_type;
            fn handle(&mut self, msg: #msg_ident #ty_generics, ctx: &mut Self::Context) -> #return_type {
               self.#method_ident( #(msg.#arg_indices),*)
            }
        }
    );

    //panic!("{}",res);
    res
}


pub fn actor_impl_simple(input : &ItemImpl) -> proc_macro2::TokenStream {
    let i = &input.self_ty;
    let actor_name = make_ident(&quote!(#i));
    let handle_name = actor_handle_name(&actor_name);


    let mut msg_structs = vec![];
    let mut msg_impls: Vec<ItemImpl> = vec![];
    let mut handle_methods: Vec<_> = vec![];
    let mut handler_impls: Vec<_> = vec![];

    for i in input.items.iter() {
        if let ImplItem::Method(ref m) = i {
            let mut msg_name = fn_msg_name(&m.sig.ident, &actor_name);
            let mut return_type = return_type(&m.sig);

            msg_structs.push(::msg::gen_msg_struct(&actor_name, &m.sig));
            msg_impls.push(::msg::gen_msg_msg_impl(&actor_name, &m.sig));

            handle_methods.push(::actor_fns::gen_msg_handle_method(&handle_name, &actor_name, &m.sig,false));
            handle_methods.push(::actor_fns::gen_msg_handle_method_async(&handle_name, &actor_name, &m.sig));
            handler_impls.push(::actor_fns::gen_impl_handler(&actor_name, &actor_name, &m.sig));
        }
    }

    let res = quote! {
        # ( # msg_structs) *
        # ( # msg_impls) *


        impl # handle_name {
            # ( # handle_methods) *
        }
        #(#handler_impls)*
    };
    res
}

pub fn actor_impl_traited(input : &ItemImpl) -> proc_macro2::TokenStream {
    let i = &input.self_ty;
    let actor_name = make_ident(&quote!(#i));
    let t = &input.trait_.as_ref().unwrap().1;
    let trait_name = make_ident(&quote!(#t));

    let handle_name = actor_handle_name(&actor_name);
    let trait_handle_name = actor_handle_name(&actor_name);

    let handle_impl = ::addr::generate_actor_handle_impl_traited(&input);
    let mut handler_impls: Vec<_> = vec![];

    for i in input.items.iter() {
        if let ImplItem::Method(ref m) = i {
            handler_impls.push(::actor_fns::gen_impl_handler(&actor_name, &trait_name, &m.sig));
        }
    }

    let res = quote! {
        #handle_impl
        #(#handler_impls)*
    };
    res
}

/*
pub struct ActorImplRes {
    pub source_impl: ItemImpl,
    pub generated: proc_macro2::TokenStream,
    pub register: proc_macro2::TokenStream,
}
pub fn actor_impl(input: &ItemImpl) -> ActorImplRes {
    let (actor_name, handle_name): (Ident, Ident) = if let &Type::Path(ref p) = input.self_ty.deref() {
        let a = make_ident(&quote!( #p));
        let b = actor_handle_name(&a);
        (a, b)
    } else {
        panic!("Unsupported impl target")
    };

    let trait_path: Option<Path> = if let Some((_, ref p, _)) = input.trait_ {
        Some(p.clone())
    } else {
        None
    };

    let trait_ident: Option<Ident> = trait_path.as_ref().and_then(|p| {
        p.segments.last().as_ref().map(|v| v.clone().value().ident.clone())
    });

    let base_actor_name = if let Some(ref p) = trait_ident {
        p.clone()
    } else {
        actor_name.clone()
    };


    let mut msg_structs = vec![];
    let mut msg_impls: Vec<ItemImpl> = vec![];
    let mut handle_methods: Vec<_> = vec![];
    let mut registers: Vec<_> = vec![];

    for i in input.items.iter() {
        if let ImplItem::Method(ref m) = i {
            let mut msg_name = fn_msg_name(&m.sig.ident, &base_actor_name);
            let mut return_type = return_type(&m.sig);

            msg_structs.push(::msg::gen_msg_struct(&base_actor_name, &m.sig));
            msg_impls.push(::msg::gen_msg_msg_impl(&base_actor_name, &m.sig));

            handle_methods.push(::actor_fns::gen_msg_handle_method(&handle_name, &base_actor_name, &m.sig));
            handle_methods.push(::actor_fns::gen_msg_handle_method_async(&handle_name, &base_actor_name, &m.sig));
            registers.push(::actor_fns::gen_impl_handler(&actor_name, &base_actor_name, &m.sig));
        }
    }
    let mut opt_handle_from = quote!();

    let res = quote! {
        # ( # msg_structs) *
        # ( # msg_impls) *


        impl # handle_name {
            # ( # handle_methods) *
        }
        #opt_handle_from
    };

    ActorImplRes {
        source_impl: input.clone(),
        generated: res.into(),
        register: quote!(#(#registers)*),
    }
}

*/