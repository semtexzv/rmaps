use ::prelude::*;

pub fn gen_msg_handle_sig(handle_name : &Ident, base_actor_name : &Ident , sig: &MethodSig, traited : bool) -> MethodSig {
    let method_ident = &sig.ident;
    let msg_ident = fn_msg_name(method_ident, base_actor_name);

    let msg_generics = ::msg::msg_generics(base_actor_name,sig);

    let mut msg_return_type = return_type(&sig);

    let args = &get_args_except_first(&sig.decl.inputs);
    let (ref arg_names, _) = split_arg_names_and_types(&args);

    let mut sig = sig.clone();
    /*
    let params = sig.decl.generics.params;
    sig.decl.generics.params = if params.is_empty() {
        parse_quote!(_RF : Future<Item=#msg_return_type,Error=::actix::MailboxError>,)
    } else {
        parse_quote!(#params,_RF : Future<Item=#msg_return_type,Error=::actix::MailboxError>)
    };
    */
    //common::prelude::dev::Request<common::prelude::Syn, A, Msg_A_a<T>>

    let mut sig = sig.clone();
    sig.decl.inputs = parse_quote!(&self,#(#args),*);

    if traited {
        sig.decl.output = parse_quote!(-> ::actix::dev::Request<::actix::Syn,Self::ActorType,#msg_ident #msg_generics>);
    } else {
        sig.decl.output = parse_quote!(-> ::actix::dev::Request<::actix::Syn,#base_actor_name,#msg_ident #msg_generics>);
    }
    sig
}