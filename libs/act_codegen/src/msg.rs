use prelude::*;

pub fn msg_generics(base_actor_name: &Ident, sig: &MethodSig) -> Generics {
    let method_ident = &sig.ident;
    let msg_ident = fn_msg_name(method_ident, base_actor_name);
    let (impl_generics, ty_generics, where_clause) = sig.decl.generics.split_for_impl();

    parse_quote!(#ty_generics)
}

pub fn gen_msg_struct(base_actor_name: &Ident, sig: &MethodSig) -> DeriveInput {
    let method_ident = &sig.ident;
    let msg_ident = fn_msg_name(method_ident, base_actor_name);
    let (impl_generics, ty_generics, where_clause) = sig.decl.generics.split_for_impl();

    let mut args = get_args_except_first(&sig.decl.inputs);
    let (arg_names, arg_types) = split_arg_names_and_types(&args);

    parse_quote!( struct #msg_ident #ty_generics (#(#arg_types),*);)
}


pub fn gen_msg_msg_impl(base_actor_name: &Ident, sig: &MethodSig) -> ItemImpl {
    let method_ident = &sig.ident;
    let msg_ident = fn_msg_name(method_ident, base_actor_name);
    let mut return_type = return_type(&sig);

    let (impl_generics, ty_generics, where_clause) = sig.decl.generics.split_for_impl();

    parse_quote!(
        impl #impl_generics ::actix::Message for #msg_ident #ty_generics #where_clause {
            type Result = #return_type;
        }
    )
}