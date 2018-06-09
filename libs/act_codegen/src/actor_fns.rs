use prelude::*;
use proc_macro2;
use proc_macro;

pub fn do_actor_mod(id: Ident, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut _mod: ItemMod = parse(item).unwrap();

    let mut regs = vec![];
    let mut output = quote!();
    for x in _mod.content.as_ref().unwrap().1.iter() {
        if let Item::Impl(_impl) = x {
            let i: ItemImpl = _impl.clone();
            let actor_impl = actor_impl(&i);

            let orig = actor_impl.source_impl;

            let generated = actor_impl.generated;
            let register = actor_impl.register;
            regs.push(register);

            output = quote! {
                #output
                #orig
                #generated
            }
        } else {
            quote! {
                #output
                #x
            };
        }
    }

    let id = _mod.ident;


    quote_spanned!(Span::call_site() =>
        pub mod #id {
            #output

            pub fn setup() {
                #(#regs);*
            }
        }
    ).into()
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

pub fn gen_msg_struct(base_actor_name: &Ident, sig: &MethodSig) -> DeriveInput {
    let method_ident = &sig.ident;
    let msg_ident = fn_msg_name(method_ident, base_actor_name);

    let mut args = get_args_except_first(&sig.decl.inputs);
    let (arg_names, arg_types) = split_arg_names_and_types(&args);

    parse_quote!( struct #msg_ident(#(#arg_types),*);)
}

pub fn gen_msg_handle_method(handle_name: &Ident, base_actor_name: &Ident, sig: &MethodSig) -> ImplItemMethod {
    let method_ident = &sig.ident;
    let msg_ident = fn_msg_name(method_ident, base_actor_name);

    let args = get_args_except_first(&sig.decl.inputs);
    let (arg_names, _) = split_arg_names_and_types(&args);

    let handle_method: ImplItemMethod = parse2(quote_spanned!( sig.span() =>
        pub #sig {
            let data = #msg_ident(#(#arg_names),*);
            self.send(data);
        }
    )).unwrap();

    handle_method
}


pub fn gen_handler_register(actor_name: &Ident, base_actor_name: &Ident, sig: &MethodSig) -> proc_macro2::TokenStream {
    let method_ident = &sig.ident;
    let msg_ident = fn_msg_name(method_ident, base_actor_name);
    let mut args = get_args_except_first(&sig.decl.inputs);
    let (arg_names, arg_types) = split_arg_names_and_types(&args);
    let arg_indices: Vec<usize> = arg_names.iter().enumerate().map(|(x, y)| x).collect();

    quote!(
        ::act::world().register_handler(move |a : &mut #actor_name , mut msg : Box<#msg_ident>| {
            let msg = *msg;
            a.#method_ident( #(msg.#arg_indices),*);
        });
    )
}

pub struct ActorImplRes {
    source_impl: ItemImpl,
    generated: proc_macro2::TokenStream,
    register: proc_macro2::TokenStream,
}

pub fn actor_impl(input: &ItemImpl) -> ActorImplRes {
    let (actor_name, handle_name, setup_name): (Ident, Ident, Ident) = if let &Type::Path(ref p) = input.self_ty.deref() {
        let a = make_ident(&quote!( #p));
        let b = actor_handle_name(&a);
        let c = Ident::new(&format!("setup_{}", &a), Span::call_site());
        (a, b, c)
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
            msg_structs.push(::actor_fns::gen_msg_struct(&base_actor_name, &m.sig));
            msg_impls.push(parse_quote!(
                impl ::act::Message for #msg_name {

                }
            ));

            handle_methods.push(::actor_fns::gen_msg_handle_method(&handle_name, &base_actor_name, &m.sig));
            registers.push(::actor_fns::gen_handler_register(&actor_name, &base_actor_name, &m.sig));
        }
    }
    let mut opt_handle_from = quote!();

    if let Some(ref tp) = trait_ident {
        // If we are implementing a trait, the message types already exist
        msg_structs.clear();
        msg_impls.clear();

        let t_handle_name = actor_handle_name(&quote!(#tp));

        opt_handle_from = quote! {
            impl ::std::convert::Into<#t_handle_name> for #handle_name {
                fn into(self) -> #t_handle_name {
                    #t_handle_name {
                        chan : self.chan
                    }
                }
            }
        }
    } else {

    };

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
        register: quote!(#(#registers;)*),
    }
}
