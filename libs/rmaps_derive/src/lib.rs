#![feature(proc_macro)]
#![recursion_limit = "128"]

extern crate proc_macro;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::spanned::Spanned;
use syn::*;

#[derive(Debug, Clone)]
struct FieldPropertyData {
    name: Ident,
    is_layout: bool,
    can_be_zoom: bool,
    can_be_feature: bool,
    src_name: Ident,
}

fn get_property_data(name: &Ident, meta: &Meta) -> FieldPropertyData {
    use std::ops::Deref;

    let mut res = FieldPropertyData {
        name: name.clone(),
        is_layout: false,
        can_be_zoom: true,
        can_be_feature: true,
        src_name: name.clone(),
    };

    let inner: Vec<_> = match meta {
        Meta::List(MetaList { ref ident, ref nested, .. }) if ident == "property" => {
            nested.iter().cloned().collect()
        }
        _ => {
            Vec::new()
        }
    };

    for item in inner {
        match item {
            NestedMeta::Meta(Meta::Word(w)) => {
                match quote!(#w).to_string().deref() {
                    "layout" => {
                        res.is_layout = true;
                    }
                    "nozoom" => res.can_be_zoom = false,
                    "nofeature" => res.can_be_feature = false,
                    _ => {
                        panic!("Unknown field attribute {}", w);
                    }
                }
            }
            NestedMeta::Meta(Meta::NameValue(MetaNameValue { ref ident, lit: Lit::Str(ref l), .. })) => {
                if quote!(#ident).to_string() == "name" {
                    res.src_name = parse_str(l.value().deref()).unwrap();
                } else {
                    panic!("Unknown field attribute {}", ident);
                }
            }
            x @ _ => {
                panic!("Unknown meta attribute {:?}", x);
            }
        }
    }


    return res;
}

#[proc_macro_derive(Properties, attributes(properties, property))]
pub fn derive_layer_properties(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_layer_properties(&ast)
}


fn impl_layer_properties(ast: &DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;
    let mut style_layer_name: Ident = ast.ident.clone();

    let generics = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let fields = match ast.data {
        Data::Struct(DataStruct {
                         fields: Fields::Named(ref fields),
                         ..
                     }) => {
            &fields.named
        }
        _ => {
            panic!("#[derive(Property)] only defined for structs.");
        }
    };


    let meta_attrs = ast.attrs.iter()
        .flat_map(Attribute::interpret_meta)
        .flat_map(|meta| {
            match meta {
                Meta::List(MetaList { ref ident, ref nested, .. }) if ident == "properties" => {
                    nested.iter().cloned().collect()
                }
                _ => {
                    Vec::new()
                }
            }
        });


    for meta in meta_attrs {
        match meta {
            NestedMeta::Meta(Meta::Word(ref ident)) => {
                style_layer_name = ident.clone()
            }
            _ => {}
        }
    }


    let evaluations: Vec<_> = fields.iter()
        .map(|field| {
            let attrs: Vec<Meta> = field.attrs.iter()
                .flat_map(Attribute::interpret_meta).collect();

            let mut res = FieldPropertyData {
                name: field.ident.clone().unwrap(),
                is_layout: false,
                can_be_zoom: true,
                can_be_feature: true,
                src_name: field.ident.clone().unwrap(),
            };

            for a in attrs {
                match &a {
                    Meta::List(MetaList { ref ident, ref nested, .. }) if ident == "property" => {
                        res = get_property_data(field.ident.as_ref().unwrap(), &a);
                    }
                    _ => {}
                }
            }

            let FieldPropertyData {
                name,
                is_layout,
                can_be_zoom,
                can_be_feature,
                src_name,
            } = res;

            let access = if is_layout {
                quote!(layer.get_layout())
            } else {
                quote!(layer.get_paint())
            };
            let name_str = name.to_string();
            quote_spanned! { field.span()=> {

                    let expr = &#access.#src_name;
                    match evaluator.evaluate(&mut self.#name ,&expr, #can_be_zoom, #can_be_feature)  {
                        Ok(true) => {
                            modified = true;
                        }
                        Ok(false) => {

                        }
                        Err(e) => {
                            bail!("Error when evaluating {} : {:?}", #name_str,e);
                        },
                        _ => {}

                    }
                }
            }
        }).collect();

    //panic!("style layer: {:?} fields : {:?}", style_layer_name, evaluations);

    let res = quote! {
        impl #impl_generics ::map::render::property::Properties for #struct_name #ty_generics #where_clause {
            type SourceLayerType  = ::map::style::#style_layer_name;

            fn eval(&mut self, layer: &Self::SourceLayerType, evaluator : &::map::render::property::PropertiesEvaluator) -> Result<bool> {
                use map::style::StyleLayer;

                let mut modified = false;
                #(#evaluations)*
                Ok(modified)
            }
        }
    };

    //panic!("{}", res);

    return res.into();
}