#![recursion_limit = "128"]
extern crate proc_macro;

extern crate syn;

extern crate quote;

use proc_macro::TokenStream;

use syn::*;
use quote::*;


#[derive(Debug, Clone)]
enum SourceType {
    Layout,
    Paint,
    Custom,
}

#[derive(Debug, Clone)]
struct FieldPropertyData {
    /// Specifies field name in `Properties` implementor
    name: Ident,
    /// Whether this is layout or paint property
    src_type: SourceType,
    /// If `nozoom` attribute is specified, this is false, property can't be zoom dependent
    can_be_zoom: bool,
    /// If `nofeature` attribute is specified, this is false, property can't be feature data dependent
    can_be_feature: bool,
    /// Name of field in `layout` or `paint` struct of source style layer
    src_name: Ident,
    /// Name of generated property, passed to visitors, used to match this data to shader input, by default
    /// its the same as `name`
    prop_name: Ident,
}

impl FieldPropertyData {
    fn new(name: &Ident) -> Self {
        FieldPropertyData {
            name: name.clone(),
            src_type: SourceType::Paint,
            can_be_zoom: true,
            can_be_feature: true,
            src_name: name.clone(),
            prop_name: name.clone(),
        }
    }
}

fn get_property_data(name: &Ident, meta: &Meta) -> FieldPropertyData {
    use std::ops::Deref;

    let mut res = FieldPropertyData::new(&name);

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
                    "nozoom" => res.can_be_zoom = false,
                    "nofeature" => res.can_be_feature = false,
                    _ => {
                        panic!("Unknown field attribute {}", w);
                    }
                }
            }
            NestedMeta::Meta(Meta::NameValue(MetaNameValue { ref ident, lit: Lit::Str(ref l), .. })) => {
                match quote!(#ident).to_string().deref() {
                    "layout" => {
                        res.src_name = parse_str(l.value().deref()).unwrap();
                        res.src_type = SourceType::Layout;
                    }
                    "paint" => {
                        res.src_name = parse_str(l.value().deref()).unwrap();
                        res.src_type = SourceType::Paint;
                    }
                    "custom" => {
                        res.src_name = parse_str(l.value().deref()).unwrap();
                        res.src_type = SourceType::Custom;
                    }
                    "prop_name" => {
                        res.prop_name = parse_str(l.value().deref()).unwrap();
                    }
                    _ => {
                        panic!("Unknown field attribute : `{}`", ident);
                    }
                }
            }
            x @ _ => {
                panic!("Unknown meta attribute: ");
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


    let datas: Vec<_> = fields.iter()
        .map(|field| {
            let attrs: Vec<Meta> = field.attrs.iter()
                .flat_map(Attribute::interpret_meta).collect();

            let name = field.ident.clone().unwrap();

            let mut res = FieldPropertyData::new(&name);

            for a in attrs {
                match &a {
                    Meta::List(MetaList { ref ident, ref nested, .. }) if ident == "property" => {
                        res = get_property_data(field.ident.as_ref().unwrap(), &a);
                    }
                    _ => {}
                }
            }
            res
        }).collect();


    let evaluations: Vec<_> = datas.iter().map(|res| {
        let res = res.clone();

        let FieldPropertyData {
            name,
            src_type,
            can_be_zoom,
            can_be_feature,
            src_name,
            prop_name,
        } = res;

        let property_retrieval = match src_type {
            SourceType::Layout => {
                quote!(&layer.get_layout().#src_name)
            }
            SourceType::Paint => {
                quote!(&layer.get_paint().#src_name)
            }
            SourceType::Custom => {
                quote!(#src_name(layer))
            }
        };

        let name_str = name.to_string();
        quote! {
            let expr = #property_retrieval;
            match evaluator.evaluate(&mut self.#name , &expr, #can_be_zoom, #can_be_feature)  {
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
    }).collect();

    let visits: Vec<_> = datas.iter().map(|data| {
        let FieldPropertyData {
            name,
            src_type,
            can_be_zoom,
            can_be_feature,
            src_name,
            prop_name,
        } = data.clone();

        let property_retrieval = match src_type {
            SourceType::Layout => {
                quote!(layer.get_layout().#src_name)
            }
            SourceType::Paint => {
                quote!(layer.get_paint().#src_name)
            }
            SourceType::Custom => {
                quote!(#src_name(layer))
            }
        };

        let prop_name_str = prop_name.to_string();
        quote!(visitor.visit(#prop_name_str,  &#property_retrieval, &self.#name, #can_be_zoom, #can_be_feature))
    }).collect();

    let res = quote! {
        impl #impl_generics ::map::render::property::Properties for #struct_name #ty_generics #where_clause {
            type SourceLayerType  = ::map::style::#style_layer_name;

            fn accept<V: PropertiesVisitor>(&self, layer: &Self::SourceLayerType, visitor: &mut V)  {
                use ::map::render::property::*;
                use ::map::style::StyleLayer;

                #(#visits);*
            }


            fn eval(&mut self, layer: &Self::SourceLayerType, evaluator : &::map::render::property::PropertiesEvaluator) -> Result<bool> {
                use map::style::StyleLayer;

                let mut modified = false;
                #(#evaluations)*
                Ok(modified)
            }
        }

    };



    return res.into();
}