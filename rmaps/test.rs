#![feature(prelude_import)]
#![no_std]
//#![feature(custom_attribute)]
#![feature(slice_patterns)]
#![feature(proc_macro)]
#![feature(proc_macro_mod)]
#![feature(never_type)]
#![feature(associated_type_defaults)]
#![feature(box_syntax)]
#![allow(unused_imports)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
pub extern crate common;
pub extern crate css_color_parser;
pub extern crate mapbox_tiles;

pub extern crate act_codegen;

/*
pub extern crate act;
#[macro_use]
pub extern crate act_codegen;
*/

pub mod prelude {

    //map::storage::actor_impls::setup();
    //map::storage::setup_FileSource();
    pub use act_codegen::*;
    pub use common::export::*;
    pub fn start_in_thread<
        A: Actor<Context = Context<A>> + Send + 'static,
        F: FnOnce() -> A + Send + 'static,
    >(
        a: F,
    ) -> Addr<Syn, A> {
        let (tx, rx) = ::std::sync::mpsc::channel();
        ::std::thread::spawn(move || {
            let sys = System::new("aa");
            let actor = a();
            let addr = actor.start();
            let _ = tx.send(addr);
            let _ = sys.run();
        });
        rx.recv().unwrap()
    }
}
pub mod map {
    use prelude::*;
    pub mod render {
        use prelude::*;
    }
    pub mod layers {
        use prelude::*;
        pub mod background {
            use map::style;
            use prelude::*;
            pub struct BackgroundLayer {
                style_layer: style::BackgroundLayer,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::fmt::Debug for BackgroundLayer {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        BackgroundLayer {
                            style_layer: ref __self_0_0,
                        } => {
                            let mut debug_trait_builder = f.debug_struct("BackgroundLayer");
                            let _ = debug_trait_builder.field("style_layer", &&(*__self_0_0));
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            impl super::Layer for BackgroundLayer {
                fn render<S: glium::Surface>(&mut self, surface: &mut S) -> Result<()> {
                    let color = self.style_layer.paint.color.eval();
                    let c = color.to_rgba();
                    surface.clear_color(c[0], c[1], c[2], c[3]);
                    Ok(())
                }
            }
            impl BackgroundLayer {
                pub fn parse(layer: style::BackgroundLayer) -> Self {
                    return BackgroundLayer { style_layer: layer };
                }
            }
        }
        pub mod raster {
            use map::style;
            use prelude::*;
            pub struct RasterLayer {
                style_layer: style::RasterLayer,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::fmt::Debug for RasterLayer {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        RasterLayer {
                            style_layer: ref __self_0_0,
                        } => {
                            let mut debug_trait_builder = f.debug_struct("RasterLayer");
                            let _ = debug_trait_builder.field("style_layer", &&(*__self_0_0));
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            impl super::Layer for RasterLayer {
                fn render<S: glium::Surface>(&mut self, surface: &mut S) -> Result<()> {
                    Ok(())
                }
            }
            impl RasterLayer {
                pub fn parse(layer: style::RasterLayer) -> Self {
                    return RasterLayer { style_layer: layer };
                }
            }
        }
        pub mod fill {
            use map::style;
            use prelude::*;
            pub struct FillLayer {
                style_layer: style::FillLayer,
                shader_program: glium::Program,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::fmt::Debug for FillLayer {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        FillLayer {
                            style_layer: ref __self_0_0,
                            shader_program: ref __self_0_1,
                        } => {
                            let mut debug_trait_builder = f.debug_struct("FillLayer");
                            let _ = debug_trait_builder.field("style_layer", &&(*__self_0_0));
                            let _ = debug_trait_builder.field("shader_program", &&(*__self_0_1));
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            impl super::Layer for FillLayer {
                fn render<S: glium::Surface>(&mut self, surface: &mut S) -> Result<()> {
                    {
                        ::rt::begin_panic(
                            "not yet implemented",
                            &("rmaps/src/map/layers/fill.rs", 13u32, 9u32),
                        )
                    }
                }
            }
            impl FillLayer {
                pub fn parse(f: &glium::backend::Facade, layer: style::FillLayer) -> Self {
                    let mut shader_program = {
                        let context = ::backend::Facade::get_context(f);
                        let version = {
                            let num: u32 = 100;
                            ::Version(::Api::GlEs, (num / 100) as u8, ((num % 100) / 10) as u8)
                        };
                        if context.is_glsl_version_supported(&version) {
                            let __vertex_shader: &str = "";
                            let __tessellation_control_shader: Option<
                                &str,
                            > = None;
                            let __tessellation_evaluation_shader:
                                        Option<&str> = None;
                            let __geometry_shader: Option<&str> = None;
                            let __fragment_shader: &str = "";
                            let __outputs_srgb: bool = false;
                            let __uses_point_size: bool = false;
                            let __vertex_shader =
                                    "#version 100\n\nuniform highp mat4 matrix;\nattribute highp vec2 position;\nattribute highp vec3 color;\n\nvarying highp vec3 vColor;\n\nvoid main() {\n    gl_Position = vec4(position, 0.0, 1.0) * matrix;\n    vColor = color;\n}";
                            let __fragment_shader =
                                    "#version 100\nvarying highp vec3 vColor;\nvoid main() {\n    gl_FragColor = vec4(vColor, 1.0);\n}";
                            let input = ::program::ProgramCreationInput::SourceCode {
                                vertex_shader: __vertex_shader,
                                tessellation_control_shader: __tessellation_control_shader,
                                tessellation_evaluation_shader: __tessellation_evaluation_shader,
                                geometry_shader: __geometry_shader,
                                fragment_shader: __fragment_shader,
                                transform_feedback_varyings: None,
                                outputs_srgb: __outputs_srgb,
                                uses_point_size: __uses_point_size,
                            };
                            ::program::Program::new(context, input)
                                .map_err(|err| ::program::ProgramChooserCreationError::from(err))
                        } else {
                            Err(::program::ProgramChooserCreationError::NoVersion)
                        }
                    };
                    FillLayer {
                        style_layer: layer,
                        shader_program: shader_program.unwrap(),
                    }
                }
            }
        }
        pub trait Layer: Sized + Debug {
            fn render<S: glium::Surface>(&mut self, surface: &mut S) -> Result<()>;
        }
        pub enum LayerHolder {
            Background(background::BackgroundLayer),
            Raster(raster::RasterLayer),
            Fill(fill::FillLayer),
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for LayerHolder {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match (&*self,) {
                    (&LayerHolder::Background(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("Background");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&LayerHolder::Raster(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("Raster");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&LayerHolder::Fill(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("Fill");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        impl Layer for LayerHolder {
            fn render<S: glium::Surface>(&mut self, surface: &mut S) -> Result<()> {
                match self {
                    LayerHolder::Background(b) => b.render(surface),
                    LayerHolder::Raster(r) => r.render(surface),
                    _ => Ok(()),
                }
            }
        }
        use super::style::*;
        pub fn parse_style_layers(
            facade: &glium::backend::Facade,
            style: &super::style::Style,
        ) -> Vec<LayerHolder> {
            let mut res = <[_]>::into_vec(box []);
            for l in style.layers.iter() {
                match l {
                    StyleLayer::Background(l) => res.push(LayerHolder::Background(
                        background::BackgroundLayer::parse(l.clone()),
                    )),
                    StyleLayer::Fill(l) => {
                        res.push(LayerHolder::Fill(fill::FillLayer::parse(facade, l.clone())))
                    }
                    StyleLayer::Raster(l) => {
                        res.push(LayerHolder::Raster(raster::RasterLayer::parse(l.clone())))
                    }
                    _ => {}
                }
            }
            res
        }
    }
    pub mod style {
        use common::json;
        use prelude::*;
        mod expr {
            use prelude::*;
            #[serde(untagged)]
            pub enum Value {
                String(String),
                Num(f32),
                Bool(bool),
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::fmt::Debug for Value {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match (&*self,) {
                        (&Value::String(ref __self_0),) => {
                            let mut debug_trait_builder = f.debug_tuple("String");
                            let _ = debug_trait_builder.field(&&(*__self_0));
                            debug_trait_builder.finish()
                        }
                        (&Value::Num(ref __self_0),) => {
                            let mut debug_trait_builder = f.debug_tuple("Num");
                            let _ = debug_trait_builder.field(&&(*__self_0));
                            debug_trait_builder.finish()
                        }
                        (&Value::Bool(ref __self_0),) => {
                            let mut debug_trait_builder = f.debug_tuple("Bool");
                            let _ = debug_trait_builder.field(&&(*__self_0));
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _IMPL_DESERIALIZE_FOR_Value: () = {
                extern crate serde as _serde;
                #[allow(unused_macros)]
                macro_rules! try(( $ __expr : expr ) => {
                                     match $ __expr {
                                     _serde :: export :: Ok ( __val ) => __val
                                     , _serde :: export :: Err ( __err ) => {
                                     return _serde :: export :: Err ( __err )
                                     ; } } });
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for Value {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        let __content =
                            match <_serde::private::de::Content as _serde::Deserialize>::deserialize(
                                __deserializer,
                            ) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                        if let _serde::export::Ok(__ok) = _serde::export::Result::map(
                            <String as _serde::Deserialize>::deserialize(
                                _serde::private::de::ContentRefDeserializer::<__D::Error>::new(
                                    &__content,
                                ),
                            ),
                            Value::String,
                        ) {
                            return _serde::export::Ok(__ok);
                        }
                        if let _serde::export::Ok(__ok) = _serde::export::Result::map(
                            <f32 as _serde::Deserialize>::deserialize(
                                _serde::private::de::ContentRefDeserializer::<__D::Error>::new(
                                    &__content,
                                ),
                            ),
                            Value::Num,
                        ) {
                            return _serde::export::Ok(__ok);
                        }
                        if let _serde::export::Ok(__ok) = _serde::export::Result::map(
                            <bool as _serde::Deserialize>::deserialize(
                                _serde::private::de::ContentRefDeserializer::<__D::Error>::new(
                                    &__content,
                                ),
                            ),
                            Value::Bool,
                        ) {
                            return _serde::export::Ok(__ok);
                        }
                        _serde::export::Err(_serde::de::Error::custom(
                            "data did not match any variant of untagged enum Value",
                        ))
                    }
                }
            };
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::clone::Clone for Value {
                #[inline]
                fn clone(&self) -> Value {
                    match (&*self,) {
                        (&Value::String(ref __self_0),) => {
                            Value::String(::std::clone::Clone::clone(&(*__self_0)))
                        }
                        (&Value::Num(ref __self_0),) => {
                            Value::Num(::std::clone::Clone::clone(&(*__self_0)))
                        }
                        (&Value::Bool(ref __self_0),) => {
                            Value::Bool(::std::clone::Clone::clone(&(*__self_0)))
                        }
                    }
                }
            }
            #[serde(untagged)]
            pub enum PropKey {
                #[serde(rename = "$type")]
                Type,

                #[serde(rename = "$id")]
                Id,
                Key(String),
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::fmt::Debug for PropKey {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match (&*self,) {
                        (&PropKey::Type,) => {
                            let mut debug_trait_builder = f.debug_tuple("Type");
                            debug_trait_builder.finish()
                        }
                        (&PropKey::Id,) => {
                            let mut debug_trait_builder = f.debug_tuple("Id");
                            debug_trait_builder.finish()
                        }
                        (&PropKey::Key(ref __self_0),) => {
                            let mut debug_trait_builder = f.debug_tuple("Key");
                            let _ = debug_trait_builder.field(&&(*__self_0));
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _IMPL_DESERIALIZE_FOR_PropKey: () = {
                extern crate serde as _serde;
                #[allow(unused_macros)]
                macro_rules! try(( $ __expr : expr ) => {
                                     match $ __expr {
                                     _serde :: export :: Ok ( __val ) => __val
                                     , _serde :: export :: Err ( __err ) => {
                                     return _serde :: export :: Err ( __err )
                                     ; } } });
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for PropKey {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        let __content =
                            match <_serde::private::de::Content as _serde::Deserialize>::deserialize(
                                __deserializer,
                            ) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                        if let _serde::export::Ok(__ok) =
                            match _serde::Deserializer::deserialize_any(
                                _serde::private::de::ContentRefDeserializer::<__D::Error>::new(
                                    &__content,
                                ),
                                _serde::private::de::UntaggedUnitVisitor::new("PropKey", "Type"),
                            ) {
                                _serde::export::Ok(()) => _serde::export::Ok(PropKey::Type),
                                _serde::export::Err(__err) => _serde::export::Err(__err),
                            } {
                            return _serde::export::Ok(__ok);
                        }
                        if let _serde::export::Ok(__ok) =
                            match _serde::Deserializer::deserialize_any(
                                _serde::private::de::ContentRefDeserializer::<__D::Error>::new(
                                    &__content,
                                ),
                                _serde::private::de::UntaggedUnitVisitor::new("PropKey", "Id"),
                            ) {
                                _serde::export::Ok(()) => _serde::export::Ok(PropKey::Id),
                                _serde::export::Err(__err) => _serde::export::Err(__err),
                            } {
                            return _serde::export::Ok(__ok);
                        }
                        if let _serde::export::Ok(__ok) = _serde::export::Result::map(
                            <String as _serde::Deserialize>::deserialize(
                                _serde::private::de::ContentRefDeserializer::<__D::Error>::new(
                                    &__content,
                                ),
                            ),
                            PropKey::Key,
                        ) {
                            return _serde::export::Ok(__ok);
                        }
                        _serde::export::Err(_serde::de::Error::custom(
                            "data did not match any variant of untagged enum PropKey",
                        ))
                    }
                }
            };
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::clone::Clone for PropKey {
                #[inline]
                fn clone(&self) -> PropKey {
                    match (&*self,) {
                        (&PropKey::Type,) => PropKey::Type,
                        (&PropKey::Id,) => PropKey::Id,
                        (&PropKey::Key(ref __self_0),) => {
                            PropKey::Key(::std::clone::Clone::clone(&(*__self_0)))
                        }
                    }
                }
            }
            pub enum Filter {
                Raw(bool),
                Has(PropKey),
                NotHas(PropKey),
                In(PropKey, Vec<Value>),
                NotIn(PropKey, Vec<Value>),
                Eq(PropKey, Value),
                Neq(PropKey, Value),
                Gt(PropKey, Value),
                Geq(PropKey, Value),
                Lt(PropKey, Value),
                Leq(PropKey, Value),
                All(Vec<Filter>),
                Any(Vec<Filter>),
                None(Vec<Filter>),
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::fmt::Debug for Filter {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match (&*self,) {
                        (&Filter::Raw(ref __self_0),) => {
                            let mut debug_trait_builder = f.debug_tuple("Raw");
                            let _ = debug_trait_builder.field(&&(*__self_0));
                            debug_trait_builder.finish()
                        }
                        (&Filter::Has(ref __self_0),) => {
                            let mut debug_trait_builder = f.debug_tuple("Has");
                            let _ = debug_trait_builder.field(&&(*__self_0));
                            debug_trait_builder.finish()
                        }
                        (&Filter::NotHas(ref __self_0),) => {
                            let mut debug_trait_builder = f.debug_tuple("NotHas");
                            let _ = debug_trait_builder.field(&&(*__self_0));
                            debug_trait_builder.finish()
                        }
                        (&Filter::In(ref __self_0, ref __self_1),) => {
                            let mut debug_trait_builder = f.debug_tuple("In");
                            let _ = debug_trait_builder.field(&&(*__self_0));
                            let _ = debug_trait_builder.field(&&(*__self_1));
                            debug_trait_builder.finish()
                        }
                        (&Filter::NotIn(ref __self_0, ref __self_1),) => {
                            let mut debug_trait_builder = f.debug_tuple("NotIn");
                            let _ = debug_trait_builder.field(&&(*__self_0));
                            let _ = debug_trait_builder.field(&&(*__self_1));
                            debug_trait_builder.finish()
                        }
                        (&Filter::Eq(ref __self_0, ref __self_1),) => {
                            let mut debug_trait_builder = f.debug_tuple("Eq");
                            let _ = debug_trait_builder.field(&&(*__self_0));
                            let _ = debug_trait_builder.field(&&(*__self_1));
                            debug_trait_builder.finish()
                        }
                        (&Filter::Neq(ref __self_0, ref __self_1),) => {
                            let mut debug_trait_builder = f.debug_tuple("Neq");
                            let _ = debug_trait_builder.field(&&(*__self_0));
                            let _ = debug_trait_builder.field(&&(*__self_1));
                            debug_trait_builder.finish()
                        }
                        (&Filter::Gt(ref __self_0, ref __self_1),) => {
                            let mut debug_trait_builder = f.debug_tuple("Gt");
                            let _ = debug_trait_builder.field(&&(*__self_0));
                            let _ = debug_trait_builder.field(&&(*__self_1));
                            debug_trait_builder.finish()
                        }
                        (&Filter::Geq(ref __self_0, ref __self_1),) => {
                            let mut debug_trait_builder = f.debug_tuple("Geq");
                            let _ = debug_trait_builder.field(&&(*__self_0));
                            let _ = debug_trait_builder.field(&&(*__self_1));
                            debug_trait_builder.finish()
                        }
                        (&Filter::Lt(ref __self_0, ref __self_1),) => {
                            let mut debug_trait_builder = f.debug_tuple("Lt");
                            let _ = debug_trait_builder.field(&&(*__self_0));
                            let _ = debug_trait_builder.field(&&(*__self_1));
                            debug_trait_builder.finish()
                        }
                        (&Filter::Leq(ref __self_0, ref __self_1),) => {
                            let mut debug_trait_builder = f.debug_tuple("Leq");
                            let _ = debug_trait_builder.field(&&(*__self_0));
                            let _ = debug_trait_builder.field(&&(*__self_1));
                            debug_trait_builder.finish()
                        }
                        (&Filter::All(ref __self_0),) => {
                            let mut debug_trait_builder = f.debug_tuple("All");
                            let _ = debug_trait_builder.field(&&(*__self_0));
                            debug_trait_builder.finish()
                        }
                        (&Filter::Any(ref __self_0),) => {
                            let mut debug_trait_builder = f.debug_tuple("Any");
                            let _ = debug_trait_builder.field(&&(*__self_0));
                            debug_trait_builder.finish()
                        }
                        (&Filter::None(ref __self_0),) => {
                            let mut debug_trait_builder = f.debug_tuple("None");
                            let _ = debug_trait_builder.field(&&(*__self_0));
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::clone::Clone for Filter {
                #[inline]
                fn clone(&self) -> Filter {
                    match (&*self,) {
                        (&Filter::Raw(ref __self_0),) => {
                            Filter::Raw(::std::clone::Clone::clone(&(*__self_0)))
                        }
                        (&Filter::Has(ref __self_0),) => {
                            Filter::Has(::std::clone::Clone::clone(&(*__self_0)))
                        }
                        (&Filter::NotHas(ref __self_0),) => {
                            Filter::NotHas(::std::clone::Clone::clone(&(*__self_0)))
                        }
                        (&Filter::In(ref __self_0, ref __self_1),) => Filter::In(
                            ::std::clone::Clone::clone(&(*__self_0)),
                            ::std::clone::Clone::clone(&(*__self_1)),
                        ),
                        (&Filter::NotIn(ref __self_0, ref __self_1),) => Filter::NotIn(
                            ::std::clone::Clone::clone(&(*__self_0)),
                            ::std::clone::Clone::clone(&(*__self_1)),
                        ),
                        (&Filter::Eq(ref __self_0, ref __self_1),) => Filter::Eq(
                            ::std::clone::Clone::clone(&(*__self_0)),
                            ::std::clone::Clone::clone(&(*__self_1)),
                        ),
                        (&Filter::Neq(ref __self_0, ref __self_1),) => Filter::Neq(
                            ::std::clone::Clone::clone(&(*__self_0)),
                            ::std::clone::Clone::clone(&(*__self_1)),
                        ),
                        (&Filter::Gt(ref __self_0, ref __self_1),) => Filter::Gt(
                            ::std::clone::Clone::clone(&(*__self_0)),
                            ::std::clone::Clone::clone(&(*__self_1)),
                        ),
                        (&Filter::Geq(ref __self_0, ref __self_1),) => Filter::Geq(
                            ::std::clone::Clone::clone(&(*__self_0)),
                            ::std::clone::Clone::clone(&(*__self_1)),
                        ),
                        (&Filter::Lt(ref __self_0, ref __self_1),) => Filter::Lt(
                            ::std::clone::Clone::clone(&(*__self_0)),
                            ::std::clone::Clone::clone(&(*__self_1)),
                        ),
                        (&Filter::Leq(ref __self_0, ref __self_1),) => Filter::Leq(
                            ::std::clone::Clone::clone(&(*__self_0)),
                            ::std::clone::Clone::clone(&(*__self_1)),
                        ),
                        (&Filter::All(ref __self_0),) => {
                            Filter::All(::std::clone::Clone::clone(&(*__self_0)))
                        }
                        (&Filter::Any(ref __self_0),) => {
                            Filter::Any(::std::clone::Clone::clone(&(*__self_0)))
                        }
                        (&Filter::None(ref __self_0),) => {
                            Filter::None(::std::clone::Clone::clone(&(*__self_0)))
                        }
                    }
                }
            }
            use common::json;
            use common::serde::{self, Deserialize, Deserializer, Serialize, Serializer};
            fn from_jvalue<T: ::common::serde::de::DeserializeOwned>(
                val: &json::Value,
            ) -> StdResult<T, json::Error> {
                return json::from_value(val.clone());
            }
            impl<'de> Deserialize<'de> for Filter {
                fn deserialize<D>(
                    deserializer: D,
                ) -> StdResult<Self, <D as Deserializer<'de>>::Error>
                where
                    D: Deserializer<'de>,
                {
                    #[serde(untagged)]
                    enum Help {
                        Bool(bool),
                        Arr(Vec<json::Value>),
                    }
                    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
                    const _IMPL_DESERIALIZE_FOR_Help: () = {
                        extern crate serde as _serde;
                        #[allow(unused_macros)]
                        macro_rules! try(( $ __expr : expr ) => {
                                             match $ __expr {
                                             _serde :: export :: Ok ( __val )
                                             => __val , _serde :: export ::
                                             Err ( __err ) => {
                                             return _serde :: export :: Err (
                                             __err ) ; } } });
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for Help {
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::export::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                let __content =
                                        match <_serde::private::de::Content as
                                                  _serde::Deserialize>::deserialize(__deserializer)
                                            {
                                            _serde::export::Ok(__val) =>
                                            __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        };
                                if let _serde::export::Ok(__ok) =
                                           _serde::export::Result::map(<bool
                                                                           as
                                                                           _serde::Deserialize>::deserialize(_serde::private::de::ContentRefDeserializer::<__D::Error>::new(&__content)),
                                                                       Help::Bool)
                                           {
                                        return _serde::export::Ok(__ok);
                                    }
                                if let _serde::export::Ok(__ok) =
                                           _serde::export::Result::map(<Vec<json::Value>
                                                                           as
                                                                           _serde::Deserialize>::deserialize(_serde::private::de::ContentRefDeserializer::<__D::Error>::new(&__content)),
                                                                       Help::Arr)
                                           {
                                        return _serde::export::Ok(__ok);
                                    }
                                _serde::export::Err(_serde::de::Error::custom(
                                    "data did not match any variant of untagged enum Help",
                                ))
                            }
                        }
                    };
                    let data: Help = Deserialize::deserialize(deserializer)?;
                    if let Help::Bool(b) = data {
                        return Ok(Filter::Raw(b));
                    }
                    let mut data = if let Help::Arr(d) = data {
                        d
                    } else {
                        {
                            {
                                ::rt::begin_panic(
                                    "explicit panic",
                                    &("rmaps/src/map/style/expr.rs", 69u32, 13u32),
                                )
                            }
                        }
                    };
                    let serde_err = |e| serde::de::Error::custom("Invalid filter");
                    match data[..] {
                        [json::Value::String(ref first), ref mut rest..] => {
                            return Ok(match (first.as_ref(), rest) {
                                ("has", [key]) => Filter::Has(from_jvalue(key).map_err(serde_err)?),
                                ("!has", [key]) => {
                                    Filter::NotHas(from_jvalue(key).map_err(serde_err)?)
                                }
                                ("==", [key, value]) => Filter::Eq(
                                    from_jvalue(key).map_err(serde_err)?,
                                    from_jvalue(value).map_err(serde_err)?,
                                ),
                                ("!=", [key, value]) => Filter::Neq(
                                    from_jvalue(key).map_err(serde_err)?,
                                    from_jvalue(value).map_err(serde_err)?,
                                ),
                                (">", [key, value]) => Filter::Gt(
                                    from_jvalue(key).map_err(serde_err)?,
                                    from_jvalue(value).map_err(serde_err)?,
                                ),
                                (">=", [key, value]) => Filter::Geq(
                                    from_jvalue(key).map_err(serde_err)?,
                                    from_jvalue(value).map_err(serde_err)?,
                                ),
                                ("<", [key, value]) => Filter::Lt(
                                    from_jvalue(key).map_err(serde_err)?,
                                    from_jvalue(value).map_err(serde_err)?,
                                ),
                                ("<=", [key, value]) => Filter::Leq(
                                    from_jvalue(key).map_err(serde_err)?,
                                    from_jvalue(value).map_err(serde_err)?,
                                ),
                                ("in", [key, rest..]) => {
                                    let vals = rest
                                        .iter()
                                        .map(|v| from_jvalue(v).map_err(serde_err))
                                        .collect::<StdResult<Vec<_>, _>>()?;
                                    Filter::In(from_jvalue(key).map_err(serde_err)?, vals)
                                }
                                ("!in", [key, rest..]) => {
                                    let vals = rest
                                        .iter()
                                        .map(|v| from_jvalue(v).map_err(serde_err))
                                        .collect::<StdResult<Vec<_>, _>>()?;
                                    Filter::NotIn(from_jvalue(key).map_err(serde_err)?, vals)
                                }
                                ("all", rest) => {
                                    let filters = rest
                                        .iter()
                                        .map(|v| from_jvalue(v).map_err(serde_err))
                                        .collect::<StdResult<Vec<Filter>, _>>()?;
                                    Filter::All(filters)
                                }
                                ("any", rest) => {
                                    let filters = rest
                                        .iter()
                                        .map(|v| from_jvalue(v).map_err(serde_err))
                                        .collect::<StdResult<Vec<Filter>, _>>()?;
                                    Filter::Any(filters)
                                }
                                ("none", rest) => {
                                    let filters = rest
                                        .iter()
                                        .map(|v| from_jvalue(v).map_err(serde_err))
                                        .collect::<StdResult<Vec<Filter>, _>>()?;
                                    Filter::None(filters)
                                }
                                _ => {
                                    return Err(serde::de::Error::custom("Invalid filter"));
                                }
                            });
                        }
                        _ => {}
                    }
                    {
                        ::rt::begin_panic(
                            "not yet implemented",
                            &("rmaps/src/map/style/expr.rs", 163u32, 9u32),
                        )
                    }
                }
            }
        }
        mod function {
            use common::json;
            use common::serde::{self, de::DeserializeOwned, Deserialize, Deserializer, Serialize};
            use prelude::*;
            pub enum FunctionStop<T: DeserializeOwned + Clone> {
                Value(f32, T),
                ValueAndZoom { value: f32, zoom: f32, res: T },
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl<T: ::std::fmt::Debug + DeserializeOwned + Clone> ::std::fmt::Debug for FunctionStop<T> {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match (&*self,) {
                        (&FunctionStop::Value(ref __self_0, ref __self_1),) => {
                            let mut debug_trait_builder = f.debug_tuple("Value");
                            let _ = debug_trait_builder.field(&&(*__self_0));
                            let _ = debug_trait_builder.field(&&(*__self_1));
                            debug_trait_builder.finish()
                        }
                        (&FunctionStop::ValueAndZoom {
                            value: ref __self_0,
                            zoom: ref __self_1,
                            res: ref __self_2,
                        },) => {
                            let mut debug_trait_builder = f.debug_struct("ValueAndZoom");
                            let _ = debug_trait_builder.field("value", &&(*__self_0));
                            let _ = debug_trait_builder.field("zoom", &&(*__self_1));
                            let _ = debug_trait_builder.field("res", &&(*__self_2));
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl<T: ::std::clone::Clone + DeserializeOwned + Clone> ::std::clone::Clone for FunctionStop<T> {
                #[inline]
                fn clone(&self) -> FunctionStop<T> {
                    match (&*self,) {
                        (&FunctionStop::Value(ref __self_0, ref __self_1),) => FunctionStop::Value(
                            ::std::clone::Clone::clone(&(*__self_0)),
                            ::std::clone::Clone::clone(&(*__self_1)),
                        ),
                        (&FunctionStop::ValueAndZoom {
                            value: ref __self_0,
                            zoom: ref __self_1,
                            res: ref __self_2,
                        },) => FunctionStop::ValueAndZoom {
                            value: ::std::clone::Clone::clone(&(*__self_0)),
                            zoom: ::std::clone::Clone::clone(&(*__self_1)),
                            res: ::std::clone::Clone::clone(&(*__self_2)),
                        },
                    }
                }
            }
            fn from_jvalue<T: ::common::serde::de::DeserializeOwned>(
                val: &json::Value,
            ) -> StdResult<T, json::Error> {
                return json::from_value(val.clone());
            }
            impl<'de, T: DeserializeOwned + Clone> Deserialize<'de> for FunctionStop<T> {
                fn deserialize<D>(
                    deserializer: D,
                ) -> StdResult<Self, <D as Deserializer<'de>>::Error>
                where
                    D: Deserializer<'de>,
                {
                    let serde_err = |_e| serde::de::Error::custom("Invalid Function stop");
                    let data: Vec<json::Value> = Deserialize::deserialize(deserializer)?;
                    match data[..] {
                        [json::Value::Object(ref obj), ref x] => {
                            let zoom = obj.get("zoom").unwrap();
                            let value = obj.get("value").unwrap();
                            return Ok(FunctionStop::ValueAndZoom {
                                value: from_jvalue(value).map_err(serde_err)?,
                                zoom: from_jvalue(zoom).map_err(serde_err)?,
                                res: from_jvalue(&x).map_err(serde_err)?,
                            });
                        }
                        [json::Value::Number(ref n), ref x] => {
                            return Ok(FunctionStop::Value(
                                n.as_f64().unwrap() as _,
                                from_jvalue(&x).map_err(serde_err)?,
                            ));
                        }
                        _ => {
                            return Err(serde::de::Error::custom("Invalid Function stop"));
                        }
                    }
                }
            }
            #[serde(untagged)]
            pub enum FunctionType {
                Identity,
                Exponential,
                Interval,
                Categorical,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::fmt::Debug for FunctionType {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match (&*self,) {
                        (&FunctionType::Identity,) => {
                            let mut debug_trait_builder = f.debug_tuple("Identity");
                            debug_trait_builder.finish()
                        }
                        (&FunctionType::Exponential,) => {
                            let mut debug_trait_builder = f.debug_tuple("Exponential");
                            debug_trait_builder.finish()
                        }
                        (&FunctionType::Interval,) => {
                            let mut debug_trait_builder = f.debug_tuple("Interval");
                            debug_trait_builder.finish()
                        }
                        (&FunctionType::Categorical,) => {
                            let mut debug_trait_builder = f.debug_tuple("Categorical");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _IMPL_DESERIALIZE_FOR_FunctionType: () = {
                extern crate serde as _serde;
                #[allow(unused_macros)]
                macro_rules! try(( $ __expr : expr ) => {
                                     match $ __expr {
                                     _serde :: export :: Ok ( __val ) => __val
                                     , _serde :: export :: Err ( __err ) => {
                                     return _serde :: export :: Err ( __err )
                                     ; } } });
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for FunctionType {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        let __content =
                            match <_serde::private::de::Content as _serde::Deserialize>::deserialize(
                                __deserializer,
                            ) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                        if let _serde::export::Ok(__ok) =
                            match _serde::Deserializer::deserialize_any(
                                _serde::private::de::ContentRefDeserializer::<__D::Error>::new(
                                    &__content,
                                ),
                                _serde::private::de::UntaggedUnitVisitor::new(
                                    "FunctionType",
                                    "Identity",
                                ),
                            ) {
                                _serde::export::Ok(()) => {
                                    _serde::export::Ok(FunctionType::Identity)
                                }
                                _serde::export::Err(__err) => _serde::export::Err(__err),
                            } {
                            return _serde::export::Ok(__ok);
                        }
                        if let _serde::export::Ok(__ok) =
                            match _serde::Deserializer::deserialize_any(
                                _serde::private::de::ContentRefDeserializer::<__D::Error>::new(
                                    &__content,
                                ),
                                _serde::private::de::UntaggedUnitVisitor::new(
                                    "FunctionType",
                                    "Exponential",
                                ),
                            ) {
                                _serde::export::Ok(()) => {
                                    _serde::export::Ok(FunctionType::Exponential)
                                }
                                _serde::export::Err(__err) => _serde::export::Err(__err),
                            } {
                            return _serde::export::Ok(__ok);
                        }
                        if let _serde::export::Ok(__ok) =
                            match _serde::Deserializer::deserialize_any(
                                _serde::private::de::ContentRefDeserializer::<__D::Error>::new(
                                    &__content,
                                ),
                                _serde::private::de::UntaggedUnitVisitor::new(
                                    "FunctionType",
                                    "Interval",
                                ),
                            ) {
                                _serde::export::Ok(()) => {
                                    _serde::export::Ok(FunctionType::Interval)
                                }
                                _serde::export::Err(__err) => _serde::export::Err(__err),
                            } {
                            return _serde::export::Ok(__ok);
                        }
                        if let _serde::export::Ok(__ok) =
                            match _serde::Deserializer::deserialize_any(
                                _serde::private::de::ContentRefDeserializer::<__D::Error>::new(
                                    &__content,
                                ),
                                _serde::private::de::UntaggedUnitVisitor::new(
                                    "FunctionType",
                                    "Categorical",
                                ),
                            ) {
                                _serde::export::Ok(()) => {
                                    _serde::export::Ok(FunctionType::Categorical)
                                }
                                _serde::export::Err(__err) => _serde::export::Err(__err),
                            } {
                            return _serde::export::Ok(__ok);
                        }
                        _serde::export::Err(_serde::de::Error::custom(
                            "data did not match any variant of untagged enum FunctionType",
                        ))
                    }
                }
            };
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::clone::Clone for FunctionType {
                #[inline]
                fn clone(&self) -> FunctionType {
                    match (&*self,) {
                        (&FunctionType::Identity,) => FunctionType::Identity,
                        (&FunctionType::Exponential,) => FunctionType::Exponential,
                        (&FunctionType::Interval,) => FunctionType::Interval,
                        (&FunctionType::Categorical,) => FunctionType::Categorical,
                    }
                }
            }
            #[serde(untagged)]
            pub enum Function<T: DeserializeOwned + Clone> {
                #[serde(bound(deserialize = "T : DeserializeOwned"))]
                Raw(T),
                Interpolated {
                    property: Option<String>,
                    base: Option<f32>,
                    #[serde(rename = "type")]
                    typ: Option<String>,
                    #[serde(bound(deserialize = "T : DeserializeOwned"))]
                    default: Option<T>,
                    #[serde(rename = "colorSpace")]
                    color_space: Option<String>,
                    #[serde(bound(deserialize = "T : DeserializeOwned"))]
                    stops: Vec<FunctionStop<T>>,
                },
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl<T: ::std::fmt::Debug + DeserializeOwned + Clone> ::std::fmt::Debug for Function<T> {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match (&*self,) {
                        (&Function::Raw(ref __self_0),) => {
                            let mut debug_trait_builder = f.debug_tuple("Raw");
                            let _ = debug_trait_builder.field(&&(*__self_0));
                            debug_trait_builder.finish()
                        }
                        (&Function::Interpolated {
                            property: ref __self_0,
                            base: ref __self_1,
                            typ: ref __self_2,
                            default: ref __self_3,
                            color_space: ref __self_4,
                            stops: ref __self_5,
                        },) => {
                            let mut debug_trait_builder = f.debug_struct("Interpolated");
                            let _ = debug_trait_builder.field("property", &&(*__self_0));
                            let _ = debug_trait_builder.field("base", &&(*__self_1));
                            let _ = debug_trait_builder.field("typ", &&(*__self_2));
                            let _ = debug_trait_builder.field("default", &&(*__self_3));
                            let _ = debug_trait_builder.field("color_space", &&(*__self_4));
                            let _ = debug_trait_builder.field("stops", &&(*__self_5));
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _IMPL_DESERIALIZE_FOR_Function: () = {
                extern crate serde as _serde;
                #[allow(unused_macros)]
                macro_rules! try(( $ __expr : expr ) => {
                                     match $ __expr {
                                     _serde :: export :: Ok ( __val ) => __val
                                     , _serde :: export :: Err ( __err ) => {
                                     return _serde :: export :: Err ( __err )
                                     ; } } });
                #[automatically_derived]
                impl<'de, T: DeserializeOwned + Clone> _serde::Deserialize<'de> for Function<T>
                where
                    T: DeserializeOwned,
                    T: DeserializeOwned,
                    T: DeserializeOwned,
                {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        let __content =
                            match <_serde::private::de::Content as _serde::Deserialize>::deserialize(
                                __deserializer,
                            ) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                        if let _serde::export::Ok(__ok) = _serde::export::Result::map(
                            <T as _serde::Deserialize>::deserialize(
                                _serde::private::de::ContentRefDeserializer::<__D::Error>::new(
                                    &__content,
                                ),
                            ),
                            Function::Raw,
                        ) {
                            return _serde::export::Ok(__ok);
                        }
                        if let _serde::export::Ok(__ok) = {
                            #[allow(non_camel_case_types)]
                            enum __Field {
                                __field0,
                                __field1,
                                __field2,
                                __field3,
                                __field4,
                                __field5,
                                __ignore,
                            }
                            struct __FieldVisitor;
                            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                type Value = __Field;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::export::Formatter,
                                ) -> _serde::export::fmt::Result {
                                    _serde::export::Formatter::write_str(
                                        __formatter,
                                        "field identifier",
                                    )
                                }
                                fn visit_u64<__E>(
                                    self,
                                    __value: u64,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        0u64 => _serde::export::Ok(__Field::__field0),
                                        1u64 => _serde::export::Ok(__Field::__field1),
                                        2u64 => _serde::export::Ok(__Field::__field2),
                                        3u64 => _serde::export::Ok(__Field::__field3),
                                        4u64 => _serde::export::Ok(__Field::__field4),
                                        5u64 => _serde::export::Ok(__Field::__field5),
                                        _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                            _serde::de::Unexpected::Unsigned(__value),
                                            &"field index 0 <= i < 6",
                                        )),
                                    }
                                }
                                fn visit_str<__E>(
                                    self,
                                    __value: &str,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        "property" => _serde::export::Ok(__Field::__field0),
                                        "base" => _serde::export::Ok(__Field::__field1),
                                        "type" => _serde::export::Ok(__Field::__field2),
                                        "default" => _serde::export::Ok(__Field::__field3),
                                        "colorSpace" => _serde::export::Ok(__Field::__field4),
                                        "stops" => _serde::export::Ok(__Field::__field5),
                                        _ => _serde::export::Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(
                                    self,
                                    __value: &[u8],
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        b"property" => _serde::export::Ok(__Field::__field0),
                                        b"base" => _serde::export::Ok(__Field::__field1),
                                        b"type" => _serde::export::Ok(__Field::__field2),
                                        b"default" => _serde::export::Ok(__Field::__field3),
                                        b"colorSpace" => _serde::export::Ok(__Field::__field4),
                                        b"stops" => _serde::export::Ok(__Field::__field5),
                                        _ => _serde::export::Ok(__Field::__ignore),
                                    }
                                }
                            }
                            impl<'de> _serde::Deserialize<'de> for __Field {
                                #[inline]
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::export::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::Deserializer::deserialize_identifier(
                                        __deserializer,
                                        __FieldVisitor,
                                    )
                                }
                            }
                            struct __Visitor<'de, T: DeserializeOwned + Clone>
                            where
                                T: DeserializeOwned,
                                T: DeserializeOwned,
                                T: DeserializeOwned,
                            {
                                marker: _serde::export::PhantomData<Function<T>>,
                                lifetime: _serde::export::PhantomData<&'de ()>,
                            }
                            impl<'de, T: DeserializeOwned + Clone> _serde::de::Visitor<'de> for __Visitor<'de, T>
                            where
                                T: DeserializeOwned,
                                T: DeserializeOwned,
                                T: DeserializeOwned,
                            {
                                type Value = Function<T>;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::export::Formatter,
                                ) -> _serde::export::fmt::Result {
                                    _serde::export::Formatter::write_str(
                                        __formatter,
                                        "struct variant Function::Interpolated",
                                    )
                                }
                                #[inline]
                                fn visit_map<__A>(
                                    self,
                                    mut __map: __A,
                                ) -> _serde::export::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::MapAccess<'de>,
                                {
                                    let mut __field0:
                                                       _serde::export::Option<Option<String>> =
                                                   _serde::export::None;
                                    let mut __field1:
                                                       _serde::export::Option<Option<f32>> =
                                                   _serde::export::None;
                                    let mut __field2:
                                                       _serde::export::Option<Option<String>> =
                                                   _serde::export::None;
                                    let mut __field3:
                                                       _serde::export::Option<Option<T>> =
                                                   _serde::export::None;
                                    let mut __field4:
                                                       _serde::export::Option<Option<String>> =
                                                   _serde::export::None;
                                    let mut __field5:
                                                       _serde::export::Option<Vec<FunctionStop<T>>> =
                                                   _serde::export::None;
                                    while let _serde::export::Some(__key) =
                                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        } {
                                        match __key {
                                            __Field::__field0 => {
                                                if _serde::export::Option::is_some(&__field0) {
                                                    return _serde::export::Err(<__A::Error
                                                                                              as
                                                                                              _serde::de::Error>::duplicate_field("property"));
                                                }
                                                __field0 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Option<String>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field1 => {
                                                if _serde::export::Option::is_some(&__field1) {
                                                    return _serde::export::Err(<__A::Error
                                                                                              as
                                                                                              _serde::de::Error>::duplicate_field("base"));
                                                }
                                                __field1 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Option<f32>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field2 => {
                                                if _serde::export::Option::is_some(&__field2) {
                                                    return _serde::export::Err(<__A::Error
                                                                                              as
                                                                                              _serde::de::Error>::duplicate_field("type"));
                                                }
                                                __field2 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Option<String>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field3 => {
                                                if _serde::export::Option::is_some(&__field3) {
                                                    return _serde::export::Err(<__A::Error
                                                                                              as
                                                                                              _serde::de::Error>::duplicate_field("default"));
                                                }
                                                __field3 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Option<T>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field4 => {
                                                if _serde::export::Option::is_some(&__field4) {
                                                    return _serde::export::Err(<__A::Error
                                                                                              as
                                                                                              _serde::de::Error>::duplicate_field("colorSpace"));
                                                }
                                                __field4 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Option<String>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field5 => {
                                                if _serde::export::Option::is_some(&__field5) {
                                                    return _serde::export::Err(<__A::Error
                                                                                              as
                                                                                              _serde::de::Error>::duplicate_field("stops"));
                                                }
                                                __field5 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Vec<FunctionStop<T>>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            _ => {
                                                let _ = match _serde::de::MapAccess::next_value::<
                                                    _serde::de::IgnoredAny,
                                                >(
                                                    &mut __map
                                                ) {
                                                    _serde::export::Ok(__val) => __val,
                                                    _serde::export::Err(__err) => {
                                                        return _serde::export::Err(__err);
                                                    }
                                                };
                                            }
                                        }
                                    }
                                    let __field0 = match __field0 {
                                        _serde::export::Some(__field0) => __field0,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field("property") {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    let __field1 = match __field1 {
                                        _serde::export::Some(__field1) => __field1,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field("base") {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    let __field2 = match __field2 {
                                        _serde::export::Some(__field2) => __field2,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field("type") {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    let __field3 = match __field3 {
                                        _serde::export::Some(__field3) => __field3,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field("default") {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    let __field4 = match __field4 {
                                        _serde::export::Some(__field4) => __field4,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field("colorSpace") {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    let __field5 = match __field5 {
                                        _serde::export::Some(__field5) => __field5,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field("stops") {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    _serde::export::Ok(Function::Interpolated {
                                        property: __field0,
                                        base: __field1,
                                        typ: __field2,
                                        default: __field3,
                                        color_space: __field4,
                                        stops: __field5,
                                    })
                                }
                            }
                            const FIELDS: &'static [&'static str] =
                                &["property", "base", "type", "default", "colorSpace", "stops"];
                            _serde::Deserializer::deserialize_any(
                                _serde::private::de::ContentRefDeserializer::<__D::Error>::new(
                                    &__content,
                                ),
                                __Visitor {
                                    marker: _serde::export::PhantomData::<Function<T>>,
                                    lifetime: _serde::export::PhantomData,
                                },
                            )
                        } {
                            return _serde::export::Ok(__ok);
                        }
                        _serde::export::Err(_serde::de::Error::custom(
                            "data did not match any variant of untagged enum Function",
                        ))
                    }
                }
            };
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl<T: ::std::clone::Clone + DeserializeOwned + Clone> ::std::clone::Clone for Function<T> {
                #[inline]
                fn clone(&self) -> Function<T> {
                    match (&*self,) {
                        (&Function::Raw(ref __self_0),) => {
                            Function::Raw(::std::clone::Clone::clone(&(*__self_0)))
                        }
                        (&Function::Interpolated {
                            property: ref __self_0,
                            base: ref __self_1,
                            typ: ref __self_2,
                            default: ref __self_3,
                            color_space: ref __self_4,
                            stops: ref __self_5,
                        },) => Function::Interpolated {
                            property: ::std::clone::Clone::clone(&(*__self_0)),
                            base: ::std::clone::Clone::clone(&(*__self_1)),
                            typ: ::std::clone::Clone::clone(&(*__self_2)),
                            default: ::std::clone::Clone::clone(&(*__self_3)),
                            color_space: ::std::clone::Clone::clone(&(*__self_4)),
                            stops: ::std::clone::Clone::clone(&(*__self_5)),
                        },
                    }
                }
            }
            impl<T: DeserializeOwned + Clone> Function<T> {
                pub fn eval(&self) -> T {
                    match self {
                        Function::Raw(c) => c.clone(),
                        _ => {
                            ::rt::begin_panic(
                                "explicit panic",
                                &("rmaps/src/map/style/function.rs", 84u32, 18u32),
                            )
                        }
                    }
                }
            }
        }
        mod color {
            use common::serde::{self, Deserialize, Deserializer};
            use css_color_parser::{self, Color as CssColor};
            use prelude::*;
            pub struct Color(pub CssColor);
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::fmt::Debug for Color {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        Color(ref __self_0_0) => {
                            let mut debug_trait_builder = f.debug_tuple("Color");
                            let _ = debug_trait_builder.field(&&(*__self_0_0));
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::clone::Clone for Color {
                #[inline]
                fn clone(&self) -> Color {
                    match *self {
                        Color(ref __self_0_0) => Color(::std::clone::Clone::clone(&(*__self_0_0))),
                    }
                }
            }
            impl<'de> serde::Deserialize<'de> for Color {
                fn deserialize<D>(
                    deserializer: D,
                ) -> StdResult<Self, <D as Deserializer<'de>>::Error>
                where
                    D: Deserializer<'de>,
                {
                    use std::str::FromStr;
                    let data: String = Deserialize::deserialize(deserializer)?;
                    let color = CssColor::from_str(&data)
                        .map_err(|_| serde::de::Error::custom("Invalid color"))?;
                    Ok(Color(color))
                }
            }
            impl Color {
                pub fn to_rgba(&self) -> [f32; 4] {
                    return [
                        self.0.r as f32 / 255f32,
                        self.0.g as f32 / 255f32,
                        self.0.b as f32 / 255f32,
                        self.0.a,
                    ];
                }
            }
        }
        mod layers {
            use prelude::*;
            pub mod background {
                use super::super::color::Color;
                use super::super::function::Function;
                use super::{BaseLayout, LayerCommon, Visibility};
                use prelude::*;
                pub struct BackgroundLayer {
                    #[serde(flatten)]
                    pub common: LayerCommon,
                    #[serde(default = "Default::default")]
                    pub layout: BaseLayout,
                    #[serde(default = "Default::default")]
                    pub paint: BackgroundPaint,
                }
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl ::std::fmt::Debug for BackgroundLayer {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        match *self {
                            BackgroundLayer {
                                common: ref __self_0_0,
                                layout: ref __self_0_1,
                                paint: ref __self_0_2,
                            } => {
                                let mut debug_trait_builder = f.debug_struct("BackgroundLayer");
                                let _ = debug_trait_builder.field("common", &&(*__self_0_0));
                                let _ = debug_trait_builder.field("layout", &&(*__self_0_1));
                                let _ = debug_trait_builder.field("paint", &&(*__self_0_2));
                                debug_trait_builder.finish()
                            }
                        }
                    }
                }
                #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
                const _IMPL_DESERIALIZE_FOR_BackgroundLayer: () = {
                    extern crate serde as _serde;
                    #[allow(unused_macros)]
                    macro_rules! try(( $ __expr : expr ) => {
                                         match $ __expr {
                                         _serde :: export :: Ok ( __val ) =>
                                         __val , _serde :: export :: Err (
                                         __err ) => {
                                         return _serde :: export :: Err (
                                         __err ) ; } } });
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for BackgroundLayer {
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::export::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            #[allow(non_camel_case_types)]
                            enum __Field<'de> {
                                __field1,
                                __field2,
                                __other(_serde::private::de::Content<'de>),
                            }
                            struct __FieldVisitor;
                            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                type Value = __Field<'de>;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::export::Formatter,
                                ) -> _serde::export::fmt::Result {
                                    _serde::export::Formatter::write_str(
                                        __formatter,
                                        "field identifier",
                                    )
                                }
                                fn visit_bool<__E>(
                                    self,
                                    __value: bool,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::Bool(__value),
                                    ))
                                }
                                fn visit_i8<__E>(
                                    self,
                                    __value: i8,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::I8(__value),
                                    ))
                                }
                                fn visit_i16<__E>(
                                    self,
                                    __value: i16,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::I16(__value),
                                    ))
                                }
                                fn visit_i32<__E>(
                                    self,
                                    __value: i32,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::I32(__value),
                                    ))
                                }
                                fn visit_i64<__E>(
                                    self,
                                    __value: i64,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::I64(__value),
                                    ))
                                }
                                fn visit_u8<__E>(
                                    self,
                                    __value: u8,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::U8(__value),
                                    ))
                                }
                                fn visit_u16<__E>(
                                    self,
                                    __value: u16,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::U16(__value),
                                    ))
                                }
                                fn visit_u32<__E>(
                                    self,
                                    __value: u32,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::U32(__value),
                                    ))
                                }
                                fn visit_u64<__E>(
                                    self,
                                    __value: u64,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::U64(__value),
                                    ))
                                }
                                fn visit_f32<__E>(
                                    self,
                                    __value: f32,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::F32(__value),
                                    ))
                                }
                                fn visit_f64<__E>(
                                    self,
                                    __value: f64,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::F64(__value),
                                    ))
                                }
                                fn visit_char<__E>(
                                    self,
                                    __value: char,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::Char(__value),
                                    ))
                                }
                                fn visit_unit<__E>(self) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::Unit,
                                    ))
                                }
                                fn visit_borrowed_str<__E>(
                                    self,
                                    __value: &'de str,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        "layout" => _serde::export::Ok(__Field::__field1),
                                        "paint" => _serde::export::Ok(__Field::__field2),
                                        _ => {
                                            let __value =
                                                _serde::private::de::Content::Str(__value);
                                            _serde::export::Ok(__Field::__other(__value))
                                        }
                                    }
                                }
                                fn visit_borrowed_bytes<__E>(
                                    self,
                                    __value: &'de [u8],
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        b"layout" => _serde::export::Ok(__Field::__field1),
                                        b"paint" => _serde::export::Ok(__Field::__field2),
                                        _ => {
                                            let __value =
                                                _serde::private::de::Content::Bytes(__value);
                                            _serde::export::Ok(__Field::__other(__value))
                                        }
                                    }
                                }
                                fn visit_str<__E>(
                                    self,
                                    __value: &str,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        "layout" => _serde::export::Ok(__Field::__field1),
                                        "paint" => _serde::export::Ok(__Field::__field2),
                                        _ => {
                                            let __value = _serde::private::de::Content::String(
                                                __value.to_string(),
                                            );
                                            _serde::export::Ok(__Field::__other(__value))
                                        }
                                    }
                                }
                                fn visit_bytes<__E>(
                                    self,
                                    __value: &[u8],
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        b"layout" => _serde::export::Ok(__Field::__field1),
                                        b"paint" => _serde::export::Ok(__Field::__field2),
                                        _ => {
                                            let __value = _serde::private::de::Content::ByteBuf(
                                                __value.to_vec(),
                                            );
                                            _serde::export::Ok(__Field::__other(__value))
                                        }
                                    }
                                }
                            }
                            impl<'de> _serde::Deserialize<'de> for __Field<'de> {
                                #[inline]
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::export::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::Deserializer::deserialize_identifier(
                                        __deserializer,
                                        __FieldVisitor,
                                    )
                                }
                            }
                            struct __Visitor<'de> {
                                marker: _serde::export::PhantomData<BackgroundLayer>,
                                lifetime: _serde::export::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                type Value = BackgroundLayer;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::export::Formatter,
                                ) -> _serde::export::fmt::Result {
                                    _serde::export::Formatter::write_str(
                                        __formatter,
                                        "struct BackgroundLayer",
                                    )
                                }
                                #[inline]
                                fn visit_map<__A>(
                                    self,
                                    mut __map: __A,
                                ) -> _serde::export::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::MapAccess<'de>,
                                {
                                    let mut __field1:
                                                _serde::export::Option<BaseLayout> =
                                            _serde::export::None;
                                    let mut __field2:
                                                _serde::export::Option<BackgroundPaint> =
                                            _serde::export::None;
                                    let mut __collect = _serde::export::Vec::<
                                        _serde::export::Option<(
                                            _serde::private::de::Content,
                                            _serde::private::de::Content,
                                        )>,
                                    >::new(
                                    );
                                    while let _serde::export::Some(__key) =
                                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        } {
                                        match __key {
                                            __Field::__field1 => {
                                                if _serde::export::Option::is_some(&__field1) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("layout"));
                                                }
                                                __field1 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        BaseLayout,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field2 => {
                                                if _serde::export::Option::is_some(&__field2) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("paint"));
                                                }
                                                __field2 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        BackgroundPaint,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__other(__name) => {
                                                __collect.push(_serde::export::Some((
                                                    __name,
                                                    match _serde::de::MapAccess::next_value(
                                                        &mut __map,
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                )));
                                            }
                                        }
                                    }
                                    let __field1 = match __field1 {
                                        _serde::export::Some(__field1) => __field1,
                                        _serde::export::None => Default::default(),
                                    };
                                    let __field2 = match __field2 {
                                        _serde::export::Some(__field2) => __field2,
                                        _serde::export::None => Default::default(),
                                    };
                                    let __field0: LayerCommon =
                                        match _serde::de::Deserialize::deserialize(
                                            _serde::private::de::FlatMapDeserializer(
                                                &mut __collect,
                                                _serde::export::PhantomData,
                                            ),
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        };
                                    _serde::export::Ok(BackgroundLayer {
                                        common: __field0,
                                        layout: __field1,
                                        paint: __field2,
                                    })
                                }
                            }
                            _serde::Deserializer::deserialize_map(
                                __deserializer,
                                __Visitor {
                                    marker: _serde::export::PhantomData::<BackgroundLayer>,
                                    lifetime: _serde::export::PhantomData,
                                },
                            )
                        }
                    }
                };
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl ::std::clone::Clone for BackgroundLayer {
                    #[inline]
                    fn clone(&self) -> BackgroundLayer {
                        match *self {
                            BackgroundLayer {
                                common: ref __self_0_0,
                                layout: ref __self_0_1,
                                paint: ref __self_0_2,
                            } => BackgroundLayer {
                                common: ::std::clone::Clone::clone(&(*__self_0_0)),
                                layout: ::std::clone::Clone::clone(&(*__self_0_1)),
                                paint: ::std::clone::Clone::clone(&(*__self_0_2)),
                            },
                        }
                    }
                }
                pub struct BackgroundPaint {
                    #[serde(rename = "background-color")]
                    #[serde(default = "default_background_color")]
                    pub color: Function<Color>,
                    #[serde(rename = "background-opacity")]
                    #[serde(default = "default_backround_opacity")]
                    pub opacity: Function<f32>,
                    #[serde(rename = "background-pattern")]
                    pub pattern: Option<Function<String>>,
                }
                #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
                const _IMPL_DESERIALIZE_FOR_BackgroundPaint: () = {
                    extern crate serde as _serde;
                    #[allow(unused_macros)]
                    macro_rules! try(( $ __expr : expr ) => {
                                         match $ __expr {
                                         _serde :: export :: Ok ( __val ) =>
                                         __val , _serde :: export :: Err (
                                         __err ) => {
                                         return _serde :: export :: Err (
                                         __err ) ; } } });
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for BackgroundPaint {
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::export::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            #[allow(non_camel_case_types)]
                            enum __Field {
                                __field0,
                                __field1,
                                __field2,
                                __ignore,
                            }
                            struct __FieldVisitor;
                            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                type Value = __Field;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::export::Formatter,
                                ) -> _serde::export::fmt::Result {
                                    _serde::export::Formatter::write_str(
                                        __formatter,
                                        "field identifier",
                                    )
                                }
                                fn visit_u64<__E>(
                                    self,
                                    __value: u64,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        0u64 => _serde::export::Ok(__Field::__field0),
                                        1u64 => _serde::export::Ok(__Field::__field1),
                                        2u64 => _serde::export::Ok(__Field::__field2),
                                        _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                            _serde::de::Unexpected::Unsigned(__value),
                                            &"field index 0 <= i < 3",
                                        )),
                                    }
                                }
                                fn visit_str<__E>(
                                    self,
                                    __value: &str,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        "background-color" => _serde::export::Ok(__Field::__field0),
                                        "background-opacity" => {
                                            _serde::export::Ok(__Field::__field1)
                                        }
                                        "background-pattern" => {
                                            _serde::export::Ok(__Field::__field2)
                                        }
                                        _ => _serde::export::Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(
                                    self,
                                    __value: &[u8],
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        b"background-color" => {
                                            _serde::export::Ok(__Field::__field0)
                                        }
                                        b"background-opacity" => {
                                            _serde::export::Ok(__Field::__field1)
                                        }
                                        b"background-pattern" => {
                                            _serde::export::Ok(__Field::__field2)
                                        }
                                        _ => _serde::export::Ok(__Field::__ignore),
                                    }
                                }
                            }
                            impl<'de> _serde::Deserialize<'de> for __Field {
                                #[inline]
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::export::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::Deserializer::deserialize_identifier(
                                        __deserializer,
                                        __FieldVisitor,
                                    )
                                }
                            }
                            struct __Visitor<'de> {
                                marker: _serde::export::PhantomData<BackgroundPaint>,
                                lifetime: _serde::export::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                type Value = BackgroundPaint;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::export::Formatter,
                                ) -> _serde::export::fmt::Result {
                                    _serde::export::Formatter::write_str(
                                        __formatter,
                                        "struct BackgroundPaint",
                                    )
                                }
                                #[inline]
                                fn visit_seq<__A>(
                                    self,
                                    mut __seq: __A,
                                ) -> _serde::export::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::SeqAccess<'de>,
                                {
                                    let __field0 = match match _serde::de::SeqAccess::next_element::<
                                        Function<Color>,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &"struct BackgroundPaint with 3 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field1 = match match _serde::de::SeqAccess::next_element::<
                                        Function<f32>,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    1usize,
                                                    &"struct BackgroundPaint with 3 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field2 = match match _serde::de::SeqAccess::next_element::<
                                        Option<Function<String>>,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    2usize,
                                                    &"struct BackgroundPaint with 3 elements",
                                                ),
                                            );
                                        }
                                    };
                                    _serde::export::Ok(BackgroundPaint {
                                        color: __field0,
                                        opacity: __field1,
                                        pattern: __field2,
                                    })
                                }
                                #[inline]
                                fn visit_map<__A>(
                                    self,
                                    mut __map: __A,
                                ) -> _serde::export::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::MapAccess<'de>,
                                {
                                    let mut __field0:
                                                _serde::export::Option<Function<Color>> =
                                            _serde::export::None;
                                    let mut __field1:
                                                _serde::export::Option<Function<f32>> =
                                            _serde::export::None;
                                    let mut __field2:
                                                _serde::export::Option<Option<Function<String>>> =
                                            _serde::export::None;
                                    while let _serde::export::Some(__key) =
                                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        } {
                                        match __key {
                                            __Field::__field0 => {
                                                if _serde::export::Option::is_some(&__field0) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("background-color"));
                                                }
                                                __field0 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Function<Color>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field1 => {
                                                if _serde::export::Option::is_some(&__field1) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("background-opacity"));
                                                }
                                                __field1 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Function<f32>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field2 => {
                                                if _serde::export::Option::is_some(&__field2) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("background-pattern"));
                                                }
                                                __field2 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Option<Function<String>>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            _ => {
                                                let _ = match _serde::de::MapAccess::next_value::<
                                                    _serde::de::IgnoredAny,
                                                >(
                                                    &mut __map
                                                ) {
                                                    _serde::export::Ok(__val) => __val,
                                                    _serde::export::Err(__err) => {
                                                        return _serde::export::Err(__err);
                                                    }
                                                };
                                            }
                                        }
                                    }
                                    let __field0 = match __field0 {
                                        _serde::export::Some(__field0) => __field0,
                                        _serde::export::None => default_background_color(),
                                    };
                                    let __field1 = match __field1 {
                                        _serde::export::Some(__field1) => __field1,
                                        _serde::export::None => default_backround_opacity(),
                                    };
                                    let __field2 = match __field2 {
                                        _serde::export::Some(__field2) => __field2,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field(
                                                "background-pattern",
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    _serde::export::Ok(BackgroundPaint {
                                        color: __field0,
                                        opacity: __field1,
                                        pattern: __field2,
                                    })
                                }
                            }
                            const FIELDS: &'static [&'static str] = &[
                                "background-color",
                                "background-opacity",
                                "background-pattern",
                            ];
                            _serde::Deserializer::deserialize_struct(
                                __deserializer,
                                "BackgroundPaint",
                                FIELDS,
                                __Visitor {
                                    marker: _serde::export::PhantomData::<BackgroundPaint>,
                                    lifetime: _serde::export::PhantomData,
                                },
                            )
                        }
                    }
                };
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl ::std::fmt::Debug for BackgroundPaint {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        match *self {
                            BackgroundPaint {
                                color: ref __self_0_0,
                                opacity: ref __self_0_1,
                                pattern: ref __self_0_2,
                            } => {
                                let mut debug_trait_builder = f.debug_struct("BackgroundPaint");
                                let _ = debug_trait_builder.field("color", &&(*__self_0_0));
                                let _ = debug_trait_builder.field("opacity", &&(*__self_0_1));
                                let _ = debug_trait_builder.field("pattern", &&(*__self_0_2));
                                debug_trait_builder.finish()
                            }
                        }
                    }
                }
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl ::std::clone::Clone for BackgroundPaint {
                    #[inline]
                    fn clone(&self) -> BackgroundPaint {
                        match *self {
                            BackgroundPaint {
                                color: ref __self_0_0,
                                opacity: ref __self_0_1,
                                pattern: ref __self_0_2,
                            } => BackgroundPaint {
                                color: ::std::clone::Clone::clone(&(*__self_0_0)),
                                opacity: ::std::clone::Clone::clone(&(*__self_0_1)),
                                pattern: ::std::clone::Clone::clone(&(*__self_0_2)),
                            },
                        }
                    }
                }
                fn default_background_color() -> Function<Color> {
                    return Function::Raw(Color(
                        ::css_color_parser::Color::from_str("#00000").unwrap(),
                    ));
                }
                fn default_backround_opacity() -> Function<f32> {
                    return Function::Raw(1.0);
                }
                impl Default for BackgroundPaint {
                    fn default() -> Self {
                        BackgroundPaint {
                            color: default_background_color(),
                            opacity: default_backround_opacity(),
                            pattern: None,
                        }
                    }
                }
            }
            pub mod fill {
                use super::super::color::Color;
                use super::super::function::Function;
                use super::{BaseLayout, LayerCommon, Visibility};
                use prelude::*;
                pub struct FillPaint {
                    #[serde(rename = "fill-antialias")]
                    pub antialias: Option<Function<bool>>,
                    #[serde(rename = "fill-opacity")]
                    pub opacity: Option<Function<f32>>,
                    #[serde(rename = "fill-color")]
                    pub color: Option<Function<Color>>,
                    #[serde(rename = "fill-outline-color")]
                    pub outline_color: Option<Function<Color>>,
                    #[serde(rename = "fill-translate")]
                    pub translate: Option<Function<[f32; 2]>>,
                    #[serde(rename = "fill-translate-anchor")]
                    pub translate_anchor: Option<String>,
                    #[serde(rename = "fill-pattern")]
                    pub pattern: Option<Function<String>>,
                }
                #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
                const _IMPL_DESERIALIZE_FOR_FillPaint: () = {
                    extern crate serde as _serde;
                    #[allow(unused_macros)]
                    macro_rules! try(( $ __expr : expr ) => {
                                         match $ __expr {
                                         _serde :: export :: Ok ( __val ) =>
                                         __val , _serde :: export :: Err (
                                         __err ) => {
                                         return _serde :: export :: Err (
                                         __err ) ; } } });
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for FillPaint {
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::export::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            #[allow(non_camel_case_types)]
                            enum __Field {
                                __field0,
                                __field1,
                                __field2,
                                __field3,
                                __field4,
                                __field5,
                                __field6,
                                __ignore,
                            }
                            struct __FieldVisitor;
                            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                type Value = __Field;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::export::Formatter,
                                ) -> _serde::export::fmt::Result {
                                    _serde::export::Formatter::write_str(
                                        __formatter,
                                        "field identifier",
                                    )
                                }
                                fn visit_u64<__E>(
                                    self,
                                    __value: u64,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        0u64 => _serde::export::Ok(__Field::__field0),
                                        1u64 => _serde::export::Ok(__Field::__field1),
                                        2u64 => _serde::export::Ok(__Field::__field2),
                                        3u64 => _serde::export::Ok(__Field::__field3),
                                        4u64 => _serde::export::Ok(__Field::__field4),
                                        5u64 => _serde::export::Ok(__Field::__field5),
                                        6u64 => _serde::export::Ok(__Field::__field6),
                                        _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                            _serde::de::Unexpected::Unsigned(__value),
                                            &"field index 0 <= i < 7",
                                        )),
                                    }
                                }
                                fn visit_str<__E>(
                                    self,
                                    __value: &str,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        "fill-antialias" => _serde::export::Ok(__Field::__field0),
                                        "fill-opacity" => _serde::export::Ok(__Field::__field1),
                                        "fill-color" => _serde::export::Ok(__Field::__field2),
                                        "fill-outline-color" => {
                                            _serde::export::Ok(__Field::__field3)
                                        }
                                        "fill-translate" => _serde::export::Ok(__Field::__field4),
                                        "fill-translate-anchor" => {
                                            _serde::export::Ok(__Field::__field5)
                                        }
                                        "fill-pattern" => _serde::export::Ok(__Field::__field6),
                                        _ => _serde::export::Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(
                                    self,
                                    __value: &[u8],
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        b"fill-antialias" => _serde::export::Ok(__Field::__field0),
                                        b"fill-opacity" => _serde::export::Ok(__Field::__field1),
                                        b"fill-color" => _serde::export::Ok(__Field::__field2),
                                        b"fill-outline-color" => {
                                            _serde::export::Ok(__Field::__field3)
                                        }
                                        b"fill-translate" => _serde::export::Ok(__Field::__field4),
                                        b"fill-translate-anchor" => {
                                            _serde::export::Ok(__Field::__field5)
                                        }
                                        b"fill-pattern" => _serde::export::Ok(__Field::__field6),
                                        _ => _serde::export::Ok(__Field::__ignore),
                                    }
                                }
                            }
                            impl<'de> _serde::Deserialize<'de> for __Field {
                                #[inline]
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::export::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::Deserializer::deserialize_identifier(
                                        __deserializer,
                                        __FieldVisitor,
                                    )
                                }
                            }
                            struct __Visitor<'de> {
                                marker: _serde::export::PhantomData<FillPaint>,
                                lifetime: _serde::export::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                type Value = FillPaint;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::export::Formatter,
                                ) -> _serde::export::fmt::Result {
                                    _serde::export::Formatter::write_str(
                                        __formatter,
                                        "struct FillPaint",
                                    )
                                }
                                #[inline]
                                fn visit_seq<__A>(
                                    self,
                                    mut __seq: __A,
                                ) -> _serde::export::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::SeqAccess<'de>,
                                {
                                    let __field0 = match match _serde::de::SeqAccess::next_element::<
                                        Option<Function<bool>>,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &"struct FillPaint with 7 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field1 = match match _serde::de::SeqAccess::next_element::<
                                        Option<Function<f32>>,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    1usize,
                                                    &"struct FillPaint with 7 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field2 = match match _serde::de::SeqAccess::next_element::<
                                        Option<Function<Color>>,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    2usize,
                                                    &"struct FillPaint with 7 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field3 = match match _serde::de::SeqAccess::next_element::<
                                        Option<Function<Color>>,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    3usize,
                                                    &"struct FillPaint with 7 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field4 = match match _serde::de::SeqAccess::next_element::<
                                        Option<Function<[f32; 2]>>,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    4usize,
                                                    &"struct FillPaint with 7 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field5 = match match _serde::de::SeqAccess::next_element::<
                                        Option<String>,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    5usize,
                                                    &"struct FillPaint with 7 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field6 = match match _serde::de::SeqAccess::next_element::<
                                        Option<Function<String>>,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    6usize,
                                                    &"struct FillPaint with 7 elements",
                                                ),
                                            );
                                        }
                                    };
                                    _serde::export::Ok(FillPaint {
                                        antialias: __field0,
                                        opacity: __field1,
                                        color: __field2,
                                        outline_color: __field3,
                                        translate: __field4,
                                        translate_anchor: __field5,
                                        pattern: __field6,
                                    })
                                }
                                #[inline]
                                fn visit_map<__A>(
                                    self,
                                    mut __map: __A,
                                ) -> _serde::export::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::MapAccess<'de>,
                                {
                                    let mut __field0:
                                                _serde::export::Option<Option<Function<bool>>> =
                                            _serde::export::None;
                                    let mut __field1:
                                                _serde::export::Option<Option<Function<f32>>> =
                                            _serde::export::None;
                                    let mut __field2:
                                                _serde::export::Option<Option<Function<Color>>> =
                                            _serde::export::None;
                                    let mut __field3:
                                                _serde::export::Option<Option<Function<Color>>> =
                                            _serde::export::None;
                                    let mut __field4:
                                                _serde::export::Option<Option<Function<[f32; 2]>>> =
                                            _serde::export::None;
                                    let mut __field5:
                                                _serde::export::Option<Option<String>> =
                                            _serde::export::None;
                                    let mut __field6:
                                                _serde::export::Option<Option<Function<String>>> =
                                            _serde::export::None;
                                    while let _serde::export::Some(__key) =
                                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        } {
                                        match __key {
                                            __Field::__field0 => {
                                                if _serde::export::Option::is_some(&__field0) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("fill-antialias"));
                                                }
                                                __field0 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Option<Function<bool>>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field1 => {
                                                if _serde::export::Option::is_some(&__field1) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("fill-opacity"));
                                                }
                                                __field1 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Option<Function<f32>>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field2 => {
                                                if _serde::export::Option::is_some(&__field2) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("fill-color"));
                                                }
                                                __field2 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Option<Function<Color>>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field3 => {
                                                if _serde::export::Option::is_some(&__field3) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("fill-outline-color"));
                                                }
                                                __field3 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Option<Function<Color>>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field4 => {
                                                if _serde::export::Option::is_some(&__field4) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("fill-translate"));
                                                }
                                                __field4 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Option<Function<[f32; 2]>>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field5 => {
                                                if _serde::export::Option::is_some(&__field5) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("fill-translate-anchor"));
                                                }
                                                __field5 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Option<String>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field6 => {
                                                if _serde::export::Option::is_some(&__field6) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("fill-pattern"));
                                                }
                                                __field6 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Option<Function<String>>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            _ => {
                                                let _ = match _serde::de::MapAccess::next_value::<
                                                    _serde::de::IgnoredAny,
                                                >(
                                                    &mut __map
                                                ) {
                                                    _serde::export::Ok(__val) => __val,
                                                    _serde::export::Err(__err) => {
                                                        return _serde::export::Err(__err);
                                                    }
                                                };
                                            }
                                        }
                                    }
                                    let __field0 = match __field0 {
                                        _serde::export::Some(__field0) => __field0,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field(
                                                "fill-antialias",
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    let __field1 = match __field1 {
                                        _serde::export::Some(__field1) => __field1,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field("fill-opacity")
                                            {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    let __field2 = match __field2 {
                                        _serde::export::Some(__field2) => __field2,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field("fill-color") {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    let __field3 = match __field3 {
                                        _serde::export::Some(__field3) => __field3,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field(
                                                "fill-outline-color",
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    let __field4 = match __field4 {
                                        _serde::export::Some(__field4) => __field4,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field(
                                                "fill-translate",
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    let __field5 = match __field5 {
                                        _serde::export::Some(__field5) => __field5,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field(
                                                "fill-translate-anchor",
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    let __field6 = match __field6 {
                                        _serde::export::Some(__field6) => __field6,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field("fill-pattern")
                                            {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    _serde::export::Ok(FillPaint {
                                        antialias: __field0,
                                        opacity: __field1,
                                        color: __field2,
                                        outline_color: __field3,
                                        translate: __field4,
                                        translate_anchor: __field5,
                                        pattern: __field6,
                                    })
                                }
                            }
                            const FIELDS: &'static [&'static str] = &[
                                "fill-antialias",
                                "fill-opacity",
                                "fill-color",
                                "fill-outline-color",
                                "fill-translate",
                                "fill-translate-anchor",
                                "fill-pattern",
                            ];
                            _serde::Deserializer::deserialize_struct(
                                __deserializer,
                                "FillPaint",
                                FIELDS,
                                __Visitor {
                                    marker: _serde::export::PhantomData::<FillPaint>,
                                    lifetime: _serde::export::PhantomData,
                                },
                            )
                        }
                    }
                };
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl ::std::fmt::Debug for FillPaint {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        match *self {
                            FillPaint {
                                antialias: ref __self_0_0,
                                opacity: ref __self_0_1,
                                color: ref __self_0_2,
                                outline_color: ref __self_0_3,
                                translate: ref __self_0_4,
                                translate_anchor: ref __self_0_5,
                                pattern: ref __self_0_6,
                            } => {
                                let mut debug_trait_builder = f.debug_struct("FillPaint");
                                let _ = debug_trait_builder.field("antialias", &&(*__self_0_0));
                                let _ = debug_trait_builder.field("opacity", &&(*__self_0_1));
                                let _ = debug_trait_builder.field("color", &&(*__self_0_2));
                                let _ = debug_trait_builder.field("outline_color", &&(*__self_0_3));
                                let _ = debug_trait_builder.field("translate", &&(*__self_0_4));
                                let _ =
                                    debug_trait_builder.field("translate_anchor", &&(*__self_0_5));
                                let _ = debug_trait_builder.field("pattern", &&(*__self_0_6));
                                debug_trait_builder.finish()
                            }
                        }
                    }
                }
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl ::std::clone::Clone for FillPaint {
                    #[inline]
                    fn clone(&self) -> FillPaint {
                        match *self {
                            FillPaint {
                                antialias: ref __self_0_0,
                                opacity: ref __self_0_1,
                                color: ref __self_0_2,
                                outline_color: ref __self_0_3,
                                translate: ref __self_0_4,
                                translate_anchor: ref __self_0_5,
                                pattern: ref __self_0_6,
                            } => FillPaint {
                                antialias: ::std::clone::Clone::clone(&(*__self_0_0)),
                                opacity: ::std::clone::Clone::clone(&(*__self_0_1)),
                                color: ::std::clone::Clone::clone(&(*__self_0_2)),
                                outline_color: ::std::clone::Clone::clone(&(*__self_0_3)),
                                translate: ::std::clone::Clone::clone(&(*__self_0_4)),
                                translate_anchor: ::std::clone::Clone::clone(&(*__self_0_5)),
                                pattern: ::std::clone::Clone::clone(&(*__self_0_6)),
                            },
                        }
                    }
                }
                pub struct FillLayer {
                    #[serde(flatten)]
                    pub common: LayerCommon,
                    #[serde(default = "BaseLayout::default")]
                    pub layout: BaseLayout,
                    pub paint: Option<FillPaint>,
                }
                #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
                const _IMPL_DESERIALIZE_FOR_FillLayer: () = {
                    extern crate serde as _serde;
                    #[allow(unused_macros)]
                    macro_rules! try(( $ __expr : expr ) => {
                                         match $ __expr {
                                         _serde :: export :: Ok ( __val ) =>
                                         __val , _serde :: export :: Err (
                                         __err ) => {
                                         return _serde :: export :: Err (
                                         __err ) ; } } });
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for FillLayer {
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::export::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            #[allow(non_camel_case_types)]
                            enum __Field<'de> {
                                __field1,
                                __field2,
                                __other(_serde::private::de::Content<'de>),
                            }
                            struct __FieldVisitor;
                            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                type Value = __Field<'de>;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::export::Formatter,
                                ) -> _serde::export::fmt::Result {
                                    _serde::export::Formatter::write_str(
                                        __formatter,
                                        "field identifier",
                                    )
                                }
                                fn visit_bool<__E>(
                                    self,
                                    __value: bool,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::Bool(__value),
                                    ))
                                }
                                fn visit_i8<__E>(
                                    self,
                                    __value: i8,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::I8(__value),
                                    ))
                                }
                                fn visit_i16<__E>(
                                    self,
                                    __value: i16,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::I16(__value),
                                    ))
                                }
                                fn visit_i32<__E>(
                                    self,
                                    __value: i32,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::I32(__value),
                                    ))
                                }
                                fn visit_i64<__E>(
                                    self,
                                    __value: i64,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::I64(__value),
                                    ))
                                }
                                fn visit_u8<__E>(
                                    self,
                                    __value: u8,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::U8(__value),
                                    ))
                                }
                                fn visit_u16<__E>(
                                    self,
                                    __value: u16,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::U16(__value),
                                    ))
                                }
                                fn visit_u32<__E>(
                                    self,
                                    __value: u32,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::U32(__value),
                                    ))
                                }
                                fn visit_u64<__E>(
                                    self,
                                    __value: u64,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::U64(__value),
                                    ))
                                }
                                fn visit_f32<__E>(
                                    self,
                                    __value: f32,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::F32(__value),
                                    ))
                                }
                                fn visit_f64<__E>(
                                    self,
                                    __value: f64,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::F64(__value),
                                    ))
                                }
                                fn visit_char<__E>(
                                    self,
                                    __value: char,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::Char(__value),
                                    ))
                                }
                                fn visit_unit<__E>(self) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::Unit,
                                    ))
                                }
                                fn visit_borrowed_str<__E>(
                                    self,
                                    __value: &'de str,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        "layout" => _serde::export::Ok(__Field::__field1),
                                        "paint" => _serde::export::Ok(__Field::__field2),
                                        _ => {
                                            let __value =
                                                _serde::private::de::Content::Str(__value);
                                            _serde::export::Ok(__Field::__other(__value))
                                        }
                                    }
                                }
                                fn visit_borrowed_bytes<__E>(
                                    self,
                                    __value: &'de [u8],
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        b"layout" => _serde::export::Ok(__Field::__field1),
                                        b"paint" => _serde::export::Ok(__Field::__field2),
                                        _ => {
                                            let __value =
                                                _serde::private::de::Content::Bytes(__value);
                                            _serde::export::Ok(__Field::__other(__value))
                                        }
                                    }
                                }
                                fn visit_str<__E>(
                                    self,
                                    __value: &str,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        "layout" => _serde::export::Ok(__Field::__field1),
                                        "paint" => _serde::export::Ok(__Field::__field2),
                                        _ => {
                                            let __value = _serde::private::de::Content::String(
                                                __value.to_string(),
                                            );
                                            _serde::export::Ok(__Field::__other(__value))
                                        }
                                    }
                                }
                                fn visit_bytes<__E>(
                                    self,
                                    __value: &[u8],
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        b"layout" => _serde::export::Ok(__Field::__field1),
                                        b"paint" => _serde::export::Ok(__Field::__field2),
                                        _ => {
                                            let __value = _serde::private::de::Content::ByteBuf(
                                                __value.to_vec(),
                                            );
                                            _serde::export::Ok(__Field::__other(__value))
                                        }
                                    }
                                }
                            }
                            impl<'de> _serde::Deserialize<'de> for __Field<'de> {
                                #[inline]
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::export::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::Deserializer::deserialize_identifier(
                                        __deserializer,
                                        __FieldVisitor,
                                    )
                                }
                            }
                            struct __Visitor<'de> {
                                marker: _serde::export::PhantomData<FillLayer>,
                                lifetime: _serde::export::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                type Value = FillLayer;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::export::Formatter,
                                ) -> _serde::export::fmt::Result {
                                    _serde::export::Formatter::write_str(
                                        __formatter,
                                        "struct FillLayer",
                                    )
                                }
                                #[inline]
                                fn visit_map<__A>(
                                    self,
                                    mut __map: __A,
                                ) -> _serde::export::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::MapAccess<'de>,
                                {
                                    let mut __field1:
                                                _serde::export::Option<BaseLayout> =
                                            _serde::export::None;
                                    let mut __field2:
                                                _serde::export::Option<Option<FillPaint>> =
                                            _serde::export::None;
                                    let mut __collect = _serde::export::Vec::<
                                        _serde::export::Option<(
                                            _serde::private::de::Content,
                                            _serde::private::de::Content,
                                        )>,
                                    >::new(
                                    );
                                    while let _serde::export::Some(__key) =
                                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        } {
                                        match __key {
                                            __Field::__field1 => {
                                                if _serde::export::Option::is_some(&__field1) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("layout"));
                                                }
                                                __field1 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        BaseLayout,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field2 => {
                                                if _serde::export::Option::is_some(&__field2) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("paint"));
                                                }
                                                __field2 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Option<FillPaint>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__other(__name) => {
                                                __collect.push(_serde::export::Some((
                                                    __name,
                                                    match _serde::de::MapAccess::next_value(
                                                        &mut __map,
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                )));
                                            }
                                        }
                                    }
                                    let __field1 = match __field1 {
                                        _serde::export::Some(__field1) => __field1,
                                        _serde::export::None => BaseLayout::default(),
                                    };
                                    let __field2 = match __field2 {
                                        _serde::export::Some(__field2) => __field2,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field("paint") {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    let __field0: LayerCommon =
                                        match _serde::de::Deserialize::deserialize(
                                            _serde::private::de::FlatMapDeserializer(
                                                &mut __collect,
                                                _serde::export::PhantomData,
                                            ),
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        };
                                    _serde::export::Ok(FillLayer {
                                        common: __field0,
                                        layout: __field1,
                                        paint: __field2,
                                    })
                                }
                            }
                            _serde::Deserializer::deserialize_map(
                                __deserializer,
                                __Visitor {
                                    marker: _serde::export::PhantomData::<FillLayer>,
                                    lifetime: _serde::export::PhantomData,
                                },
                            )
                        }
                    }
                };
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl ::std::fmt::Debug for FillLayer {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        match *self {
                            FillLayer {
                                common: ref __self_0_0,
                                layout: ref __self_0_1,
                                paint: ref __self_0_2,
                            } => {
                                let mut debug_trait_builder = f.debug_struct("FillLayer");
                                let _ = debug_trait_builder.field("common", &&(*__self_0_0));
                                let _ = debug_trait_builder.field("layout", &&(*__self_0_1));
                                let _ = debug_trait_builder.field("paint", &&(*__self_0_2));
                                debug_trait_builder.finish()
                            }
                        }
                    }
                }
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl ::std::clone::Clone for FillLayer {
                    #[inline]
                    fn clone(&self) -> FillLayer {
                        match *self {
                            FillLayer {
                                common: ref __self_0_0,
                                layout: ref __self_0_1,
                                paint: ref __self_0_2,
                            } => FillLayer {
                                common: ::std::clone::Clone::clone(&(*__self_0_0)),
                                layout: ::std::clone::Clone::clone(&(*__self_0_1)),
                                paint: ::std::clone::Clone::clone(&(*__self_0_2)),
                            },
                        }
                    }
                }
            }
            pub mod line {
                use super::super::color::Color;
                use super::super::function::Function;
                use super::{BaseLayout, LayerCommon, Visibility};
                use prelude::*;
                pub struct LineLayer {
                    #[serde(flatten)]
                    pub common: LayerCommon,
                    pub layout: Option<LineLayout>,
                    pub paint: Option<LinePaint>,
                }
                #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
                const _IMPL_DESERIALIZE_FOR_LineLayer: () = {
                    extern crate serde as _serde;
                    #[allow(unused_macros)]
                    macro_rules! try(( $ __expr : expr ) => {
                                         match $ __expr {
                                         _serde :: export :: Ok ( __val ) =>
                                         __val , _serde :: export :: Err (
                                         __err ) => {
                                         return _serde :: export :: Err (
                                         __err ) ; } } });
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for LineLayer {
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::export::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            #[allow(non_camel_case_types)]
                            enum __Field<'de> {
                                __field1,
                                __field2,
                                __other(_serde::private::de::Content<'de>),
                            }
                            struct __FieldVisitor;
                            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                type Value = __Field<'de>;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::export::Formatter,
                                ) -> _serde::export::fmt::Result {
                                    _serde::export::Formatter::write_str(
                                        __formatter,
                                        "field identifier",
                                    )
                                }
                                fn visit_bool<__E>(
                                    self,
                                    __value: bool,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::Bool(__value),
                                    ))
                                }
                                fn visit_i8<__E>(
                                    self,
                                    __value: i8,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::I8(__value),
                                    ))
                                }
                                fn visit_i16<__E>(
                                    self,
                                    __value: i16,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::I16(__value),
                                    ))
                                }
                                fn visit_i32<__E>(
                                    self,
                                    __value: i32,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::I32(__value),
                                    ))
                                }
                                fn visit_i64<__E>(
                                    self,
                                    __value: i64,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::I64(__value),
                                    ))
                                }
                                fn visit_u8<__E>(
                                    self,
                                    __value: u8,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::U8(__value),
                                    ))
                                }
                                fn visit_u16<__E>(
                                    self,
                                    __value: u16,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::U16(__value),
                                    ))
                                }
                                fn visit_u32<__E>(
                                    self,
                                    __value: u32,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::U32(__value),
                                    ))
                                }
                                fn visit_u64<__E>(
                                    self,
                                    __value: u64,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::U64(__value),
                                    ))
                                }
                                fn visit_f32<__E>(
                                    self,
                                    __value: f32,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::F32(__value),
                                    ))
                                }
                                fn visit_f64<__E>(
                                    self,
                                    __value: f64,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::F64(__value),
                                    ))
                                }
                                fn visit_char<__E>(
                                    self,
                                    __value: char,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::Char(__value),
                                    ))
                                }
                                fn visit_unit<__E>(self) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::Unit,
                                    ))
                                }
                                fn visit_borrowed_str<__E>(
                                    self,
                                    __value: &'de str,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        "layout" => _serde::export::Ok(__Field::__field1),
                                        "paint" => _serde::export::Ok(__Field::__field2),
                                        _ => {
                                            let __value =
                                                _serde::private::de::Content::Str(__value);
                                            _serde::export::Ok(__Field::__other(__value))
                                        }
                                    }
                                }
                                fn visit_borrowed_bytes<__E>(
                                    self,
                                    __value: &'de [u8],
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        b"layout" => _serde::export::Ok(__Field::__field1),
                                        b"paint" => _serde::export::Ok(__Field::__field2),
                                        _ => {
                                            let __value =
                                                _serde::private::de::Content::Bytes(__value);
                                            _serde::export::Ok(__Field::__other(__value))
                                        }
                                    }
                                }
                                fn visit_str<__E>(
                                    self,
                                    __value: &str,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        "layout" => _serde::export::Ok(__Field::__field1),
                                        "paint" => _serde::export::Ok(__Field::__field2),
                                        _ => {
                                            let __value = _serde::private::de::Content::String(
                                                __value.to_string(),
                                            );
                                            _serde::export::Ok(__Field::__other(__value))
                                        }
                                    }
                                }
                                fn visit_bytes<__E>(
                                    self,
                                    __value: &[u8],
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        b"layout" => _serde::export::Ok(__Field::__field1),
                                        b"paint" => _serde::export::Ok(__Field::__field2),
                                        _ => {
                                            let __value = _serde::private::de::Content::ByteBuf(
                                                __value.to_vec(),
                                            );
                                            _serde::export::Ok(__Field::__other(__value))
                                        }
                                    }
                                }
                            }
                            impl<'de> _serde::Deserialize<'de> for __Field<'de> {
                                #[inline]
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::export::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::Deserializer::deserialize_identifier(
                                        __deserializer,
                                        __FieldVisitor,
                                    )
                                }
                            }
                            struct __Visitor<'de> {
                                marker: _serde::export::PhantomData<LineLayer>,
                                lifetime: _serde::export::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                type Value = LineLayer;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::export::Formatter,
                                ) -> _serde::export::fmt::Result {
                                    _serde::export::Formatter::write_str(
                                        __formatter,
                                        "struct LineLayer",
                                    )
                                }
                                #[inline]
                                fn visit_map<__A>(
                                    self,
                                    mut __map: __A,
                                ) -> _serde::export::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::MapAccess<'de>,
                                {
                                    let mut __field1:
                                                _serde::export::Option<Option<LineLayout>> =
                                            _serde::export::None;
                                    let mut __field2:
                                                _serde::export::Option<Option<LinePaint>> =
                                            _serde::export::None;
                                    let mut __collect = _serde::export::Vec::<
                                        _serde::export::Option<(
                                            _serde::private::de::Content,
                                            _serde::private::de::Content,
                                        )>,
                                    >::new(
                                    );
                                    while let _serde::export::Some(__key) =
                                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        } {
                                        match __key {
                                            __Field::__field1 => {
                                                if _serde::export::Option::is_some(&__field1) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("layout"));
                                                }
                                                __field1 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Option<LineLayout>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field2 => {
                                                if _serde::export::Option::is_some(&__field2) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("paint"));
                                                }
                                                __field2 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Option<LinePaint>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__other(__name) => {
                                                __collect.push(_serde::export::Some((
                                                    __name,
                                                    match _serde::de::MapAccess::next_value(
                                                        &mut __map,
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                )));
                                            }
                                        }
                                    }
                                    let __field1 = match __field1 {
                                        _serde::export::Some(__field1) => __field1,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field("layout") {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    let __field2 = match __field2 {
                                        _serde::export::Some(__field2) => __field2,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field("paint") {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    let __field0: LayerCommon =
                                        match _serde::de::Deserialize::deserialize(
                                            _serde::private::de::FlatMapDeserializer(
                                                &mut __collect,
                                                _serde::export::PhantomData,
                                            ),
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        };
                                    _serde::export::Ok(LineLayer {
                                        common: __field0,
                                        layout: __field1,
                                        paint: __field2,
                                    })
                                }
                            }
                            _serde::Deserializer::deserialize_map(
                                __deserializer,
                                __Visitor {
                                    marker: _serde::export::PhantomData::<LineLayer>,
                                    lifetime: _serde::export::PhantomData,
                                },
                            )
                        }
                    }
                };
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl ::std::fmt::Debug for LineLayer {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        match *self {
                            LineLayer {
                                common: ref __self_0_0,
                                layout: ref __self_0_1,
                                paint: ref __self_0_2,
                            } => {
                                let mut debug_trait_builder = f.debug_struct("LineLayer");
                                let _ = debug_trait_builder.field("common", &&(*__self_0_0));
                                let _ = debug_trait_builder.field("layout", &&(*__self_0_1));
                                let _ = debug_trait_builder.field("paint", &&(*__self_0_2));
                                debug_trait_builder.finish()
                            }
                        }
                    }
                }
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl ::std::clone::Clone for LineLayer {
                    #[inline]
                    fn clone(&self) -> LineLayer {
                        match *self {
                            LineLayer {
                                common: ref __self_0_0,
                                layout: ref __self_0_1,
                                paint: ref __self_0_2,
                            } => LineLayer {
                                common: ::std::clone::Clone::clone(&(*__self_0_0)),
                                layout: ::std::clone::Clone::clone(&(*__self_0_1)),
                                paint: ::std::clone::Clone::clone(&(*__self_0_2)),
                            },
                        }
                    }
                }
                pub struct LineLayout {
                    #[serde(rename = "line-cap")]
                    cap: Option<String>,
                    #[serde(rename = "line-join")]
                    join: Option<String>,
                    #[serde(rename = "line-miter-limit")]
                    miter_limit: Option<Function<f32>>,
                    #[serde(rename = "line-round-limit")]
                    round_limit: Option<Function<f32>>,
                    visibility: Option<Visibility>,
                }
                #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
                const _IMPL_DESERIALIZE_FOR_LineLayout: () = {
                    extern crate serde as _serde;
                    #[allow(unused_macros)]
                    macro_rules! try(( $ __expr : expr ) => {
                                         match $ __expr {
                                         _serde :: export :: Ok ( __val ) =>
                                         __val , _serde :: export :: Err (
                                         __err ) => {
                                         return _serde :: export :: Err (
                                         __err ) ; } } });
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for LineLayout {
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::export::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            #[allow(non_camel_case_types)]
                            enum __Field {
                                __field0,
                                __field1,
                                __field2,
                                __field3,
                                __field4,
                                __ignore,
                            }
                            struct __FieldVisitor;
                            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                type Value = __Field;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::export::Formatter,
                                ) -> _serde::export::fmt::Result {
                                    _serde::export::Formatter::write_str(
                                        __formatter,
                                        "field identifier",
                                    )
                                }
                                fn visit_u64<__E>(
                                    self,
                                    __value: u64,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        0u64 => _serde::export::Ok(__Field::__field0),
                                        1u64 => _serde::export::Ok(__Field::__field1),
                                        2u64 => _serde::export::Ok(__Field::__field2),
                                        3u64 => _serde::export::Ok(__Field::__field3),
                                        4u64 => _serde::export::Ok(__Field::__field4),
                                        _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                            _serde::de::Unexpected::Unsigned(__value),
                                            &"field index 0 <= i < 5",
                                        )),
                                    }
                                }
                                fn visit_str<__E>(
                                    self,
                                    __value: &str,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        "line-cap" => _serde::export::Ok(__Field::__field0),
                                        "line-join" => _serde::export::Ok(__Field::__field1),
                                        "line-miter-limit" => _serde::export::Ok(__Field::__field2),
                                        "line-round-limit" => _serde::export::Ok(__Field::__field3),
                                        "visibility" => _serde::export::Ok(__Field::__field4),
                                        _ => _serde::export::Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(
                                    self,
                                    __value: &[u8],
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        b"line-cap" => _serde::export::Ok(__Field::__field0),
                                        b"line-join" => _serde::export::Ok(__Field::__field1),
                                        b"line-miter-limit" => {
                                            _serde::export::Ok(__Field::__field2)
                                        }
                                        b"line-round-limit" => {
                                            _serde::export::Ok(__Field::__field3)
                                        }
                                        b"visibility" => _serde::export::Ok(__Field::__field4),
                                        _ => _serde::export::Ok(__Field::__ignore),
                                    }
                                }
                            }
                            impl<'de> _serde::Deserialize<'de> for __Field {
                                #[inline]
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::export::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::Deserializer::deserialize_identifier(
                                        __deserializer,
                                        __FieldVisitor,
                                    )
                                }
                            }
                            struct __Visitor<'de> {
                                marker: _serde::export::PhantomData<LineLayout>,
                                lifetime: _serde::export::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                type Value = LineLayout;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::export::Formatter,
                                ) -> _serde::export::fmt::Result {
                                    _serde::export::Formatter::write_str(
                                        __formatter,
                                        "struct LineLayout",
                                    )
                                }
                                #[inline]
                                fn visit_seq<__A>(
                                    self,
                                    mut __seq: __A,
                                ) -> _serde::export::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::SeqAccess<'de>,
                                {
                                    let __field0 = match match _serde::de::SeqAccess::next_element::<
                                        Option<String>,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &"struct LineLayout with 5 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field1 = match match _serde::de::SeqAccess::next_element::<
                                        Option<String>,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    1usize,
                                                    &"struct LineLayout with 5 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field2 = match match _serde::de::SeqAccess::next_element::<
                                        Option<Function<f32>>,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    2usize,
                                                    &"struct LineLayout with 5 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field3 = match match _serde::de::SeqAccess::next_element::<
                                        Option<Function<f32>>,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    3usize,
                                                    &"struct LineLayout with 5 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field4 = match match _serde::de::SeqAccess::next_element::<
                                        Option<Visibility>,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    4usize,
                                                    &"struct LineLayout with 5 elements",
                                                ),
                                            );
                                        }
                                    };
                                    _serde::export::Ok(LineLayout {
                                        cap: __field0,
                                        join: __field1,
                                        miter_limit: __field2,
                                        round_limit: __field3,
                                        visibility: __field4,
                                    })
                                }
                                #[inline]
                                fn visit_map<__A>(
                                    self,
                                    mut __map: __A,
                                ) -> _serde::export::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::MapAccess<'de>,
                                {
                                    let mut __field0:
                                                _serde::export::Option<Option<String>> =
                                            _serde::export::None;
                                    let mut __field1:
                                                _serde::export::Option<Option<String>> =
                                            _serde::export::None;
                                    let mut __field2:
                                                _serde::export::Option<Option<Function<f32>>> =
                                            _serde::export::None;
                                    let mut __field3:
                                                _serde::export::Option<Option<Function<f32>>> =
                                            _serde::export::None;
                                    let mut __field4:
                                                _serde::export::Option<Option<Visibility>> =
                                            _serde::export::None;
                                    while let _serde::export::Some(__key) =
                                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        } {
                                        match __key {
                                            __Field::__field0 => {
                                                if _serde::export::Option::is_some(&__field0) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("line-cap"));
                                                }
                                                __field0 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Option<String>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field1 => {
                                                if _serde::export::Option::is_some(&__field1) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("line-join"));
                                                }
                                                __field1 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Option<String>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field2 => {
                                                if _serde::export::Option::is_some(&__field2) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("line-miter-limit"));
                                                }
                                                __field2 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Option<Function<f32>>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field3 => {
                                                if _serde::export::Option::is_some(&__field3) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("line-round-limit"));
                                                }
                                                __field3 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Option<Function<f32>>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field4 => {
                                                if _serde::export::Option::is_some(&__field4) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("visibility"));
                                                }
                                                __field4 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Option<Visibility>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            _ => {
                                                let _ = match _serde::de::MapAccess::next_value::<
                                                    _serde::de::IgnoredAny,
                                                >(
                                                    &mut __map
                                                ) {
                                                    _serde::export::Ok(__val) => __val,
                                                    _serde::export::Err(__err) => {
                                                        return _serde::export::Err(__err);
                                                    }
                                                };
                                            }
                                        }
                                    }
                                    let __field0 = match __field0 {
                                        _serde::export::Some(__field0) => __field0,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field("line-cap") {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    let __field1 = match __field1 {
                                        _serde::export::Some(__field1) => __field1,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field("line-join") {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    let __field2 = match __field2 {
                                        _serde::export::Some(__field2) => __field2,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field(
                                                "line-miter-limit",
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    let __field3 = match __field3 {
                                        _serde::export::Some(__field3) => __field3,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field(
                                                "line-round-limit",
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    let __field4 = match __field4 {
                                        _serde::export::Some(__field4) => __field4,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field("visibility") {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    _serde::export::Ok(LineLayout {
                                        cap: __field0,
                                        join: __field1,
                                        miter_limit: __field2,
                                        round_limit: __field3,
                                        visibility: __field4,
                                    })
                                }
                            }
                            const FIELDS: &'static [&'static str] = &[
                                "line-cap",
                                "line-join",
                                "line-miter-limit",
                                "line-round-limit",
                                "visibility",
                            ];
                            _serde::Deserializer::deserialize_struct(
                                __deserializer,
                                "LineLayout",
                                FIELDS,
                                __Visitor {
                                    marker: _serde::export::PhantomData::<LineLayout>,
                                    lifetime: _serde::export::PhantomData,
                                },
                            )
                        }
                    }
                };
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl ::std::fmt::Debug for LineLayout {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        match *self {
                            LineLayout {
                                cap: ref __self_0_0,
                                join: ref __self_0_1,
                                miter_limit: ref __self_0_2,
                                round_limit: ref __self_0_3,
                                visibility: ref __self_0_4,
                            } => {
                                let mut debug_trait_builder = f.debug_struct("LineLayout");
                                let _ = debug_trait_builder.field("cap", &&(*__self_0_0));
                                let _ = debug_trait_builder.field("join", &&(*__self_0_1));
                                let _ = debug_trait_builder.field("miter_limit", &&(*__self_0_2));
                                let _ = debug_trait_builder.field("round_limit", &&(*__self_0_3));
                                let _ = debug_trait_builder.field("visibility", &&(*__self_0_4));
                                debug_trait_builder.finish()
                            }
                        }
                    }
                }
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl ::std::clone::Clone for LineLayout {
                    #[inline]
                    fn clone(&self) -> LineLayout {
                        match *self {
                            LineLayout {
                                cap: ref __self_0_0,
                                join: ref __self_0_1,
                                miter_limit: ref __self_0_2,
                                round_limit: ref __self_0_3,
                                visibility: ref __self_0_4,
                            } => LineLayout {
                                cap: ::std::clone::Clone::clone(&(*__self_0_0)),
                                join: ::std::clone::Clone::clone(&(*__self_0_1)),
                                miter_limit: ::std::clone::Clone::clone(&(*__self_0_2)),
                                round_limit: ::std::clone::Clone::clone(&(*__self_0_3)),
                                visibility: ::std::clone::Clone::clone(&(*__self_0_4)),
                            },
                        }
                    }
                }
                pub struct LinePaint {
                    #[serde(rename = "line-opacity")]
                    opacity: Option<Function<f32>>,
                    #[serde(rename = "line-color")]
                    color: Option<Function<Color>>,
                    #[serde(rename = "line-translate")]
                    translate: Option<Function<[f32; 2]>>,
                    #[serde(rename = "line-translate-anchor")]
                    translate_anchor: Option<String>,
                    #[serde(rename = "line-width")]
                    width: Option<Function<f32>>,
                    #[serde(rename = "line-gap_width")]
                    gap_width: Option<Function<f32>>,
                    #[serde(rename = "line-offset")]
                    offset: Option<Function<f32>>,
                    #[serde(rename = "line-blur")]
                    blur: Option<Function<f32>>,
                    #[serde(rename = "line-dasharray")]
                    dash_array: Option<Function<Vec<f32>>>,
                    #[serde(rename = "line-pattern")]
                    pattern: Option<Function<String>>,
                }
                #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
                const _IMPL_DESERIALIZE_FOR_LinePaint: () = {
                    extern crate serde as _serde;
                    #[allow(unused_macros)]
                    macro_rules! try(( $ __expr : expr ) => {
                                         match $ __expr {
                                         _serde :: export :: Ok ( __val ) =>
                                         __val , _serde :: export :: Err (
                                         __err ) => {
                                         return _serde :: export :: Err (
                                         __err ) ; } } });
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for LinePaint {
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::export::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            #[allow(non_camel_case_types)]
                            enum __Field {
                                __field0,
                                __field1,
                                __field2,
                                __field3,
                                __field4,
                                __field5,
                                __field6,
                                __field7,
                                __field8,
                                __field9,
                                __ignore,
                            }
                            struct __FieldVisitor;
                            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                type Value = __Field;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::export::Formatter,
                                ) -> _serde::export::fmt::Result {
                                    _serde::export::Formatter::write_str(
                                        __formatter,
                                        "field identifier",
                                    )
                                }
                                fn visit_u64<__E>(
                                    self,
                                    __value: u64,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        0u64 => _serde::export::Ok(__Field::__field0),
                                        1u64 => _serde::export::Ok(__Field::__field1),
                                        2u64 => _serde::export::Ok(__Field::__field2),
                                        3u64 => _serde::export::Ok(__Field::__field3),
                                        4u64 => _serde::export::Ok(__Field::__field4),
                                        5u64 => _serde::export::Ok(__Field::__field5),
                                        6u64 => _serde::export::Ok(__Field::__field6),
                                        7u64 => _serde::export::Ok(__Field::__field7),
                                        8u64 => _serde::export::Ok(__Field::__field8),
                                        9u64 => _serde::export::Ok(__Field::__field9),
                                        _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                            _serde::de::Unexpected::Unsigned(__value),
                                            &"field index 0 <= i < 10",
                                        )),
                                    }
                                }
                                fn visit_str<__E>(
                                    self,
                                    __value: &str,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        "line-opacity" => _serde::export::Ok(__Field::__field0),
                                        "line-color" => _serde::export::Ok(__Field::__field1),
                                        "line-translate" => _serde::export::Ok(__Field::__field2),
                                        "line-translate-anchor" => {
                                            _serde::export::Ok(__Field::__field3)
                                        }
                                        "line-width" => _serde::export::Ok(__Field::__field4),
                                        "line-gap_width" => _serde::export::Ok(__Field::__field5),
                                        "line-offset" => _serde::export::Ok(__Field::__field6),
                                        "line-blur" => _serde::export::Ok(__Field::__field7),
                                        "line-dasharray" => _serde::export::Ok(__Field::__field8),
                                        "line-pattern" => _serde::export::Ok(__Field::__field9),
                                        _ => _serde::export::Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(
                                    self,
                                    __value: &[u8],
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        b"line-opacity" => _serde::export::Ok(__Field::__field0),
                                        b"line-color" => _serde::export::Ok(__Field::__field1),
                                        b"line-translate" => _serde::export::Ok(__Field::__field2),
                                        b"line-translate-anchor" => {
                                            _serde::export::Ok(__Field::__field3)
                                        }
                                        b"line-width" => _serde::export::Ok(__Field::__field4),
                                        b"line-gap_width" => _serde::export::Ok(__Field::__field5),
                                        b"line-offset" => _serde::export::Ok(__Field::__field6),
                                        b"line-blur" => _serde::export::Ok(__Field::__field7),
                                        b"line-dasharray" => _serde::export::Ok(__Field::__field8),
                                        b"line-pattern" => _serde::export::Ok(__Field::__field9),
                                        _ => _serde::export::Ok(__Field::__ignore),
                                    }
                                }
                            }
                            impl<'de> _serde::Deserialize<'de> for __Field {
                                #[inline]
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::export::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::Deserializer::deserialize_identifier(
                                        __deserializer,
                                        __FieldVisitor,
                                    )
                                }
                            }
                            struct __Visitor<'de> {
                                marker: _serde::export::PhantomData<LinePaint>,
                                lifetime: _serde::export::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                type Value = LinePaint;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::export::Formatter,
                                ) -> _serde::export::fmt::Result {
                                    _serde::export::Formatter::write_str(
                                        __formatter,
                                        "struct LinePaint",
                                    )
                                }
                                #[inline]
                                fn visit_seq<__A>(
                                    self,
                                    mut __seq: __A,
                                ) -> _serde::export::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::SeqAccess<'de>,
                                {
                                    let __field0 = match match _serde::de::SeqAccess::next_element::<
                                        Option<Function<f32>>,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &"struct LinePaint with 10 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field1 = match match _serde::de::SeqAccess::next_element::<
                                        Option<Function<Color>>,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    1usize,
                                                    &"struct LinePaint with 10 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field2 = match match _serde::de::SeqAccess::next_element::<
                                        Option<Function<[f32; 2]>>,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    2usize,
                                                    &"struct LinePaint with 10 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field3 = match match _serde::de::SeqAccess::next_element::<
                                        Option<String>,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    3usize,
                                                    &"struct LinePaint with 10 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field4 = match match _serde::de::SeqAccess::next_element::<
                                        Option<Function<f32>>,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    4usize,
                                                    &"struct LinePaint with 10 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field5 = match match _serde::de::SeqAccess::next_element::<
                                        Option<Function<f32>>,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    5usize,
                                                    &"struct LinePaint with 10 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field6 = match match _serde::de::SeqAccess::next_element::<
                                        Option<Function<f32>>,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    6usize,
                                                    &"struct LinePaint with 10 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field7 = match match _serde::de::SeqAccess::next_element::<
                                        Option<Function<f32>>,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    7usize,
                                                    &"struct LinePaint with 10 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field8 = match match _serde::de::SeqAccess::next_element::<
                                        Option<Function<Vec<f32>>>,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    8usize,
                                                    &"struct LinePaint with 10 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field9 = match match _serde::de::SeqAccess::next_element::<
                                        Option<Function<String>>,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    9usize,
                                                    &"struct LinePaint with 10 elements",
                                                ),
                                            );
                                        }
                                    };
                                    _serde::export::Ok(LinePaint {
                                        opacity: __field0,
                                        color: __field1,
                                        translate: __field2,
                                        translate_anchor: __field3,
                                        width: __field4,
                                        gap_width: __field5,
                                        offset: __field6,
                                        blur: __field7,
                                        dash_array: __field8,
                                        pattern: __field9,
                                    })
                                }
                                #[inline]
                                fn visit_map<__A>(
                                    self,
                                    mut __map: __A,
                                ) -> _serde::export::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::MapAccess<'de>,
                                {
                                    let mut __field0:
                                                _serde::export::Option<Option<Function<f32>>> =
                                            _serde::export::None;
                                    let mut __field1:
                                                _serde::export::Option<Option<Function<Color>>> =
                                            _serde::export::None;
                                    let mut __field2:
                                                _serde::export::Option<Option<Function<[f32; 2]>>> =
                                            _serde::export::None;
                                    let mut __field3:
                                                _serde::export::Option<Option<String>> =
                                            _serde::export::None;
                                    let mut __field4:
                                                _serde::export::Option<Option<Function<f32>>> =
                                            _serde::export::None;
                                    let mut __field5:
                                                _serde::export::Option<Option<Function<f32>>> =
                                            _serde::export::None;
                                    let mut __field6:
                                                _serde::export::Option<Option<Function<f32>>> =
                                            _serde::export::None;
                                    let mut __field7:
                                                _serde::export::Option<Option<Function<f32>>> =
                                            _serde::export::None;
                                    let mut __field8:
                                                _serde::export::Option<Option<Function<Vec<f32>>>> =
                                            _serde::export::None;
                                    let mut __field9:
                                                _serde::export::Option<Option<Function<String>>> =
                                            _serde::export::None;
                                    while let _serde::export::Some(__key) =
                                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        } {
                                        match __key {
                                            __Field::__field0 => {
                                                if _serde::export::Option::is_some(&__field0) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("line-opacity"));
                                                }
                                                __field0 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Option<Function<f32>>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field1 => {
                                                if _serde::export::Option::is_some(&__field1) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("line-color"));
                                                }
                                                __field1 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Option<Function<Color>>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field2 => {
                                                if _serde::export::Option::is_some(&__field2) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("line-translate"));
                                                }
                                                __field2 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Option<Function<[f32; 2]>>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field3 => {
                                                if _serde::export::Option::is_some(&__field3) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("line-translate-anchor"));
                                                }
                                                __field3 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Option<String>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field4 => {
                                                if _serde::export::Option::is_some(&__field4) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("line-width"));
                                                }
                                                __field4 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Option<Function<f32>>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field5 => {
                                                if _serde::export::Option::is_some(&__field5) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("line-gap_width"));
                                                }
                                                __field5 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Option<Function<f32>>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field6 => {
                                                if _serde::export::Option::is_some(&__field6) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("line-offset"));
                                                }
                                                __field6 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Option<Function<f32>>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field7 => {
                                                if _serde::export::Option::is_some(&__field7) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("line-blur"));
                                                }
                                                __field7 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Option<Function<f32>>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field8 => {
                                                if _serde::export::Option::is_some(&__field8) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("line-dasharray"));
                                                }
                                                __field8 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Option<Function<Vec<f32>>>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field9 => {
                                                if _serde::export::Option::is_some(&__field9) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("line-pattern"));
                                                }
                                                __field9 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Option<Function<String>>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            _ => {
                                                let _ = match _serde::de::MapAccess::next_value::<
                                                    _serde::de::IgnoredAny,
                                                >(
                                                    &mut __map
                                                ) {
                                                    _serde::export::Ok(__val) => __val,
                                                    _serde::export::Err(__err) => {
                                                        return _serde::export::Err(__err);
                                                    }
                                                };
                                            }
                                        }
                                    }
                                    let __field0 = match __field0 {
                                        _serde::export::Some(__field0) => __field0,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field("line-opacity")
                                            {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    let __field1 = match __field1 {
                                        _serde::export::Some(__field1) => __field1,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field("line-color") {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    let __field2 = match __field2 {
                                        _serde::export::Some(__field2) => __field2,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field(
                                                "line-translate",
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    let __field3 = match __field3 {
                                        _serde::export::Some(__field3) => __field3,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field(
                                                "line-translate-anchor",
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    let __field4 = match __field4 {
                                        _serde::export::Some(__field4) => __field4,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field("line-width") {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    let __field5 = match __field5 {
                                        _serde::export::Some(__field5) => __field5,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field(
                                                "line-gap_width",
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    let __field6 = match __field6 {
                                        _serde::export::Some(__field6) => __field6,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field("line-offset")
                                            {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    let __field7 = match __field7 {
                                        _serde::export::Some(__field7) => __field7,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field("line-blur") {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    let __field8 = match __field8 {
                                        _serde::export::Some(__field8) => __field8,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field(
                                                "line-dasharray",
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    let __field9 = match __field9 {
                                        _serde::export::Some(__field9) => __field9,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field("line-pattern")
                                            {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    _serde::export::Ok(LinePaint {
                                        opacity: __field0,
                                        color: __field1,
                                        translate: __field2,
                                        translate_anchor: __field3,
                                        width: __field4,
                                        gap_width: __field5,
                                        offset: __field6,
                                        blur: __field7,
                                        dash_array: __field8,
                                        pattern: __field9,
                                    })
                                }
                            }
                            const FIELDS: &'static [&'static str] = &[
                                "line-opacity",
                                "line-color",
                                "line-translate",
                                "line-translate-anchor",
                                "line-width",
                                "line-gap_width",
                                "line-offset",
                                "line-blur",
                                "line-dasharray",
                                "line-pattern",
                            ];
                            _serde::Deserializer::deserialize_struct(
                                __deserializer,
                                "LinePaint",
                                FIELDS,
                                __Visitor {
                                    marker: _serde::export::PhantomData::<LinePaint>,
                                    lifetime: _serde::export::PhantomData,
                                },
                            )
                        }
                    }
                };
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl ::std::fmt::Debug for LinePaint {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        match *self {
                            LinePaint {
                                opacity: ref __self_0_0,
                                color: ref __self_0_1,
                                translate: ref __self_0_2,
                                translate_anchor: ref __self_0_3,
                                width: ref __self_0_4,
                                gap_width: ref __self_0_5,
                                offset: ref __self_0_6,
                                blur: ref __self_0_7,
                                dash_array: ref __self_0_8,
                                pattern: ref __self_0_9,
                            } => {
                                let mut debug_trait_builder = f.debug_struct("LinePaint");
                                let _ = debug_trait_builder.field("opacity", &&(*__self_0_0));
                                let _ = debug_trait_builder.field("color", &&(*__self_0_1));
                                let _ = debug_trait_builder.field("translate", &&(*__self_0_2));
                                let _ =
                                    debug_trait_builder.field("translate_anchor", &&(*__self_0_3));
                                let _ = debug_trait_builder.field("width", &&(*__self_0_4));
                                let _ = debug_trait_builder.field("gap_width", &&(*__self_0_5));
                                let _ = debug_trait_builder.field("offset", &&(*__self_0_6));
                                let _ = debug_trait_builder.field("blur", &&(*__self_0_7));
                                let _ = debug_trait_builder.field("dash_array", &&(*__self_0_8));
                                let _ = debug_trait_builder.field("pattern", &&(*__self_0_9));
                                debug_trait_builder.finish()
                            }
                        }
                    }
                }
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl ::std::clone::Clone for LinePaint {
                    #[inline]
                    fn clone(&self) -> LinePaint {
                        match *self {
                            LinePaint {
                                opacity: ref __self_0_0,
                                color: ref __self_0_1,
                                translate: ref __self_0_2,
                                translate_anchor: ref __self_0_3,
                                width: ref __self_0_4,
                                gap_width: ref __self_0_5,
                                offset: ref __self_0_6,
                                blur: ref __self_0_7,
                                dash_array: ref __self_0_8,
                                pattern: ref __self_0_9,
                            } => LinePaint {
                                opacity: ::std::clone::Clone::clone(&(*__self_0_0)),
                                color: ::std::clone::Clone::clone(&(*__self_0_1)),
                                translate: ::std::clone::Clone::clone(&(*__self_0_2)),
                                translate_anchor: ::std::clone::Clone::clone(&(*__self_0_3)),
                                width: ::std::clone::Clone::clone(&(*__self_0_4)),
                                gap_width: ::std::clone::Clone::clone(&(*__self_0_5)),
                                offset: ::std::clone::Clone::clone(&(*__self_0_6)),
                                blur: ::std::clone::Clone::clone(&(*__self_0_7)),
                                dash_array: ::std::clone::Clone::clone(&(*__self_0_8)),
                                pattern: ::std::clone::Clone::clone(&(*__self_0_9)),
                            },
                        }
                    }
                }
            }
            pub mod raster {
                use super::super::color::Color;
                use super::super::function::Function;
                use super::{BaseLayout, LayerCommon, Visibility};
                use prelude::*;
                pub struct RasterLayer {
                    #[serde(flatten)]
                    pub common: LayerCommon,
                    #[serde(default = "BaseLayout::default")]
                    pub layout: BaseLayout,
                    pub paint: Option<RasterPaint>,
                }
                #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
                const _IMPL_DESERIALIZE_FOR_RasterLayer: () = {
                    extern crate serde as _serde;
                    #[allow(unused_macros)]
                    macro_rules! try(( $ __expr : expr ) => {
                                         match $ __expr {
                                         _serde :: export :: Ok ( __val ) =>
                                         __val , _serde :: export :: Err (
                                         __err ) => {
                                         return _serde :: export :: Err (
                                         __err ) ; } } });
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for RasterLayer {
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::export::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            #[allow(non_camel_case_types)]
                            enum __Field<'de> {
                                __field1,
                                __field2,
                                __other(_serde::private::de::Content<'de>),
                            }
                            struct __FieldVisitor;
                            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                type Value = __Field<'de>;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::export::Formatter,
                                ) -> _serde::export::fmt::Result {
                                    _serde::export::Formatter::write_str(
                                        __formatter,
                                        "field identifier",
                                    )
                                }
                                fn visit_bool<__E>(
                                    self,
                                    __value: bool,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::Bool(__value),
                                    ))
                                }
                                fn visit_i8<__E>(
                                    self,
                                    __value: i8,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::I8(__value),
                                    ))
                                }
                                fn visit_i16<__E>(
                                    self,
                                    __value: i16,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::I16(__value),
                                    ))
                                }
                                fn visit_i32<__E>(
                                    self,
                                    __value: i32,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::I32(__value),
                                    ))
                                }
                                fn visit_i64<__E>(
                                    self,
                                    __value: i64,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::I64(__value),
                                    ))
                                }
                                fn visit_u8<__E>(
                                    self,
                                    __value: u8,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::U8(__value),
                                    ))
                                }
                                fn visit_u16<__E>(
                                    self,
                                    __value: u16,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::U16(__value),
                                    ))
                                }
                                fn visit_u32<__E>(
                                    self,
                                    __value: u32,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::U32(__value),
                                    ))
                                }
                                fn visit_u64<__E>(
                                    self,
                                    __value: u64,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::U64(__value),
                                    ))
                                }
                                fn visit_f32<__E>(
                                    self,
                                    __value: f32,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::F32(__value),
                                    ))
                                }
                                fn visit_f64<__E>(
                                    self,
                                    __value: f64,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::F64(__value),
                                    ))
                                }
                                fn visit_char<__E>(
                                    self,
                                    __value: char,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::Char(__value),
                                    ))
                                }
                                fn visit_unit<__E>(self) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::Unit,
                                    ))
                                }
                                fn visit_borrowed_str<__E>(
                                    self,
                                    __value: &'de str,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        "layout" => _serde::export::Ok(__Field::__field1),
                                        "paint" => _serde::export::Ok(__Field::__field2),
                                        _ => {
                                            let __value =
                                                _serde::private::de::Content::Str(__value);
                                            _serde::export::Ok(__Field::__other(__value))
                                        }
                                    }
                                }
                                fn visit_borrowed_bytes<__E>(
                                    self,
                                    __value: &'de [u8],
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        b"layout" => _serde::export::Ok(__Field::__field1),
                                        b"paint" => _serde::export::Ok(__Field::__field2),
                                        _ => {
                                            let __value =
                                                _serde::private::de::Content::Bytes(__value);
                                            _serde::export::Ok(__Field::__other(__value))
                                        }
                                    }
                                }
                                fn visit_str<__E>(
                                    self,
                                    __value: &str,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        "layout" => _serde::export::Ok(__Field::__field1),
                                        "paint" => _serde::export::Ok(__Field::__field2),
                                        _ => {
                                            let __value = _serde::private::de::Content::String(
                                                __value.to_string(),
                                            );
                                            _serde::export::Ok(__Field::__other(__value))
                                        }
                                    }
                                }
                                fn visit_bytes<__E>(
                                    self,
                                    __value: &[u8],
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        b"layout" => _serde::export::Ok(__Field::__field1),
                                        b"paint" => _serde::export::Ok(__Field::__field2),
                                        _ => {
                                            let __value = _serde::private::de::Content::ByteBuf(
                                                __value.to_vec(),
                                            );
                                            _serde::export::Ok(__Field::__other(__value))
                                        }
                                    }
                                }
                            }
                            impl<'de> _serde::Deserialize<'de> for __Field<'de> {
                                #[inline]
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::export::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::Deserializer::deserialize_identifier(
                                        __deserializer,
                                        __FieldVisitor,
                                    )
                                }
                            }
                            struct __Visitor<'de> {
                                marker: _serde::export::PhantomData<RasterLayer>,
                                lifetime: _serde::export::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                type Value = RasterLayer;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::export::Formatter,
                                ) -> _serde::export::fmt::Result {
                                    _serde::export::Formatter::write_str(
                                        __formatter,
                                        "struct RasterLayer",
                                    )
                                }
                                #[inline]
                                fn visit_map<__A>(
                                    self,
                                    mut __map: __A,
                                ) -> _serde::export::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::MapAccess<'de>,
                                {
                                    let mut __field1:
                                                _serde::export::Option<BaseLayout> =
                                            _serde::export::None;
                                    let mut __field2:
                                                _serde::export::Option<Option<RasterPaint>> =
                                            _serde::export::None;
                                    let mut __collect = _serde::export::Vec::<
                                        _serde::export::Option<(
                                            _serde::private::de::Content,
                                            _serde::private::de::Content,
                                        )>,
                                    >::new(
                                    );
                                    while let _serde::export::Some(__key) =
                                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        } {
                                        match __key {
                                            __Field::__field1 => {
                                                if _serde::export::Option::is_some(&__field1) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("layout"));
                                                }
                                                __field1 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        BaseLayout,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field2 => {
                                                if _serde::export::Option::is_some(&__field2) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("paint"));
                                                }
                                                __field2 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Option<RasterPaint>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__other(__name) => {
                                                __collect.push(_serde::export::Some((
                                                    __name,
                                                    match _serde::de::MapAccess::next_value(
                                                        &mut __map,
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                )));
                                            }
                                        }
                                    }
                                    let __field1 = match __field1 {
                                        _serde::export::Some(__field1) => __field1,
                                        _serde::export::None => BaseLayout::default(),
                                    };
                                    let __field2 = match __field2 {
                                        _serde::export::Some(__field2) => __field2,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field("paint") {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    let __field0: LayerCommon =
                                        match _serde::de::Deserialize::deserialize(
                                            _serde::private::de::FlatMapDeserializer(
                                                &mut __collect,
                                                _serde::export::PhantomData,
                                            ),
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        };
                                    _serde::export::Ok(RasterLayer {
                                        common: __field0,
                                        layout: __field1,
                                        paint: __field2,
                                    })
                                }
                            }
                            _serde::Deserializer::deserialize_map(
                                __deserializer,
                                __Visitor {
                                    marker: _serde::export::PhantomData::<RasterLayer>,
                                    lifetime: _serde::export::PhantomData,
                                },
                            )
                        }
                    }
                };
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl ::std::fmt::Debug for RasterLayer {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        match *self {
                            RasterLayer {
                                common: ref __self_0_0,
                                layout: ref __self_0_1,
                                paint: ref __self_0_2,
                            } => {
                                let mut debug_trait_builder = f.debug_struct("RasterLayer");
                                let _ = debug_trait_builder.field("common", &&(*__self_0_0));
                                let _ = debug_trait_builder.field("layout", &&(*__self_0_1));
                                let _ = debug_trait_builder.field("paint", &&(*__self_0_2));
                                debug_trait_builder.finish()
                            }
                        }
                    }
                }
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl ::std::clone::Clone for RasterLayer {
                    #[inline]
                    fn clone(&self) -> RasterLayer {
                        match *self {
                            RasterLayer {
                                common: ref __self_0_0,
                                layout: ref __self_0_1,
                                paint: ref __self_0_2,
                            } => RasterLayer {
                                common: ::std::clone::Clone::clone(&(*__self_0_0)),
                                layout: ::std::clone::Clone::clone(&(*__self_0_1)),
                                paint: ::std::clone::Clone::clone(&(*__self_0_2)),
                            },
                        }
                    }
                }
                pub struct RasterPaint {
                    #[serde(rename = "raster-opacity")]
                    opacity: Option<Function<f32>>,
                    #[serde(rename = "raster-hue-rotate")]
                    hue_rotate: Option<Function<f32>>,
                    #[serde(rename = "raster-brightness-min")]
                    brightness_min: Option<Function<f32>>,
                    #[serde(rename = "raster-brightness-max")]
                    brightness_max: Option<Function<f32>>,
                    #[serde(rename = "raster-saturation")]
                    saturation: Option<Function<f32>>,
                    #[serde(rename = "raster-contrast")]
                    contrast: Option<Function<f32>>,
                    #[serde(rename = "raster-fade-duration")]
                    fade_duration: Option<Function<f32>>,
                }
                #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
                const _IMPL_DESERIALIZE_FOR_RasterPaint: () = {
                    extern crate serde as _serde;
                    #[allow(unused_macros)]
                    macro_rules! try(( $ __expr : expr ) => {
                                         match $ __expr {
                                         _serde :: export :: Ok ( __val ) =>
                                         __val , _serde :: export :: Err (
                                         __err ) => {
                                         return _serde :: export :: Err (
                                         __err ) ; } } });
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for RasterPaint {
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::export::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            #[allow(non_camel_case_types)]
                            enum __Field {
                                __field0,
                                __field1,
                                __field2,
                                __field3,
                                __field4,
                                __field5,
                                __field6,
                                __ignore,
                            }
                            struct __FieldVisitor;
                            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                type Value = __Field;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::export::Formatter,
                                ) -> _serde::export::fmt::Result {
                                    _serde::export::Formatter::write_str(
                                        __formatter,
                                        "field identifier",
                                    )
                                }
                                fn visit_u64<__E>(
                                    self,
                                    __value: u64,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        0u64 => _serde::export::Ok(__Field::__field0),
                                        1u64 => _serde::export::Ok(__Field::__field1),
                                        2u64 => _serde::export::Ok(__Field::__field2),
                                        3u64 => _serde::export::Ok(__Field::__field3),
                                        4u64 => _serde::export::Ok(__Field::__field4),
                                        5u64 => _serde::export::Ok(__Field::__field5),
                                        6u64 => _serde::export::Ok(__Field::__field6),
                                        _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                            _serde::de::Unexpected::Unsigned(__value),
                                            &"field index 0 <= i < 7",
                                        )),
                                    }
                                }
                                fn visit_str<__E>(
                                    self,
                                    __value: &str,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        "raster-opacity" => _serde::export::Ok(__Field::__field0),
                                        "raster-hue-rotate" => {
                                            _serde::export::Ok(__Field::__field1)
                                        }
                                        "raster-brightness-min" => {
                                            _serde::export::Ok(__Field::__field2)
                                        }
                                        "raster-brightness-max" => {
                                            _serde::export::Ok(__Field::__field3)
                                        }
                                        "raster-saturation" => {
                                            _serde::export::Ok(__Field::__field4)
                                        }
                                        "raster-contrast" => _serde::export::Ok(__Field::__field5),
                                        "raster-fade-duration" => {
                                            _serde::export::Ok(__Field::__field6)
                                        }
                                        _ => _serde::export::Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(
                                    self,
                                    __value: &[u8],
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        b"raster-opacity" => _serde::export::Ok(__Field::__field0),
                                        b"raster-hue-rotate" => {
                                            _serde::export::Ok(__Field::__field1)
                                        }
                                        b"raster-brightness-min" => {
                                            _serde::export::Ok(__Field::__field2)
                                        }
                                        b"raster-brightness-max" => {
                                            _serde::export::Ok(__Field::__field3)
                                        }
                                        b"raster-saturation" => {
                                            _serde::export::Ok(__Field::__field4)
                                        }
                                        b"raster-contrast" => _serde::export::Ok(__Field::__field5),
                                        b"raster-fade-duration" => {
                                            _serde::export::Ok(__Field::__field6)
                                        }
                                        _ => _serde::export::Ok(__Field::__ignore),
                                    }
                                }
                            }
                            impl<'de> _serde::Deserialize<'de> for __Field {
                                #[inline]
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::export::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::Deserializer::deserialize_identifier(
                                        __deserializer,
                                        __FieldVisitor,
                                    )
                                }
                            }
                            struct __Visitor<'de> {
                                marker: _serde::export::PhantomData<RasterPaint>,
                                lifetime: _serde::export::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                type Value = RasterPaint;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::export::Formatter,
                                ) -> _serde::export::fmt::Result {
                                    _serde::export::Formatter::write_str(
                                        __formatter,
                                        "struct RasterPaint",
                                    )
                                }
                                #[inline]
                                fn visit_seq<__A>(
                                    self,
                                    mut __seq: __A,
                                ) -> _serde::export::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::SeqAccess<'de>,
                                {
                                    let __field0 = match match _serde::de::SeqAccess::next_element::<
                                        Option<Function<f32>>,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &"struct RasterPaint with 7 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field1 = match match _serde::de::SeqAccess::next_element::<
                                        Option<Function<f32>>,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    1usize,
                                                    &"struct RasterPaint with 7 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field2 = match match _serde::de::SeqAccess::next_element::<
                                        Option<Function<f32>>,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    2usize,
                                                    &"struct RasterPaint with 7 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field3 = match match _serde::de::SeqAccess::next_element::<
                                        Option<Function<f32>>,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    3usize,
                                                    &"struct RasterPaint with 7 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field4 = match match _serde::de::SeqAccess::next_element::<
                                        Option<Function<f32>>,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    4usize,
                                                    &"struct RasterPaint with 7 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field5 = match match _serde::de::SeqAccess::next_element::<
                                        Option<Function<f32>>,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    5usize,
                                                    &"struct RasterPaint with 7 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field6 = match match _serde::de::SeqAccess::next_element::<
                                        Option<Function<f32>>,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    6usize,
                                                    &"struct RasterPaint with 7 elements",
                                                ),
                                            );
                                        }
                                    };
                                    _serde::export::Ok(RasterPaint {
                                        opacity: __field0,
                                        hue_rotate: __field1,
                                        brightness_min: __field2,
                                        brightness_max: __field3,
                                        saturation: __field4,
                                        contrast: __field5,
                                        fade_duration: __field6,
                                    })
                                }
                                #[inline]
                                fn visit_map<__A>(
                                    self,
                                    mut __map: __A,
                                ) -> _serde::export::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::MapAccess<'de>,
                                {
                                    let mut __field0:
                                                _serde::export::Option<Option<Function<f32>>> =
                                            _serde::export::None;
                                    let mut __field1:
                                                _serde::export::Option<Option<Function<f32>>> =
                                            _serde::export::None;
                                    let mut __field2:
                                                _serde::export::Option<Option<Function<f32>>> =
                                            _serde::export::None;
                                    let mut __field3:
                                                _serde::export::Option<Option<Function<f32>>> =
                                            _serde::export::None;
                                    let mut __field4:
                                                _serde::export::Option<Option<Function<f32>>> =
                                            _serde::export::None;
                                    let mut __field5:
                                                _serde::export::Option<Option<Function<f32>>> =
                                            _serde::export::None;
                                    let mut __field6:
                                                _serde::export::Option<Option<Function<f32>>> =
                                            _serde::export::None;
                                    while let _serde::export::Some(__key) =
                                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        } {
                                        match __key {
                                            __Field::__field0 => {
                                                if _serde::export::Option::is_some(&__field0) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("raster-opacity"));
                                                }
                                                __field0 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Option<Function<f32>>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field1 => {
                                                if _serde::export::Option::is_some(&__field1) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("raster-hue-rotate"));
                                                }
                                                __field1 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Option<Function<f32>>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field2 => {
                                                if _serde::export::Option::is_some(&__field2) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("raster-brightness-min"));
                                                }
                                                __field2 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Option<Function<f32>>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field3 => {
                                                if _serde::export::Option::is_some(&__field3) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("raster-brightness-max"));
                                                }
                                                __field3 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Option<Function<f32>>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field4 => {
                                                if _serde::export::Option::is_some(&__field4) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("raster-saturation"));
                                                }
                                                __field4 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Option<Function<f32>>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field5 => {
                                                if _serde::export::Option::is_some(&__field5) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("raster-contrast"));
                                                }
                                                __field5 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Option<Function<f32>>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field6 => {
                                                if _serde::export::Option::is_some(&__field6) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("raster-fade-duration"));
                                                }
                                                __field6 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Option<Function<f32>>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            _ => {
                                                let _ = match _serde::de::MapAccess::next_value::<
                                                    _serde::de::IgnoredAny,
                                                >(
                                                    &mut __map
                                                ) {
                                                    _serde::export::Ok(__val) => __val,
                                                    _serde::export::Err(__err) => {
                                                        return _serde::export::Err(__err);
                                                    }
                                                };
                                            }
                                        }
                                    }
                                    let __field0 = match __field0 {
                                        _serde::export::Some(__field0) => __field0,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field(
                                                "raster-opacity",
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    let __field1 = match __field1 {
                                        _serde::export::Some(__field1) => __field1,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field(
                                                "raster-hue-rotate",
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    let __field2 = match __field2 {
                                        _serde::export::Some(__field2) => __field2,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field(
                                                "raster-brightness-min",
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    let __field3 = match __field3 {
                                        _serde::export::Some(__field3) => __field3,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field(
                                                "raster-brightness-max",
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    let __field4 = match __field4 {
                                        _serde::export::Some(__field4) => __field4,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field(
                                                "raster-saturation",
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    let __field5 = match __field5 {
                                        _serde::export::Some(__field5) => __field5,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field(
                                                "raster-contrast",
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    let __field6 = match __field6 {
                                        _serde::export::Some(__field6) => __field6,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field(
                                                "raster-fade-duration",
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    _serde::export::Ok(RasterPaint {
                                        opacity: __field0,
                                        hue_rotate: __field1,
                                        brightness_min: __field2,
                                        brightness_max: __field3,
                                        saturation: __field4,
                                        contrast: __field5,
                                        fade_duration: __field6,
                                    })
                                }
                            }
                            const FIELDS: &'static [&'static str] = &[
                                "raster-opacity",
                                "raster-hue-rotate",
                                "raster-brightness-min",
                                "raster-brightness-max",
                                "raster-saturation",
                                "raster-contrast",
                                "raster-fade-duration",
                            ];
                            _serde::Deserializer::deserialize_struct(
                                __deserializer,
                                "RasterPaint",
                                FIELDS,
                                __Visitor {
                                    marker: _serde::export::PhantomData::<RasterPaint>,
                                    lifetime: _serde::export::PhantomData,
                                },
                            )
                        }
                    }
                };
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl ::std::fmt::Debug for RasterPaint {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        match *self {
                            RasterPaint {
                                opacity: ref __self_0_0,
                                hue_rotate: ref __self_0_1,
                                brightness_min: ref __self_0_2,
                                brightness_max: ref __self_0_3,
                                saturation: ref __self_0_4,
                                contrast: ref __self_0_5,
                                fade_duration: ref __self_0_6,
                            } => {
                                let mut debug_trait_builder = f.debug_struct("RasterPaint");
                                let _ = debug_trait_builder.field("opacity", &&(*__self_0_0));
                                let _ = debug_trait_builder.field("hue_rotate", &&(*__self_0_1));
                                let _ =
                                    debug_trait_builder.field("brightness_min", &&(*__self_0_2));
                                let _ =
                                    debug_trait_builder.field("brightness_max", &&(*__self_0_3));
                                let _ = debug_trait_builder.field("saturation", &&(*__self_0_4));
                                let _ = debug_trait_builder.field("contrast", &&(*__self_0_5));
                                let _ = debug_trait_builder.field("fade_duration", &&(*__self_0_6));
                                debug_trait_builder.finish()
                            }
                        }
                    }
                }
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl ::std::clone::Clone for RasterPaint {
                    #[inline]
                    fn clone(&self) -> RasterPaint {
                        match *self {
                            RasterPaint {
                                opacity: ref __self_0_0,
                                hue_rotate: ref __self_0_1,
                                brightness_min: ref __self_0_2,
                                brightness_max: ref __self_0_3,
                                saturation: ref __self_0_4,
                                contrast: ref __self_0_5,
                                fade_duration: ref __self_0_6,
                            } => RasterPaint {
                                opacity: ::std::clone::Clone::clone(&(*__self_0_0)),
                                hue_rotate: ::std::clone::Clone::clone(&(*__self_0_1)),
                                brightness_min: ::std::clone::Clone::clone(&(*__self_0_2)),
                                brightness_max: ::std::clone::Clone::clone(&(*__self_0_3)),
                                saturation: ::std::clone::Clone::clone(&(*__self_0_4)),
                                contrast: ::std::clone::Clone::clone(&(*__self_0_5)),
                                fade_duration: ::std::clone::Clone::clone(&(*__self_0_6)),
                            },
                        }
                    }
                }
            }
            pub mod symbol {
                use super::super::color::Color;
                use super::super::function::Function;
                use super::{BaseLayout, LayerCommon, Visibility};
                use prelude::*;
                pub struct SymbolLayer {
                    #[serde(flatten)]
                    pub common: LayerCommon,
                    pub layout: Option<SymbolLayout>,
                    pub paint: Option<SymbolPaint>,
                }
                #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
                const _IMPL_DESERIALIZE_FOR_SymbolLayer: () = {
                    extern crate serde as _serde;
                    #[allow(unused_macros)]
                    macro_rules! try(( $ __expr : expr ) => {
                                         match $ __expr {
                                         _serde :: export :: Ok ( __val ) =>
                                         __val , _serde :: export :: Err (
                                         __err ) => {
                                         return _serde :: export :: Err (
                                         __err ) ; } } });
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for SymbolLayer {
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::export::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            #[allow(non_camel_case_types)]
                            enum __Field<'de> {
                                __field1,
                                __field2,
                                __other(_serde::private::de::Content<'de>),
                            }
                            struct __FieldVisitor;
                            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                type Value = __Field<'de>;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::export::Formatter,
                                ) -> _serde::export::fmt::Result {
                                    _serde::export::Formatter::write_str(
                                        __formatter,
                                        "field identifier",
                                    )
                                }
                                fn visit_bool<__E>(
                                    self,
                                    __value: bool,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::Bool(__value),
                                    ))
                                }
                                fn visit_i8<__E>(
                                    self,
                                    __value: i8,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::I8(__value),
                                    ))
                                }
                                fn visit_i16<__E>(
                                    self,
                                    __value: i16,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::I16(__value),
                                    ))
                                }
                                fn visit_i32<__E>(
                                    self,
                                    __value: i32,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::I32(__value),
                                    ))
                                }
                                fn visit_i64<__E>(
                                    self,
                                    __value: i64,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::I64(__value),
                                    ))
                                }
                                fn visit_u8<__E>(
                                    self,
                                    __value: u8,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::U8(__value),
                                    ))
                                }
                                fn visit_u16<__E>(
                                    self,
                                    __value: u16,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::U16(__value),
                                    ))
                                }
                                fn visit_u32<__E>(
                                    self,
                                    __value: u32,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::U32(__value),
                                    ))
                                }
                                fn visit_u64<__E>(
                                    self,
                                    __value: u64,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::U64(__value),
                                    ))
                                }
                                fn visit_f32<__E>(
                                    self,
                                    __value: f32,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::F32(__value),
                                    ))
                                }
                                fn visit_f64<__E>(
                                    self,
                                    __value: f64,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::F64(__value),
                                    ))
                                }
                                fn visit_char<__E>(
                                    self,
                                    __value: char,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::Char(__value),
                                    ))
                                }
                                fn visit_unit<__E>(self) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    _serde::export::Ok(__Field::__other(
                                        _serde::private::de::Content::Unit,
                                    ))
                                }
                                fn visit_borrowed_str<__E>(
                                    self,
                                    __value: &'de str,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        "layout" => _serde::export::Ok(__Field::__field1),
                                        "paint" => _serde::export::Ok(__Field::__field2),
                                        _ => {
                                            let __value =
                                                _serde::private::de::Content::Str(__value);
                                            _serde::export::Ok(__Field::__other(__value))
                                        }
                                    }
                                }
                                fn visit_borrowed_bytes<__E>(
                                    self,
                                    __value: &'de [u8],
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        b"layout" => _serde::export::Ok(__Field::__field1),
                                        b"paint" => _serde::export::Ok(__Field::__field2),
                                        _ => {
                                            let __value =
                                                _serde::private::de::Content::Bytes(__value);
                                            _serde::export::Ok(__Field::__other(__value))
                                        }
                                    }
                                }
                                fn visit_str<__E>(
                                    self,
                                    __value: &str,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        "layout" => _serde::export::Ok(__Field::__field1),
                                        "paint" => _serde::export::Ok(__Field::__field2),
                                        _ => {
                                            let __value = _serde::private::de::Content::String(
                                                __value.to_string(),
                                            );
                                            _serde::export::Ok(__Field::__other(__value))
                                        }
                                    }
                                }
                                fn visit_bytes<__E>(
                                    self,
                                    __value: &[u8],
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        b"layout" => _serde::export::Ok(__Field::__field1),
                                        b"paint" => _serde::export::Ok(__Field::__field2),
                                        _ => {
                                            let __value = _serde::private::de::Content::ByteBuf(
                                                __value.to_vec(),
                                            );
                                            _serde::export::Ok(__Field::__other(__value))
                                        }
                                    }
                                }
                            }
                            impl<'de> _serde::Deserialize<'de> for __Field<'de> {
                                #[inline]
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::export::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::Deserializer::deserialize_identifier(
                                        __deserializer,
                                        __FieldVisitor,
                                    )
                                }
                            }
                            struct __Visitor<'de> {
                                marker: _serde::export::PhantomData<SymbolLayer>,
                                lifetime: _serde::export::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                type Value = SymbolLayer;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::export::Formatter,
                                ) -> _serde::export::fmt::Result {
                                    _serde::export::Formatter::write_str(
                                        __formatter,
                                        "struct SymbolLayer",
                                    )
                                }
                                #[inline]
                                fn visit_map<__A>(
                                    self,
                                    mut __map: __A,
                                ) -> _serde::export::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::MapAccess<'de>,
                                {
                                    let mut __field1:
                                                _serde::export::Option<Option<SymbolLayout>> =
                                            _serde::export::None;
                                    let mut __field2:
                                                _serde::export::Option<Option<SymbolPaint>> =
                                            _serde::export::None;
                                    let mut __collect = _serde::export::Vec::<
                                        _serde::export::Option<(
                                            _serde::private::de::Content,
                                            _serde::private::de::Content,
                                        )>,
                                    >::new(
                                    );
                                    while let _serde::export::Some(__key) =
                                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        } {
                                        match __key {
                                            __Field::__field1 => {
                                                if _serde::export::Option::is_some(&__field1) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("layout"));
                                                }
                                                __field1 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Option<SymbolLayout>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field2 => {
                                                if _serde::export::Option::is_some(&__field2) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("paint"));
                                                }
                                                __field2 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Option<SymbolPaint>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__other(__name) => {
                                                __collect.push(_serde::export::Some((
                                                    __name,
                                                    match _serde::de::MapAccess::next_value(
                                                        &mut __map,
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                )));
                                            }
                                        }
                                    }
                                    let __field1 = match __field1 {
                                        _serde::export::Some(__field1) => __field1,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field("layout") {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    let __field2 = match __field2 {
                                        _serde::export::Some(__field2) => __field2,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field("paint") {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    let __field0: LayerCommon =
                                        match _serde::de::Deserialize::deserialize(
                                            _serde::private::de::FlatMapDeserializer(
                                                &mut __collect,
                                                _serde::export::PhantomData,
                                            ),
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        };
                                    _serde::export::Ok(SymbolLayer {
                                        common: __field0,
                                        layout: __field1,
                                        paint: __field2,
                                    })
                                }
                            }
                            _serde::Deserializer::deserialize_map(
                                __deserializer,
                                __Visitor {
                                    marker: _serde::export::PhantomData::<SymbolLayer>,
                                    lifetime: _serde::export::PhantomData,
                                },
                            )
                        }
                    }
                };
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl ::std::fmt::Debug for SymbolLayer {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        match *self {
                            SymbolLayer {
                                common: ref __self_0_0,
                                layout: ref __self_0_1,
                                paint: ref __self_0_2,
                            } => {
                                let mut debug_trait_builder = f.debug_struct("SymbolLayer");
                                let _ = debug_trait_builder.field("common", &&(*__self_0_0));
                                let _ = debug_trait_builder.field("layout", &&(*__self_0_1));
                                let _ = debug_trait_builder.field("paint", &&(*__self_0_2));
                                debug_trait_builder.finish()
                            }
                        }
                    }
                }
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl ::std::clone::Clone for SymbolLayer {
                    #[inline]
                    fn clone(&self) -> SymbolLayer {
                        match *self {
                            SymbolLayer {
                                common: ref __self_0_0,
                                layout: ref __self_0_1,
                                paint: ref __self_0_2,
                            } => SymbolLayer {
                                common: ::std::clone::Clone::clone(&(*__self_0_0)),
                                layout: ::std::clone::Clone::clone(&(*__self_0_1)),
                                paint: ::std::clone::Clone::clone(&(*__self_0_2)),
                            },
                        }
                    }
                }
                pub struct SymbolLayout {
                    #[serde(rename = "symbol-placement")]
                    placement: Option<Function<String>>,
                    #[serde(rename = "symbol-spacing")]
                    spacing: Option<Function<f32>>,
                    #[serde(rename = "symbol-avoid-edges")]
                    avoid_edges: Option<bool>,
                }
                #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
                const _IMPL_DESERIALIZE_FOR_SymbolLayout: () = {
                    extern crate serde as _serde;
                    #[allow(unused_macros)]
                    macro_rules! try(( $ __expr : expr ) => {
                                         match $ __expr {
                                         _serde :: export :: Ok ( __val ) =>
                                         __val , _serde :: export :: Err (
                                         __err ) => {
                                         return _serde :: export :: Err (
                                         __err ) ; } } });
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for SymbolLayout {
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::export::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            #[allow(non_camel_case_types)]
                            enum __Field {
                                __field0,
                                __field1,
                                __field2,
                                __ignore,
                            }
                            struct __FieldVisitor;
                            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                type Value = __Field;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::export::Formatter,
                                ) -> _serde::export::fmt::Result {
                                    _serde::export::Formatter::write_str(
                                        __formatter,
                                        "field identifier",
                                    )
                                }
                                fn visit_u64<__E>(
                                    self,
                                    __value: u64,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        0u64 => _serde::export::Ok(__Field::__field0),
                                        1u64 => _serde::export::Ok(__Field::__field1),
                                        2u64 => _serde::export::Ok(__Field::__field2),
                                        _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                            _serde::de::Unexpected::Unsigned(__value),
                                            &"field index 0 <= i < 3",
                                        )),
                                    }
                                }
                                fn visit_str<__E>(
                                    self,
                                    __value: &str,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        "symbol-placement" => _serde::export::Ok(__Field::__field0),
                                        "symbol-spacing" => _serde::export::Ok(__Field::__field1),
                                        "symbol-avoid-edges" => {
                                            _serde::export::Ok(__Field::__field2)
                                        }
                                        _ => _serde::export::Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(
                                    self,
                                    __value: &[u8],
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        b"symbol-placement" => {
                                            _serde::export::Ok(__Field::__field0)
                                        }
                                        b"symbol-spacing" => _serde::export::Ok(__Field::__field1),
                                        b"symbol-avoid-edges" => {
                                            _serde::export::Ok(__Field::__field2)
                                        }
                                        _ => _serde::export::Ok(__Field::__ignore),
                                    }
                                }
                            }
                            impl<'de> _serde::Deserialize<'de> for __Field {
                                #[inline]
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::export::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::Deserializer::deserialize_identifier(
                                        __deserializer,
                                        __FieldVisitor,
                                    )
                                }
                            }
                            struct __Visitor<'de> {
                                marker: _serde::export::PhantomData<SymbolLayout>,
                                lifetime: _serde::export::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                type Value = SymbolLayout;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::export::Formatter,
                                ) -> _serde::export::fmt::Result {
                                    _serde::export::Formatter::write_str(
                                        __formatter,
                                        "struct SymbolLayout",
                                    )
                                }
                                #[inline]
                                fn visit_seq<__A>(
                                    self,
                                    mut __seq: __A,
                                ) -> _serde::export::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::SeqAccess<'de>,
                                {
                                    let __field0 = match match _serde::de::SeqAccess::next_element::<
                                        Option<Function<String>>,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &"struct SymbolLayout with 3 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field1 = match match _serde::de::SeqAccess::next_element::<
                                        Option<Function<f32>>,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    1usize,
                                                    &"struct SymbolLayout with 3 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field2 = match match _serde::de::SeqAccess::next_element::<
                                        Option<bool>,
                                    >(
                                        &mut __seq
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    2usize,
                                                    &"struct SymbolLayout with 3 elements",
                                                ),
                                            );
                                        }
                                    };
                                    _serde::export::Ok(SymbolLayout {
                                        placement: __field0,
                                        spacing: __field1,
                                        avoid_edges: __field2,
                                    })
                                }
                                #[inline]
                                fn visit_map<__A>(
                                    self,
                                    mut __map: __A,
                                ) -> _serde::export::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::MapAccess<'de>,
                                {
                                    let mut __field0:
                                                _serde::export::Option<Option<Function<String>>> =
                                            _serde::export::None;
                                    let mut __field1:
                                                _serde::export::Option<Option<Function<f32>>> =
                                            _serde::export::None;
                                    let mut __field2:
                                                _serde::export::Option<Option<bool>> =
                                            _serde::export::None;
                                    while let _serde::export::Some(__key) =
                                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        } {
                                        match __key {
                                            __Field::__field0 => {
                                                if _serde::export::Option::is_some(&__field0) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("symbol-placement"));
                                                }
                                                __field0 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Option<Function<String>>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field1 => {
                                                if _serde::export::Option::is_some(&__field1) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("symbol-spacing"));
                                                }
                                                __field1 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Option<Function<f32>>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            __Field::__field2 => {
                                                if _serde::export::Option::is_some(&__field2) {
                                                    return _serde::export::Err(<__A::Error
                                                                                       as
                                                                                       _serde::de::Error>::duplicate_field("symbol-avoid-edges"));
                                                }
                                                __field2 = _serde::export::Some(
                                                    match _serde::de::MapAccess::next_value::<
                                                        Option<bool>,
                                                    >(
                                                        &mut __map
                                                    ) {
                                                        _serde::export::Ok(__val) => __val,
                                                        _serde::export::Err(__err) => {
                                                            return _serde::export::Err(__err);
                                                        }
                                                    },
                                                );
                                            }
                                            _ => {
                                                let _ = match _serde::de::MapAccess::next_value::<
                                                    _serde::de::IgnoredAny,
                                                >(
                                                    &mut __map
                                                ) {
                                                    _serde::export::Ok(__val) => __val,
                                                    _serde::export::Err(__err) => {
                                                        return _serde::export::Err(__err);
                                                    }
                                                };
                                            }
                                        }
                                    }
                                    let __field0 = match __field0 {
                                        _serde::export::Some(__field0) => __field0,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field(
                                                "symbol-placement",
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    let __field1 = match __field1 {
                                        _serde::export::Some(__field1) => __field1,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field(
                                                "symbol-spacing",
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    let __field2 = match __field2 {
                                        _serde::export::Some(__field2) => __field2,
                                        _serde::export::None => {
                                            match _serde::private::de::missing_field(
                                                "symbol-avoid-edges",
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            }
                                        }
                                    };
                                    _serde::export::Ok(SymbolLayout {
                                        placement: __field0,
                                        spacing: __field1,
                                        avoid_edges: __field2,
                                    })
                                }
                            }
                            const FIELDS: &'static [&'static str] =
                                &["symbol-placement", "symbol-spacing", "symbol-avoid-edges"];
                            _serde::Deserializer::deserialize_struct(
                                __deserializer,
                                "SymbolLayout",
                                FIELDS,
                                __Visitor {
                                    marker: _serde::export::PhantomData::<SymbolLayout>,
                                    lifetime: _serde::export::PhantomData,
                                },
                            )
                        }
                    }
                };
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl ::std::fmt::Debug for SymbolLayout {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        match *self {
                            SymbolLayout {
                                placement: ref __self_0_0,
                                spacing: ref __self_0_1,
                                avoid_edges: ref __self_0_2,
                            } => {
                                let mut debug_trait_builder = f.debug_struct("SymbolLayout");
                                let _ = debug_trait_builder.field("placement", &&(*__self_0_0));
                                let _ = debug_trait_builder.field("spacing", &&(*__self_0_1));
                                let _ = debug_trait_builder.field("avoid_edges", &&(*__self_0_2));
                                debug_trait_builder.finish()
                            }
                        }
                    }
                }
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl ::std::clone::Clone for SymbolLayout {
                    #[inline]
                    fn clone(&self) -> SymbolLayout {
                        match *self {
                            SymbolLayout {
                                placement: ref __self_0_0,
                                spacing: ref __self_0_1,
                                avoid_edges: ref __self_0_2,
                            } => SymbolLayout {
                                placement: ::std::clone::Clone::clone(&(*__self_0_0)),
                                spacing: ::std::clone::Clone::clone(&(*__self_0_1)),
                                avoid_edges: ::std::clone::Clone::clone(&(*__self_0_2)),
                            },
                        }
                    }
                }
                pub struct SymbolPaint {}
                #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
                const _IMPL_DESERIALIZE_FOR_SymbolPaint: () = {
                    extern crate serde as _serde;
                    #[allow(unused_macros)]
                    macro_rules! try(( $ __expr : expr ) => {
                                         match $ __expr {
                                         _serde :: export :: Ok ( __val ) =>
                                         __val , _serde :: export :: Err (
                                         __err ) => {
                                         return _serde :: export :: Err (
                                         __err ) ; } } });
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for SymbolPaint {
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::export::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            #[allow(non_camel_case_types)]
                            enum __Field {
                                __ignore,
                            }
                            struct __FieldVisitor;
                            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                type Value = __Field;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::export::Formatter,
                                ) -> _serde::export::fmt::Result {
                                    _serde::export::Formatter::write_str(
                                        __formatter,
                                        "field identifier",
                                    )
                                }
                                fn visit_u64<__E>(
                                    self,
                                    __value: u64,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                            _serde::de::Unexpected::Unsigned(__value),
                                            &"field index 0 <= i < 0",
                                        )),
                                    }
                                }
                                fn visit_str<__E>(
                                    self,
                                    __value: &str,
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        _ => _serde::export::Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(
                                    self,
                                    __value: &[u8],
                                ) -> _serde::export::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        _ => _serde::export::Ok(__Field::__ignore),
                                    }
                                }
                            }
                            impl<'de> _serde::Deserialize<'de> for __Field {
                                #[inline]
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::export::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::Deserializer::deserialize_identifier(
                                        __deserializer,
                                        __FieldVisitor,
                                    )
                                }
                            }
                            struct __Visitor<'de> {
                                marker: _serde::export::PhantomData<SymbolPaint>,
                                lifetime: _serde::export::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                type Value = SymbolPaint;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::export::Formatter,
                                ) -> _serde::export::fmt::Result {
                                    _serde::export::Formatter::write_str(
                                        __formatter,
                                        "struct SymbolPaint",
                                    )
                                }
                                #[inline]
                                fn visit_seq<__A>(
                                    self,
                                    _: __A,
                                ) -> _serde::export::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::SeqAccess<'de>,
                                {
                                    _serde::export::Ok(SymbolPaint {})
                                }
                                #[inline]
                                fn visit_map<__A>(
                                    self,
                                    mut __map: __A,
                                ) -> _serde::export::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::MapAccess<'de>,
                                {
                                    while let _serde::export::Some(__key) =
                                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map)
                                        {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        } {
                                        match __key {
                                            _ => {
                                                let _ = match _serde::de::MapAccess::next_value::<
                                                    _serde::de::IgnoredAny,
                                                >(
                                                    &mut __map
                                                ) {
                                                    _serde::export::Ok(__val) => __val,
                                                    _serde::export::Err(__err) => {
                                                        return _serde::export::Err(__err);
                                                    }
                                                };
                                            }
                                        }
                                    }
                                    _serde::export::Ok(SymbolPaint {})
                                }
                            }
                            const FIELDS: &'static [&'static str] = &[];
                            _serde::Deserializer::deserialize_struct(
                                __deserializer,
                                "SymbolPaint",
                                FIELDS,
                                __Visitor {
                                    marker: _serde::export::PhantomData::<SymbolPaint>,
                                    lifetime: _serde::export::PhantomData,
                                },
                            )
                        }
                    }
                };
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl ::std::fmt::Debug for SymbolPaint {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        match *self {
                            SymbolPaint {} => {
                                let mut debug_trait_builder = f.debug_struct("SymbolPaint");
                                debug_trait_builder.finish()
                            }
                        }
                    }
                }
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl ::std::clone::Clone for SymbolPaint {
                    #[inline]
                    fn clone(&self) -> SymbolPaint {
                        match *self {
                            SymbolPaint {} => SymbolPaint {},
                        }
                    }
                }
            }
            pub use self::background::*;
            pub use self::fill::*;
            pub use self::line::*;
            pub use self::raster::*;
            pub use self::symbol::*;
            use super::expr::Filter;
            pub struct LayerCommon {
                pub id: String,
                pub source: Option<String>,
                #[serde(rename = "source-layer")]
                pub source_layer: Option<String>,
                pub minzoom: Option<f32>,
                pub maxzoom: Option<f32>,
                pub filter: Option<Filter>,
            }
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _IMPL_DESERIALIZE_FOR_LayerCommon: () = {
                extern crate serde as _serde;
                #[allow(unused_macros)]
                macro_rules! try(( $ __expr : expr ) => {
                                     match $ __expr {
                                     _serde :: export :: Ok ( __val ) => __val
                                     , _serde :: export :: Err ( __err ) => {
                                     return _serde :: export :: Err ( __err )
                                     ; } } });
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for LayerCommon {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        enum __Field {
                            __field0,
                            __field1,
                            __field2,
                            __field3,
                            __field4,
                            __field5,
                            __ignore,
                        }
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::export::Formatter,
                            ) -> _serde::export::fmt::Result {
                                _serde::export::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::export::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::export::Ok(__Field::__field0),
                                    1u64 => _serde::export::Ok(__Field::__field1),
                                    2u64 => _serde::export::Ok(__Field::__field2),
                                    3u64 => _serde::export::Ok(__Field::__field3),
                                    4u64 => _serde::export::Ok(__Field::__field4),
                                    5u64 => _serde::export::Ok(__Field::__field5),
                                    _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"field index 0 <= i < 6",
                                    )),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::export::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "id" => _serde::export::Ok(__Field::__field0),
                                    "source" => _serde::export::Ok(__Field::__field1),
                                    "source-layer" => _serde::export::Ok(__Field::__field2),
                                    "minzoom" => _serde::export::Ok(__Field::__field3),
                                    "maxzoom" => _serde::export::Ok(__Field::__field4),
                                    "filter" => _serde::export::Ok(__Field::__field5),
                                    _ => _serde::export::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::export::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"id" => _serde::export::Ok(__Field::__field0),
                                    b"source" => _serde::export::Ok(__Field::__field1),
                                    b"source-layer" => _serde::export::Ok(__Field::__field2),
                                    b"minzoom" => _serde::export::Ok(__Field::__field3),
                                    b"maxzoom" => _serde::export::Ok(__Field::__field4),
                                    b"filter" => _serde::export::Ok(__Field::__field5),
                                    _ => _serde::export::Ok(__Field::__ignore),
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::export::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        struct __Visitor<'de> {
                            marker: _serde::export::PhantomData<LayerCommon>,
                            lifetime: _serde::export::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = LayerCommon;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::export::Formatter,
                            ) -> _serde::export::fmt::Result {
                                _serde::export::Formatter::write_str(
                                    __formatter,
                                    "struct LayerCommon",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::export::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 =
                                    match match _serde::de::SeqAccess::next_element::<String>(
                                        &mut __seq,
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &"struct LayerCommon with 6 elements",
                                                ),
                                            );
                                        }
                                    };
                                let __field1 = match match _serde::de::SeqAccess::next_element::<
                                    Option<String>,
                                >(
                                    &mut __seq
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                } {
                                    _serde::export::Some(__value) => __value,
                                    _serde::export::None => {
                                        return _serde::export::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct LayerCommon with 6 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field2 = match match _serde::de::SeqAccess::next_element::<
                                    Option<String>,
                                >(
                                    &mut __seq
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                } {
                                    _serde::export::Some(__value) => __value,
                                    _serde::export::None => {
                                        return _serde::export::Err(
                                            _serde::de::Error::invalid_length(
                                                2usize,
                                                &"struct LayerCommon with 6 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field3 =
                                    match match _serde::de::SeqAccess::next_element::<Option<f32>>(
                                        &mut __seq,
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    3usize,
                                                    &"struct LayerCommon with 6 elements",
                                                ),
                                            );
                                        }
                                    };
                                let __field4 =
                                    match match _serde::de::SeqAccess::next_element::<Option<f32>>(
                                        &mut __seq,
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    4usize,
                                                    &"struct LayerCommon with 6 elements",
                                                ),
                                            );
                                        }
                                    };
                                let __field5 = match match _serde::de::SeqAccess::next_element::<
                                    Option<Filter>,
                                >(
                                    &mut __seq
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                } {
                                    _serde::export::Some(__value) => __value,
                                    _serde::export::None => {
                                        return _serde::export::Err(
                                            _serde::de::Error::invalid_length(
                                                5usize,
                                                &"struct LayerCommon with 6 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::export::Ok(LayerCommon {
                                    id: __field0,
                                    source: __field1,
                                    source_layer: __field2,
                                    minzoom: __field3,
                                    maxzoom: __field4,
                                    filter: __field5,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::export::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0:
                                            _serde::export::Option<String> =
                                        _serde::export::None;
                                let mut __field1:
                                            _serde::export::Option<Option<String>> =
                                        _serde::export::None;
                                let mut __field2:
                                            _serde::export::Option<Option<String>> =
                                        _serde::export::None;
                                let mut __field3:
                                            _serde::export::Option<Option<f32>> =
                                        _serde::export::None;
                                let mut __field4:
                                            _serde::export::Option<Option<f32>> =
                                        _serde::export::None;
                                let mut __field5:
                                            _serde::export::Option<Option<Filter>> =
                                        _serde::export::None;
                                while let _serde::export::Some(__key) =
                                    match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::export::Option::is_some(&__field0) {
                                                return _serde::export::Err(<__A::Error
                                                                                   as
                                                                                   _serde::de::Error>::duplicate_field("id"));
                                            }
                                            __field0 = _serde::export::Some(
                                                match _serde::de::MapAccess::next_value::<String>(
                                                    &mut __map,
                                                ) {
                                                    _serde::export::Ok(__val) => __val,
                                                    _serde::export::Err(__err) => {
                                                        return _serde::export::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::export::Option::is_some(&__field1) {
                                                return _serde::export::Err(<__A::Error
                                                                                   as
                                                                                   _serde::de::Error>::duplicate_field("source"));
                                            }
                                            __field1 = _serde::export::Some(
                                                match _serde::de::MapAccess::next_value::<
                                                    Option<String>,
                                                >(
                                                    &mut __map
                                                ) {
                                                    _serde::export::Ok(__val) => __val,
                                                    _serde::export::Err(__err) => {
                                                        return _serde::export::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field2 => {
                                            if _serde::export::Option::is_some(&__field2) {
                                                return _serde::export::Err(<__A::Error
                                                                                   as
                                                                                   _serde::de::Error>::duplicate_field("source-layer"));
                                            }
                                            __field2 = _serde::export::Some(
                                                match _serde::de::MapAccess::next_value::<
                                                    Option<String>,
                                                >(
                                                    &mut __map
                                                ) {
                                                    _serde::export::Ok(__val) => __val,
                                                    _serde::export::Err(__err) => {
                                                        return _serde::export::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field3 => {
                                            if _serde::export::Option::is_some(&__field3) {
                                                return _serde::export::Err(<__A::Error
                                                                                   as
                                                                                   _serde::de::Error>::duplicate_field("minzoom"));
                                            }
                                            __field3 = _serde::export::Some(
                                                match _serde::de::MapAccess::next_value::<Option<f32>>(
                                                    &mut __map,
                                                ) {
                                                    _serde::export::Ok(__val) => __val,
                                                    _serde::export::Err(__err) => {
                                                        return _serde::export::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field4 => {
                                            if _serde::export::Option::is_some(&__field4) {
                                                return _serde::export::Err(<__A::Error
                                                                                   as
                                                                                   _serde::de::Error>::duplicate_field("maxzoom"));
                                            }
                                            __field4 = _serde::export::Some(
                                                match _serde::de::MapAccess::next_value::<Option<f32>>(
                                                    &mut __map,
                                                ) {
                                                    _serde::export::Ok(__val) => __val,
                                                    _serde::export::Err(__err) => {
                                                        return _serde::export::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field5 => {
                                            if _serde::export::Option::is_some(&__field5) {
                                                return _serde::export::Err(<__A::Error
                                                                                   as
                                                                                   _serde::de::Error>::duplicate_field("filter"));
                                            }
                                            __field5 = _serde::export::Some(
                                                match _serde::de::MapAccess::next_value::<
                                                    Option<Filter>,
                                                >(
                                                    &mut __map
                                                ) {
                                                    _serde::export::Ok(__val) => __val,
                                                    _serde::export::Err(__err) => {
                                                        return _serde::export::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        _ => {
                                            let _ = match _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            };
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::export::Some(__field0) => __field0,
                                    _serde::export::None => {
                                        match _serde::private::de::missing_field("id") {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::export::Some(__field1) => __field1,
                                    _serde::export::None => {
                                        match _serde::private::de::missing_field("source") {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field2 = match __field2 {
                                    _serde::export::Some(__field2) => __field2,
                                    _serde::export::None => {
                                        match _serde::private::de::missing_field("source-layer") {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field3 = match __field3 {
                                    _serde::export::Some(__field3) => __field3,
                                    _serde::export::None => {
                                        match _serde::private::de::missing_field("minzoom") {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field4 = match __field4 {
                                    _serde::export::Some(__field4) => __field4,
                                    _serde::export::None => {
                                        match _serde::private::de::missing_field("maxzoom") {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field5 = match __field5 {
                                    _serde::export::Some(__field5) => __field5,
                                    _serde::export::None => {
                                        match _serde::private::de::missing_field("filter") {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        }
                                    }
                                };
                                _serde::export::Ok(LayerCommon {
                                    id: __field0,
                                    source: __field1,
                                    source_layer: __field2,
                                    minzoom: __field3,
                                    maxzoom: __field4,
                                    filter: __field5,
                                })
                            }
                        }
                        const FIELDS: &'static [&'static str] = &[
                            "id",
                            "source",
                            "source-layer",
                            "minzoom",
                            "maxzoom",
                            "filter",
                        ];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "LayerCommon",
                            FIELDS,
                            __Visitor {
                                marker: _serde::export::PhantomData::<LayerCommon>,
                                lifetime: _serde::export::PhantomData,
                            },
                        )
                    }
                }
            };
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::fmt::Debug for LayerCommon {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        LayerCommon {
                            id: ref __self_0_0,
                            source: ref __self_0_1,
                            source_layer: ref __self_0_2,
                            minzoom: ref __self_0_3,
                            maxzoom: ref __self_0_4,
                            filter: ref __self_0_5,
                        } => {
                            let mut debug_trait_builder = f.debug_struct("LayerCommon");
                            let _ = debug_trait_builder.field("id", &&(*__self_0_0));
                            let _ = debug_trait_builder.field("source", &&(*__self_0_1));
                            let _ = debug_trait_builder.field("source_layer", &&(*__self_0_2));
                            let _ = debug_trait_builder.field("minzoom", &&(*__self_0_3));
                            let _ = debug_trait_builder.field("maxzoom", &&(*__self_0_4));
                            let _ = debug_trait_builder.field("filter", &&(*__self_0_5));
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::clone::Clone for LayerCommon {
                #[inline]
                fn clone(&self) -> LayerCommon {
                    match *self {
                        LayerCommon {
                            id: ref __self_0_0,
                            source: ref __self_0_1,
                            source_layer: ref __self_0_2,
                            minzoom: ref __self_0_3,
                            maxzoom: ref __self_0_4,
                            filter: ref __self_0_5,
                        } => LayerCommon {
                            id: ::std::clone::Clone::clone(&(*__self_0_0)),
                            source: ::std::clone::Clone::clone(&(*__self_0_1)),
                            source_layer: ::std::clone::Clone::clone(&(*__self_0_2)),
                            minzoom: ::std::clone::Clone::clone(&(*__self_0_3)),
                            maxzoom: ::std::clone::Clone::clone(&(*__self_0_4)),
                            filter: ::std::clone::Clone::clone(&(*__self_0_5)),
                        },
                    }
                }
            }
            pub enum Visibility {
                #[serde(rename = "visible")]
                Visible,

                #[serde(rename = "none")]
                Invisible,
            }
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _IMPL_DESERIALIZE_FOR_Visibility: () = {
                extern crate serde as _serde;
                #[allow(unused_macros)]
                macro_rules! try(( $ __expr : expr ) => {
                                     match $ __expr {
                                     _serde :: export :: Ok ( __val ) => __val
                                     , _serde :: export :: Err ( __err ) => {
                                     return _serde :: export :: Err ( __err )
                                     ; } } });
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for Visibility {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        enum __Field {
                            __field0,
                            __field1,
                        }
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::export::Formatter,
                            ) -> _serde::export::fmt::Result {
                                _serde::export::Formatter::write_str(
                                    __formatter,
                                    "variant identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::export::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::export::Ok(__Field::__field0),
                                    1u64 => _serde::export::Ok(__Field::__field1),
                                    _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 2",
                                    )),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::export::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "visible" => _serde::export::Ok(__Field::__field0),
                                    "none" => _serde::export::Ok(__Field::__field1),
                                    _ => _serde::export::Err(_serde::de::Error::unknown_variant(
                                        __value, VARIANTS,
                                    )),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::export::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"visible" => _serde::export::Ok(__Field::__field0),
                                    b"none" => _serde::export::Ok(__Field::__field1),
                                    _ => {
                                        let __value = &_serde::export::from_utf8_lossy(__value);
                                        _serde::export::Err(_serde::de::Error::unknown_variant(
                                            __value, VARIANTS,
                                        ))
                                    }
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::export::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        struct __Visitor<'de> {
                            marker: _serde::export::PhantomData<Visibility>,
                            lifetime: _serde::export::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = Visibility;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::export::Formatter,
                            ) -> _serde::export::fmt::Result {
                                _serde::export::Formatter::write_str(__formatter, "enum Visibility")
                            }
                            fn visit_enum<__A>(
                                self,
                                __data: __A,
                            ) -> _serde::export::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::EnumAccess<'de>,
                            {
                                match match _serde::de::EnumAccess::variant(__data) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                } {
                                    (__Field::__field0, __variant) => {
                                        match _serde::de::VariantAccess::unit_variant(__variant) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        };
                                        _serde::export::Ok(Visibility::Visible)
                                    }
                                    (__Field::__field1, __variant) => {
                                        match _serde::de::VariantAccess::unit_variant(__variant) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        };
                                        _serde::export::Ok(Visibility::Invisible)
                                    }
                                }
                            }
                        }
                        const VARIANTS: &'static [&'static str] = &["visible", "none"];
                        _serde::Deserializer::deserialize_enum(
                            __deserializer,
                            "Visibility",
                            VARIANTS,
                            __Visitor {
                                marker: _serde::export::PhantomData::<Visibility>,
                                lifetime: _serde::export::PhantomData,
                            },
                        )
                    }
                }
            };
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::fmt::Debug for Visibility {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match (&*self,) {
                        (&Visibility::Visible,) => {
                            let mut debug_trait_builder = f.debug_tuple("Visible");
                            debug_trait_builder.finish()
                        }
                        (&Visibility::Invisible,) => {
                            let mut debug_trait_builder = f.debug_tuple("Invisible");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::clone::Clone for Visibility {
                #[inline]
                fn clone(&self) -> Visibility {
                    match (&*self,) {
                        (&Visibility::Visible,) => Visibility::Visible,
                        (&Visibility::Invisible,) => Visibility::Invisible,
                    }
                }
            }
            impl Default for Visibility {
                fn default() -> Self {
                    Visibility::Visible
                }
            }
            pub struct BaseLayout {
                #[serde(default = "Visibility::default")]
                visibility: Visibility,
            }
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _IMPL_DESERIALIZE_FOR_BaseLayout: () = {
                extern crate serde as _serde;
                #[allow(unused_macros)]
                macro_rules! try(( $ __expr : expr ) => {
                                     match $ __expr {
                                     _serde :: export :: Ok ( __val ) => __val
                                     , _serde :: export :: Err ( __err ) => {
                                     return _serde :: export :: Err ( __err )
                                     ; } } });
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for BaseLayout {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::export::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        enum __Field {
                            __field0,
                            __ignore,
                        }
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::export::Formatter,
                            ) -> _serde::export::fmt::Result {
                                _serde::export::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::export::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::export::Ok(__Field::__field0),
                                    _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"field index 0 <= i < 1",
                                    )),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::export::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "visibility" => _serde::export::Ok(__Field::__field0),
                                    _ => _serde::export::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::export::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"visibility" => _serde::export::Ok(__Field::__field0),
                                    _ => _serde::export::Ok(__Field::__ignore),
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::export::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        struct __Visitor<'de> {
                            marker: _serde::export::PhantomData<BaseLayout>,
                            lifetime: _serde::export::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = BaseLayout;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::export::Formatter,
                            ) -> _serde::export::fmt::Result {
                                _serde::export::Formatter::write_str(
                                    __formatter,
                                    "struct BaseLayout",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::export::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 =
                                    match match _serde::de::SeqAccess::next_element::<Visibility>(
                                        &mut __seq,
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                        _serde::export::Some(__value) => __value,
                                        _serde::export::None => {
                                            return _serde::export::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &"struct BaseLayout with 1 element",
                                                ),
                                            );
                                        }
                                    };
                                _serde::export::Ok(BaseLayout {
                                    visibility: __field0,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::export::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0:
                                            _serde::export::Option<Visibility> =
                                        _serde::export::None;
                                while let _serde::export::Some(__key) =
                                    match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    } {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::export::Option::is_some(&__field0) {
                                                return _serde::export::Err(<__A::Error
                                                                                   as
                                                                                   _serde::de::Error>::duplicate_field("visibility"));
                                            }
                                            __field0 = _serde::export::Some(
                                                match _serde::de::MapAccess::next_value::<Visibility>(
                                                    &mut __map,
                                                ) {
                                                    _serde::export::Ok(__val) => __val,
                                                    _serde::export::Err(__err) => {
                                                        return _serde::export::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        _ => {
                                            let _ = match _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            };
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::export::Some(__field0) => __field0,
                                    _serde::export::None => Visibility::default(),
                                };
                                _serde::export::Ok(BaseLayout {
                                    visibility: __field0,
                                })
                            }
                        }
                        const FIELDS: &'static [&'static str] = &["visibility"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "BaseLayout",
                            FIELDS,
                            __Visitor {
                                marker: _serde::export::PhantomData::<BaseLayout>,
                                lifetime: _serde::export::PhantomData,
                            },
                        )
                    }
                }
            };
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::fmt::Debug for BaseLayout {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        BaseLayout {
                            visibility: ref __self_0_0,
                        } => {
                            let mut debug_trait_builder = f.debug_struct("BaseLayout");
                            let _ = debug_trait_builder.field("visibility", &&(*__self_0_0));
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::clone::Clone for BaseLayout {
                #[inline]
                fn clone(&self) -> BaseLayout {
                    match *self {
                        BaseLayout {
                            visibility: ref __self_0_0,
                        } => BaseLayout {
                            visibility: ::std::clone::Clone::clone(&(*__self_0_0)),
                        },
                    }
                }
            }
            impl Default for BaseLayout {
                fn default() -> Self {
                    BaseLayout {
                        visibility: Default::default(),
                    }
                }
            }
        }
        use self::color::Color;
        use self::function::Function;
        pub use self::layers::*;
        pub struct TileJson {
            scheme: Option<String>,
            tiles: Option<Vec<String>>,
            minzoom: Option<f32>,
            maxzoom: Option<f32>,
            bounds: Option<[f32; 4]>,
            #[serde(rename = "tileSize")]
            tile_size: Option<i32>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for TileJson {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    TileJson {
                        scheme: ref __self_0_0,
                        tiles: ref __self_0_1,
                        minzoom: ref __self_0_2,
                        maxzoom: ref __self_0_3,
                        bounds: ref __self_0_4,
                        tile_size: ref __self_0_5,
                    } => {
                        let mut debug_trait_builder = f.debug_struct("TileJson");
                        let _ = debug_trait_builder.field("scheme", &&(*__self_0_0));
                        let _ = debug_trait_builder.field("tiles", &&(*__self_0_1));
                        let _ = debug_trait_builder.field("minzoom", &&(*__self_0_2));
                        let _ = debug_trait_builder.field("maxzoom", &&(*__self_0_3));
                        let _ = debug_trait_builder.field("bounds", &&(*__self_0_4));
                        let _ = debug_trait_builder.field("tile_size", &&(*__self_0_5));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _IMPL_DESERIALIZE_FOR_TileJson: () = {
            extern crate serde as _serde;
            #[allow(unused_macros)]
            macro_rules! try(( $ __expr : expr ) => {
                                 match $ __expr {
                                 _serde :: export :: Ok ( __val ) => __val ,
                                 _serde :: export :: Err ( __err ) => {
                                 return _serde :: export :: Err ( __err ) ; }
                                 } });
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for TileJson {
                fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::export::Formatter,
                        ) -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::export::Ok(__Field::__field0),
                                1u64 => _serde::export::Ok(__Field::__field1),
                                2u64 => _serde::export::Ok(__Field::__field2),
                                3u64 => _serde::export::Ok(__Field::__field3),
                                4u64 => _serde::export::Ok(__Field::__field4),
                                5u64 => _serde::export::Ok(__Field::__field5),
                                _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"field index 0 <= i < 6",
                                )),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "scheme" => _serde::export::Ok(__Field::__field0),
                                "tiles" => _serde::export::Ok(__Field::__field1),
                                "minzoom" => _serde::export::Ok(__Field::__field2),
                                "maxzoom" => _serde::export::Ok(__Field::__field3),
                                "bounds" => _serde::export::Ok(__Field::__field4),
                                "tileSize" => _serde::export::Ok(__Field::__field5),
                                _ => _serde::export::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"scheme" => _serde::export::Ok(__Field::__field0),
                                b"tiles" => _serde::export::Ok(__Field::__field1),
                                b"minzoom" => _serde::export::Ok(__Field::__field2),
                                b"maxzoom" => _serde::export::Ok(__Field::__field3),
                                b"bounds" => _serde::export::Ok(__Field::__field4),
                                b"tileSize" => _serde::export::Ok(__Field::__field5),
                                _ => _serde::export::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::export::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::export::PhantomData<TileJson>,
                        lifetime: _serde::export::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = TileJson;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::export::Formatter,
                        ) -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter, "struct TileJson")
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::export::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct TileJson with 6 elements",
                                    ));
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                Option<Vec<String>>,
                            >(&mut __seq)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct TileJson with 6 elements",
                                    ));
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<
                                Option<f32>,
                            >(&mut __seq)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct TileJson with 6 elements",
                                    ));
                                }
                            };
                            let __field3 = match match _serde::de::SeqAccess::next_element::<
                                Option<f32>,
                            >(&mut __seq)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct TileJson with 6 elements",
                                    ));
                                }
                            };
                            let __field4 = match match _serde::de::SeqAccess::next_element::<
                                Option<[f32; 4]>,
                            >(&mut __seq)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        4usize,
                                        &"struct TileJson with 6 elements",
                                    ));
                                }
                            };
                            let __field5 = match match _serde::de::SeqAccess::next_element::<
                                Option<i32>,
                            >(&mut __seq)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        5usize,
                                        &"struct TileJson with 6 elements",
                                    ));
                                }
                            };
                            _serde::export::Ok(TileJson {
                                scheme: __field0,
                                tiles: __field1,
                                minzoom: __field2,
                                maxzoom: __field3,
                                bounds: __field4,
                                tile_size: __field5,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::export::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::export::Option<
                                Option<String>,
                            > = _serde::export::None;
                            let mut __field1: _serde::export::Option<
                                Option<Vec<String>>,
                            > = _serde::export::None;
                            let mut __field2: _serde::export::Option<
                                Option<f32>,
                            > = _serde::export::None;
                            let mut __field3: _serde::export::Option<
                                Option<f32>,
                            > = _serde::export::None;
                            let mut __field4: _serde::export::Option<
                                Option<[f32; 4]>,
                            > = _serde::export::None;
                            let mut __field5: _serde::export::Option<
                                Option<i32>,
                            > = _serde::export::None;
                            while let _serde::export::Some(__key) =
                                match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::export::Option::is_some(&__field0) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "scheme",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<Option<String>>(
                                                &mut __map,
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::export::Option::is_some(&__field1) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "tiles",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<Vec<String>>,
                                            >(
                                                &mut __map
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::export::Option::is_some(&__field2) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "minzoom",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<Option<f32>>(
                                                &mut __map,
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::export::Option::is_some(&__field3) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "maxzoom",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<Option<f32>>(
                                                &mut __map,
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::export::Option::is_some(&__field4) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "bounds",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<[f32; 4]>,
                                            >(
                                                &mut __map
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field5 => {
                                        if _serde::export::Option::is_some(&__field5) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "tileSize",
                                                ),
                                            );
                                        }
                                        __field5 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<Option<i32>>(
                                                &mut __map,
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(
                                            &mut __map
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::export::Some(__field0) => __field0,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("scheme") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::export::Some(__field1) => __field1,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("tiles") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::export::Some(__field2) => __field2,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("minzoom") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::export::Some(__field3) => __field3,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("maxzoom") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::export::Some(__field4) => __field4,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("bounds") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field5 = match __field5 {
                                _serde::export::Some(__field5) => __field5,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("tileSize") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::export::Ok(TileJson {
                                scheme: __field0,
                                tiles: __field1,
                                minzoom: __field2,
                                maxzoom: __field3,
                                bounds: __field4,
                                tile_size: __field5,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[
                        "scheme", "tiles", "minzoom", "maxzoom", "bounds", "tileSize",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "TileJson",
                        FIELDS,
                        __Visitor {
                            marker: _serde::export::PhantomData::<TileJson>,
                            lifetime: _serde::export::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for TileJson {
            #[inline]
            fn clone(&self) -> TileJson {
                match *self {
                    TileJson {
                        scheme: ref __self_0_0,
                        tiles: ref __self_0_1,
                        minzoom: ref __self_0_2,
                        maxzoom: ref __self_0_3,
                        bounds: ref __self_0_4,
                        tile_size: ref __self_0_5,
                    } => TileJson {
                        scheme: ::std::clone::Clone::clone(&(*__self_0_0)),
                        tiles: ::std::clone::Clone::clone(&(*__self_0_1)),
                        minzoom: ::std::clone::Clone::clone(&(*__self_0_2)),
                        maxzoom: ::std::clone::Clone::clone(&(*__self_0_3)),
                        bounds: ::std::clone::Clone::clone(&(*__self_0_4)),
                        tile_size: ::std::clone::Clone::clone(&(*__self_0_5)),
                    },
                }
            }
        }
        pub struct SourceData {
            url: Option<String>,
            #[serde(flatten)]
            tilejson: TileJson,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for SourceData {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    SourceData {
                        url: ref __self_0_0,
                        tilejson: ref __self_0_1,
                    } => {
                        let mut debug_trait_builder = f.debug_struct("SourceData");
                        let _ = debug_trait_builder.field("url", &&(*__self_0_0));
                        let _ = debug_trait_builder.field("tilejson", &&(*__self_0_1));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _IMPL_DESERIALIZE_FOR_SourceData: () = {
            extern crate serde as _serde;
            #[allow(unused_macros)]
            macro_rules! try(( $ __expr : expr ) => {
                                 match $ __expr {
                                 _serde :: export :: Ok ( __val ) => __val ,
                                 _serde :: export :: Err ( __err ) => {
                                 return _serde :: export :: Err ( __err ) ; }
                                 } });
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for SourceData {
                fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field<'de> {
                        __field0,
                        __other(_serde::private::de::Content<'de>),
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field<'de>;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::export::Formatter,
                        ) -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_bool<__E>(
                            self,
                            __value: bool,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::export::Ok(__Field::__other(
                                _serde::private::de::Content::Bool(__value),
                            ))
                        }
                        fn visit_i8<__E>(
                            self,
                            __value: i8,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::export::Ok(__Field::__other(_serde::private::de::Content::I8(
                                __value,
                            )))
                        }
                        fn visit_i16<__E>(
                            self,
                            __value: i16,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::export::Ok(__Field::__other(_serde::private::de::Content::I16(
                                __value,
                            )))
                        }
                        fn visit_i32<__E>(
                            self,
                            __value: i32,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::export::Ok(__Field::__other(_serde::private::de::Content::I32(
                                __value,
                            )))
                        }
                        fn visit_i64<__E>(
                            self,
                            __value: i64,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::export::Ok(__Field::__other(_serde::private::de::Content::I64(
                                __value,
                            )))
                        }
                        fn visit_u8<__E>(
                            self,
                            __value: u8,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::export::Ok(__Field::__other(_serde::private::de::Content::U8(
                                __value,
                            )))
                        }
                        fn visit_u16<__E>(
                            self,
                            __value: u16,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::export::Ok(__Field::__other(_serde::private::de::Content::U16(
                                __value,
                            )))
                        }
                        fn visit_u32<__E>(
                            self,
                            __value: u32,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::export::Ok(__Field::__other(_serde::private::de::Content::U32(
                                __value,
                            )))
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::export::Ok(__Field::__other(_serde::private::de::Content::U64(
                                __value,
                            )))
                        }
                        fn visit_f32<__E>(
                            self,
                            __value: f32,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::export::Ok(__Field::__other(_serde::private::de::Content::F32(
                                __value,
                            )))
                        }
                        fn visit_f64<__E>(
                            self,
                            __value: f64,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::export::Ok(__Field::__other(_serde::private::de::Content::F64(
                                __value,
                            )))
                        }
                        fn visit_char<__E>(
                            self,
                            __value: char,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::export::Ok(__Field::__other(
                                _serde::private::de::Content::Char(__value),
                            ))
                        }
                        fn visit_unit<__E>(self) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::export::Ok(__Field::__other(_serde::private::de::Content::Unit))
                        }
                        fn visit_borrowed_str<__E>(
                            self,
                            __value: &'de str,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "url" => _serde::export::Ok(__Field::__field0),
                                _ => {
                                    let __value = _serde::private::de::Content::Str(__value);
                                    _serde::export::Ok(__Field::__other(__value))
                                }
                            }
                        }
                        fn visit_borrowed_bytes<__E>(
                            self,
                            __value: &'de [u8],
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"url" => _serde::export::Ok(__Field::__field0),
                                _ => {
                                    let __value = _serde::private::de::Content::Bytes(__value);
                                    _serde::export::Ok(__Field::__other(__value))
                                }
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "url" => _serde::export::Ok(__Field::__field0),
                                _ => {
                                    let __value =
                                        _serde::private::de::Content::String(__value.to_string());
                                    _serde::export::Ok(__Field::__other(__value))
                                }
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"url" => _serde::export::Ok(__Field::__field0),
                                _ => {
                                    let __value =
                                        _serde::private::de::Content::ByteBuf(__value.to_vec());
                                    _serde::export::Ok(__Field::__other(__value))
                                }
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field<'de> {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::export::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::export::PhantomData<SourceData>,
                        lifetime: _serde::export::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = SourceData;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::export::Formatter,
                        ) -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter, "struct SourceData")
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::export::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::export::Option<
                                Option<String>,
                            > = _serde::export::None;
                            let mut __collect = _serde::export::Vec::<
                                _serde::export::Option<(
                                    _serde::private::de::Content,
                                    _serde::private::de::Content,
                                )>,
                            >::new();
                            while let _serde::export::Some(__key) =
                                match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::export::Option::is_some(&__field0) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "url",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<Option<String>>(
                                                &mut __map,
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__other(__name) => {
                                        __collect.push(_serde::export::Some((
                                            __name,
                                            match _serde::de::MapAccess::next_value(&mut __map) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        )));
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::export::Some(__field0) => __field0,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("url") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1: TileJson = match _serde::de::Deserialize::deserialize(
                                _serde::private::de::FlatMapDeserializer(
                                    &mut __collect,
                                    _serde::export::PhantomData,
                                ),
                            ) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            };
                            _serde::export::Ok(SourceData {
                                url: __field0,
                                tilejson: __field1,
                            })
                        }
                    }
                    _serde::Deserializer::deserialize_map(
                        __deserializer,
                        __Visitor {
                            marker: _serde::export::PhantomData::<SourceData>,
                            lifetime: _serde::export::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for SourceData {
            #[inline]
            fn clone(&self) -> SourceData {
                match *self {
                    SourceData {
                        url: ref __self_0_0,
                        tilejson: ref __self_0_1,
                    } => SourceData {
                        url: ::std::clone::Clone::clone(&(*__self_0_0)),
                        tilejson: ::std::clone::Clone::clone(&(*__self_0_1)),
                    },
                }
            }
        }
        pub struct Style {
            pub version: i32,
            pub name: Option<String>,
            pub center: Option<[f64; 2]>,
            pub zoom: Option<f32>,
            pub sources: BTreeMap<String, StyleSource>,
            pub sprite: Option<String>,
            pub glyphs: Option<String>,
            pub layers: Vec<StyleLayer>,
        }
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _IMPL_DESERIALIZE_FOR_Style: () = {
            extern crate serde as _serde;
            #[allow(unused_macros)]
            macro_rules! try(( $ __expr : expr ) => {
                                 match $ __expr {
                                 _serde :: export :: Ok ( __val ) => __val ,
                                 _serde :: export :: Err ( __err ) => {
                                 return _serde :: export :: Err ( __err ) ; }
                                 } });
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Style {
                fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __field6,
                        __field7,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::export::Formatter,
                        ) -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::export::Ok(__Field::__field0),
                                1u64 => _serde::export::Ok(__Field::__field1),
                                2u64 => _serde::export::Ok(__Field::__field2),
                                3u64 => _serde::export::Ok(__Field::__field3),
                                4u64 => _serde::export::Ok(__Field::__field4),
                                5u64 => _serde::export::Ok(__Field::__field5),
                                6u64 => _serde::export::Ok(__Field::__field6),
                                7u64 => _serde::export::Ok(__Field::__field7),
                                _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"field index 0 <= i < 8",
                                )),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "version" => _serde::export::Ok(__Field::__field0),
                                "name" => _serde::export::Ok(__Field::__field1),
                                "center" => _serde::export::Ok(__Field::__field2),
                                "zoom" => _serde::export::Ok(__Field::__field3),
                                "sources" => _serde::export::Ok(__Field::__field4),
                                "sprite" => _serde::export::Ok(__Field::__field5),
                                "glyphs" => _serde::export::Ok(__Field::__field6),
                                "layers" => _serde::export::Ok(__Field::__field7),
                                _ => _serde::export::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"version" => _serde::export::Ok(__Field::__field0),
                                b"name" => _serde::export::Ok(__Field::__field1),
                                b"center" => _serde::export::Ok(__Field::__field2),
                                b"zoom" => _serde::export::Ok(__Field::__field3),
                                b"sources" => _serde::export::Ok(__Field::__field4),
                                b"sprite" => _serde::export::Ok(__Field::__field5),
                                b"glyphs" => _serde::export::Ok(__Field::__field6),
                                b"layers" => _serde::export::Ok(__Field::__field7),
                                _ => _serde::export::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::export::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::export::PhantomData<Style>,
                        lifetime: _serde::export::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Style;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::export::Formatter,
                        ) -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter, "struct Style")
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::export::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<i32>(
                                &mut __seq,
                            ) {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Style with 8 elements",
                                    ));
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Style with 8 elements",
                                    ));
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<
                                Option<[f64; 2]>,
                            >(&mut __seq)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct Style with 8 elements",
                                    ));
                                }
                            };
                            let __field3 = match match _serde::de::SeqAccess::next_element::<
                                Option<f32>,
                            >(&mut __seq)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct Style with 8 elements",
                                    ));
                                }
                            };
                            let __field4 = match match _serde::de::SeqAccess::next_element::<
                                BTreeMap<String, StyleSource>,
                            >(&mut __seq)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        4usize,
                                        &"struct Style with 8 elements",
                                    ));
                                }
                            };
                            let __field5 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        5usize,
                                        &"struct Style with 8 elements",
                                    ));
                                }
                            };
                            let __field6 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        6usize,
                                        &"struct Style with 8 elements",
                                    ));
                                }
                            };
                            let __field7 = match match _serde::de::SeqAccess::next_element::<
                                Vec<StyleLayer>,
                            >(&mut __seq)
                            {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            } {
                                _serde::export::Some(__value) => __value,
                                _serde::export::None => {
                                    return _serde::export::Err(_serde::de::Error::invalid_length(
                                        7usize,
                                        &"struct Style with 8 elements",
                                    ));
                                }
                            };
                            _serde::export::Ok(Style {
                                version: __field0,
                                name: __field1,
                                center: __field2,
                                zoom: __field3,
                                sources: __field4,
                                sprite: __field5,
                                glyphs: __field6,
                                layers: __field7,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::export::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::export::Option<
                                i32,
                            > = _serde::export::None;
                            let mut __field1: _serde::export::Option<
                                Option<String>,
                            > = _serde::export::None;
                            let mut __field2: _serde::export::Option<
                                Option<[f64; 2]>,
                            > = _serde::export::None;
                            let mut __field3: _serde::export::Option<
                                Option<f32>,
                            > = _serde::export::None;
                            let mut __field4: _serde::export::Option<
                                BTreeMap<String, StyleSource>,
                            > = _serde::export::None;
                            let mut __field5: _serde::export::Option<
                                Option<String>,
                            > = _serde::export::None;
                            let mut __field6: _serde::export::Option<
                                Option<String>,
                            > = _serde::export::None;
                            let mut __field7: _serde::export::Option<
                                Vec<StyleLayer>,
                            > = _serde::export::None;
                            while let _serde::export::Some(__key) =
                                match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::export::Option::is_some(&__field0) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "version",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<i32>(
                                                &mut __map,
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::export::Option::is_some(&__field1) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "name",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<Option<String>>(
                                                &mut __map,
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::export::Option::is_some(&__field2) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "center",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<[f64; 2]>,
                                            >(
                                                &mut __map
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::export::Option::is_some(&__field3) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "zoom",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<Option<f32>>(
                                                &mut __map,
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::export::Option::is_some(&__field4) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "sources",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                BTreeMap<String, StyleSource>,
                                            >(
                                                &mut __map
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field5 => {
                                        if _serde::export::Option::is_some(&__field5) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "sprite",
                                                ),
                                            );
                                        }
                                        __field5 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<Option<String>>(
                                                &mut __map,
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field6 => {
                                        if _serde::export::Option::is_some(&__field6) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "glyphs",
                                                ),
                                            );
                                        }
                                        __field6 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<Option<String>>(
                                                &mut __map,
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field7 => {
                                        if _serde::export::Option::is_some(&__field7) {
                                            return _serde::export::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "layers",
                                                ),
                                            );
                                        }
                                        __field7 = _serde::export::Some(
                                            match _serde::de::MapAccess::next_value::<Vec<StyleLayer>>(
                                                &mut __map,
                                            ) {
                                                _serde::export::Ok(__val) => __val,
                                                _serde::export::Err(__err) => {
                                                    return _serde::export::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(
                                            &mut __map
                                        ) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::export::Some(__field0) => __field0,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("version") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::export::Some(__field1) => __field1,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("name") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::export::Some(__field2) => __field2,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("center") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::export::Some(__field3) => __field3,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("zoom") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::export::Some(__field4) => __field4,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("sources") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field5 = match __field5 {
                                _serde::export::Some(__field5) => __field5,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("sprite") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field6 = match __field6 {
                                _serde::export::Some(__field6) => __field6,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("glyphs") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field7 = match __field7 {
                                _serde::export::Some(__field7) => __field7,
                                _serde::export::None => {
                                    match _serde::private::de::missing_field("layers") {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::export::Ok(Style {
                                version: __field0,
                                name: __field1,
                                center: __field2,
                                zoom: __field3,
                                sources: __field4,
                                sprite: __field5,
                                glyphs: __field6,
                                layers: __field7,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[
                        "version", "name", "center", "zoom", "sources", "sprite", "glyphs",
                        "layers",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Style",
                        FIELDS,
                        __Visitor {
                            marker: _serde::export::PhantomData::<Style>,
                            lifetime: _serde::export::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for Style {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    Style {
                        version: ref __self_0_0,
                        name: ref __self_0_1,
                        center: ref __self_0_2,
                        zoom: ref __self_0_3,
                        sources: ref __self_0_4,
                        sprite: ref __self_0_5,
                        glyphs: ref __self_0_6,
                        layers: ref __self_0_7,
                    } => {
                        let mut debug_trait_builder = f.debug_struct("Style");
                        let _ = debug_trait_builder.field("version", &&(*__self_0_0));
                        let _ = debug_trait_builder.field("name", &&(*__self_0_1));
                        let _ = debug_trait_builder.field("center", &&(*__self_0_2));
                        let _ = debug_trait_builder.field("zoom", &&(*__self_0_3));
                        let _ = debug_trait_builder.field("sources", &&(*__self_0_4));
                        let _ = debug_trait_builder.field("sprite", &&(*__self_0_5));
                        let _ = debug_trait_builder.field("glyphs", &&(*__self_0_6));
                        let _ = debug_trait_builder.field("layers", &&(*__self_0_7));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for Style {
            #[inline]
            fn clone(&self) -> Style {
                match *self {
                    Style {
                        version: ref __self_0_0,
                        name: ref __self_0_1,
                        center: ref __self_0_2,
                        zoom: ref __self_0_3,
                        sources: ref __self_0_4,
                        sprite: ref __self_0_5,
                        glyphs: ref __self_0_6,
                        layers: ref __self_0_7,
                    } => Style {
                        version: ::std::clone::Clone::clone(&(*__self_0_0)),
                        name: ::std::clone::Clone::clone(&(*__self_0_1)),
                        center: ::std::clone::Clone::clone(&(*__self_0_2)),
                        zoom: ::std::clone::Clone::clone(&(*__self_0_3)),
                        sources: ::std::clone::Clone::clone(&(*__self_0_4)),
                        sprite: ::std::clone::Clone::clone(&(*__self_0_5)),
                        glyphs: ::std::clone::Clone::clone(&(*__self_0_6)),
                        layers: ::std::clone::Clone::clone(&(*__self_0_7)),
                    },
                }
            }
        }
        #[serde(tag = "type")]
        pub enum StyleSource {
            #[serde(rename = "vector")]
            Vector(SourceData),

            #[serde(rename = "raster")]
            Raster(SourceData),

            #[serde(rename = "image")]
            Image(SourceData),
        }
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _IMPL_DESERIALIZE_FOR_StyleSource: () = {
            extern crate serde as _serde;
            #[allow(unused_macros)]
            macro_rules! try(( $ __expr : expr ) => {
                                 match $ __expr {
                                 _serde :: export :: Ok ( __val ) => __val ,
                                 _serde :: export :: Err ( __err ) => {
                                 return _serde :: export :: Err ( __err ) ; }
                                 } });
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for StyleSource {
                fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::export::Formatter,
                        ) -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter, "variant identifier")
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::export::Ok(__Field::__field0),
                                1u64 => _serde::export::Ok(__Field::__field1),
                                2u64 => _serde::export::Ok(__Field::__field2),
                                _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"variant index 0 <= i < 3",
                                )),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "vector" => _serde::export::Ok(__Field::__field0),
                                "raster" => _serde::export::Ok(__Field::__field1),
                                "image" => _serde::export::Ok(__Field::__field2),
                                _ => _serde::export::Err(_serde::de::Error::unknown_variant(
                                    __value, VARIANTS,
                                )),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"vector" => _serde::export::Ok(__Field::__field0),
                                b"raster" => _serde::export::Ok(__Field::__field1),
                                b"image" => _serde::export::Ok(__Field::__field2),
                                _ => {
                                    let __value = &_serde::export::from_utf8_lossy(__value);
                                    _serde::export::Err(_serde::de::Error::unknown_variant(
                                        __value, VARIANTS,
                                    ))
                                }
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::export::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    const VARIANTS: &'static [&'static str] = &["vector", "raster", "image"];
                    let __tagged = match _serde::Deserializer::deserialize_any(
                        __deserializer,
                        _serde::private::de::TaggedContentVisitor::<__Field>::new("type"),
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match __tagged.tag {
                        __Field::__field0 => _serde::export::Result::map(
                            <SourceData as _serde::Deserialize>::deserialize(
                                _serde::private::de::ContentDeserializer::<__D::Error>::new(
                                    __tagged.content,
                                ),
                            ),
                            StyleSource::Vector,
                        ),
                        __Field::__field1 => _serde::export::Result::map(
                            <SourceData as _serde::Deserialize>::deserialize(
                                _serde::private::de::ContentDeserializer::<__D::Error>::new(
                                    __tagged.content,
                                ),
                            ),
                            StyleSource::Raster,
                        ),
                        __Field::__field2 => _serde::export::Result::map(
                            <SourceData as _serde::Deserialize>::deserialize(
                                _serde::private::de::ContentDeserializer::<__D::Error>::new(
                                    __tagged.content,
                                ),
                            ),
                            StyleSource::Image,
                        ),
                    }
                }
            }
        };
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for StyleSource {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match (&*self,) {
                    (&StyleSource::Vector(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("Vector");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&StyleSource::Raster(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("Raster");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&StyleSource::Image(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("Image");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for StyleSource {
            #[inline]
            fn clone(&self) -> StyleSource {
                match (&*self,) {
                    (&StyleSource::Vector(ref __self_0),) => {
                        StyleSource::Vector(::std::clone::Clone::clone(&(*__self_0)))
                    }
                    (&StyleSource::Raster(ref __self_0),) => {
                        StyleSource::Raster(::std::clone::Clone::clone(&(*__self_0)))
                    }
                    (&StyleSource::Image(ref __self_0),) => {
                        StyleSource::Image(::std::clone::Clone::clone(&(*__self_0)))
                    }
                }
            }
        }
        #[serde(tag = "type")]
        pub enum StyleLayer {
            #[serde(rename = "background")]
            Background(BackgroundLayer),

            #[serde(rename = "fill")]
            Fill(FillLayer),

            #[serde(rename = "line")]
            Line(LineLayer),

            #[serde(rename = "symbol")]
            Symbols(SymbolLayer),

            #[serde(rename = "raster")]
            Raster(RasterLayer),
        }
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _IMPL_DESERIALIZE_FOR_StyleLayer: () = {
            extern crate serde as _serde;
            #[allow(unused_macros)]
            macro_rules! try(( $ __expr : expr ) => {
                                 match $ __expr {
                                 _serde :: export :: Ok ( __val ) => __val ,
                                 _serde :: export :: Err ( __err ) => {
                                 return _serde :: export :: Err ( __err ) ; }
                                 } });
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for StyleLayer {
                fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::export::Formatter,
                        ) -> _serde::export::fmt::Result {
                            _serde::export::Formatter::write_str(__formatter, "variant identifier")
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::export::Ok(__Field::__field0),
                                1u64 => _serde::export::Ok(__Field::__field1),
                                2u64 => _serde::export::Ok(__Field::__field2),
                                3u64 => _serde::export::Ok(__Field::__field3),
                                4u64 => _serde::export::Ok(__Field::__field4),
                                _ => _serde::export::Err(_serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"variant index 0 <= i < 5",
                                )),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "background" => _serde::export::Ok(__Field::__field0),
                                "fill" => _serde::export::Ok(__Field::__field1),
                                "line" => _serde::export::Ok(__Field::__field2),
                                "symbol" => _serde::export::Ok(__Field::__field3),
                                "raster" => _serde::export::Ok(__Field::__field4),
                                _ => _serde::export::Err(_serde::de::Error::unknown_variant(
                                    __value, VARIANTS,
                                )),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::export::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"background" => _serde::export::Ok(__Field::__field0),
                                b"fill" => _serde::export::Ok(__Field::__field1),
                                b"line" => _serde::export::Ok(__Field::__field2),
                                b"symbol" => _serde::export::Ok(__Field::__field3),
                                b"raster" => _serde::export::Ok(__Field::__field4),
                                _ => {
                                    let __value = &_serde::export::from_utf8_lossy(__value);
                                    _serde::export::Err(_serde::de::Error::unknown_variant(
                                        __value, VARIANTS,
                                    ))
                                }
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::export::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    const VARIANTS: &'static [&'static str] =
                        &["background", "fill", "line", "symbol", "raster"];
                    let __tagged = match _serde::Deserializer::deserialize_any(
                        __deserializer,
                        _serde::private::de::TaggedContentVisitor::<__Field>::new("type"),
                    ) {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    };
                    match __tagged.tag {
                        __Field::__field0 => _serde::export::Result::map(
                            <BackgroundLayer as _serde::Deserialize>::deserialize(
                                _serde::private::de::ContentDeserializer::<__D::Error>::new(
                                    __tagged.content,
                                ),
                            ),
                            StyleLayer::Background,
                        ),
                        __Field::__field1 => _serde::export::Result::map(
                            <FillLayer as _serde::Deserialize>::deserialize(
                                _serde::private::de::ContentDeserializer::<__D::Error>::new(
                                    __tagged.content,
                                ),
                            ),
                            StyleLayer::Fill,
                        ),
                        __Field::__field2 => _serde::export::Result::map(
                            <LineLayer as _serde::Deserialize>::deserialize(
                                _serde::private::de::ContentDeserializer::<__D::Error>::new(
                                    __tagged.content,
                                ),
                            ),
                            StyleLayer::Line,
                        ),
                        __Field::__field3 => _serde::export::Result::map(
                            <SymbolLayer as _serde::Deserialize>::deserialize(
                                _serde::private::de::ContentDeserializer::<__D::Error>::new(
                                    __tagged.content,
                                ),
                            ),
                            StyleLayer::Symbols,
                        ),
                        __Field::__field4 => _serde::export::Result::map(
                            <RasterLayer as _serde::Deserialize>::deserialize(
                                _serde::private::de::ContentDeserializer::<__D::Error>::new(
                                    __tagged.content,
                                ),
                            ),
                            StyleLayer::Raster,
                        ),
                    }
                }
            }
        };
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::fmt::Debug for StyleLayer {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match (&*self,) {
                    (&StyleLayer::Background(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("Background");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&StyleLayer::Fill(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("Fill");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&StyleLayer::Line(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("Line");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&StyleLayer::Symbols(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("Symbols");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                    (&StyleLayer::Raster(ref __self_0),) => {
                        let mut debug_trait_builder = f.debug_tuple("Raster");
                        let _ = debug_trait_builder.field(&&(*__self_0));
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for StyleLayer {
            #[inline]
            fn clone(&self) -> StyleLayer {
                match (&*self,) {
                    (&StyleLayer::Background(ref __self_0),) => {
                        StyleLayer::Background(::std::clone::Clone::clone(&(*__self_0)))
                    }
                    (&StyleLayer::Fill(ref __self_0),) => {
                        StyleLayer::Fill(::std::clone::Clone::clone(&(*__self_0)))
                    }
                    (&StyleLayer::Line(ref __self_0),) => {
                        StyleLayer::Line(::std::clone::Clone::clone(&(*__self_0)))
                    }
                    (&StyleLayer::Symbols(ref __self_0),) => {
                        StyleLayer::Symbols(::std::clone::Clone::clone(&(*__self_0)))
                    }
                    (&StyleLayer::Raster(ref __self_0),) => {
                        StyleLayer::Raster(::std::clone::Clone::clone(&(*__self_0)))
                    }
                }
            }
        }
    }
    pub mod storage {
        use prelude::*;
        pub mod resource {
            use prelude::*;
            #[rustc_copy_clone_marker]
            pub struct TileCoords {
                x: i32,
                y: i32,
                z: i32,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::fmt::Debug for TileCoords {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        TileCoords {
                            x: ref __self_0_0,
                            y: ref __self_0_1,
                            z: ref __self_0_2,
                        } => {
                            let mut debug_trait_builder = f.debug_struct("TileCoords");
                            let _ = debug_trait_builder.field("x", &&(*__self_0_0));
                            let _ = debug_trait_builder.field("y", &&(*__self_0_1));
                            let _ = debug_trait_builder.field("z", &&(*__self_0_2));
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::clone::Clone for TileCoords {
                #[inline]
                fn clone(&self) -> TileCoords {
                    {
                        let _: ::std::clone::AssertParamIsClone<i32>;
                        let _: ::std::clone::AssertParamIsClone<i32>;
                        let _: ::std::clone::AssertParamIsClone<i32>;
                        *self
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::marker::Copy for TileCoords {}
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::hash::Hash for TileCoords {
                fn hash<__H: ::std::hash::Hasher>(&self, state: &mut __H) -> () {
                    match *self {
                        TileCoords {
                            x: ref __self_0_0,
                            y: ref __self_0_1,
                            z: ref __self_0_2,
                        } => {
                            ::std::hash::Hash::hash(&(*__self_0_0), state);
                            ::std::hash::Hash::hash(&(*__self_0_1), state);
                            ::std::hash::Hash::hash(&(*__self_0_2), state)
                        }
                    }
                }
            }
            pub enum LoadPreference {
                None,
                Cache,
                Network,
                CacheOnly,
                NetworkOnly,
                Any,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::fmt::Debug for LoadPreference {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match (&*self,) {
                        (&LoadPreference::None,) => {
                            let mut debug_trait_builder = f.debug_tuple("None");
                            debug_trait_builder.finish()
                        }
                        (&LoadPreference::Cache,) => {
                            let mut debug_trait_builder = f.debug_tuple("Cache");
                            debug_trait_builder.finish()
                        }
                        (&LoadPreference::Network,) => {
                            let mut debug_trait_builder = f.debug_tuple("Network");
                            debug_trait_builder.finish()
                        }
                        (&LoadPreference::CacheOnly,) => {
                            let mut debug_trait_builder = f.debug_tuple("CacheOnly");
                            debug_trait_builder.finish()
                        }
                        (&LoadPreference::NetworkOnly,) => {
                            let mut debug_trait_builder = f.debug_tuple("NetworkOnly");
                            debug_trait_builder.finish()
                        }
                        (&LoadPreference::Any,) => {
                            let mut debug_trait_builder = f.debug_tuple("Any");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::clone::Clone for LoadPreference {
                #[inline]
                fn clone(&self) -> LoadPreference {
                    match (&*self,) {
                        (&LoadPreference::None,) => LoadPreference::None,
                        (&LoadPreference::Cache,) => LoadPreference::Cache,
                        (&LoadPreference::Network,) => LoadPreference::Network,
                        (&LoadPreference::CacheOnly,) => LoadPreference::CacheOnly,
                        (&LoadPreference::NetworkOnly,) => LoadPreference::NetworkOnly,
                        (&LoadPreference::Any,) => LoadPreference::Any,
                    }
                }
            }
            pub enum ResourceData {
                Tile {
                    template: String,
                    ratio: i32,
                    coords: TileCoords,
                },
                StyleJson {
                    url: String,
                },
                SourceJson {
                    url: String,
                },
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::fmt::Debug for ResourceData {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match (&*self,) {
                        (&ResourceData::Tile {
                            template: ref __self_0,
                            ratio: ref __self_1,
                            coords: ref __self_2,
                        },) => {
                            let mut debug_trait_builder = f.debug_struct("Tile");
                            let _ = debug_trait_builder.field("template", &&(*__self_0));
                            let _ = debug_trait_builder.field("ratio", &&(*__self_1));
                            let _ = debug_trait_builder.field("coords", &&(*__self_2));
                            debug_trait_builder.finish()
                        }
                        (&ResourceData::StyleJson { url: ref __self_0 },) => {
                            let mut debug_trait_builder = f.debug_struct("StyleJson");
                            let _ = debug_trait_builder.field("url", &&(*__self_0));
                            debug_trait_builder.finish()
                        }
                        (&ResourceData::SourceJson { url: ref __self_0 },) => {
                            let mut debug_trait_builder = f.debug_struct("SourceJson");
                            let _ = debug_trait_builder.field("url", &&(*__self_0));
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::clone::Clone for ResourceData {
                #[inline]
                fn clone(&self) -> ResourceData {
                    match (&*self,) {
                        (&ResourceData::Tile {
                            template: ref __self_0,
                            ratio: ref __self_1,
                            coords: ref __self_2,
                        },) => ResourceData::Tile {
                            template: ::std::clone::Clone::clone(&(*__self_0)),
                            ratio: ::std::clone::Clone::clone(&(*__self_1)),
                            coords: ::std::clone::Clone::clone(&(*__self_2)),
                        },
                        (&ResourceData::StyleJson { url: ref __self_0 },) => {
                            ResourceData::StyleJson {
                                url: ::std::clone::Clone::clone(&(*__self_0)),
                            }
                        }
                        (&ResourceData::SourceJson { url: ref __self_0 },) => {
                            ResourceData::SourceJson {
                                url: ::std::clone::Clone::clone(&(*__self_0)),
                            }
                        }
                    }
                }
            }
            pub struct Resource {
                pub load_pref: LoadPreference,
                pub data: ResourceData,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::fmt::Debug for Resource {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        Resource {
                            load_pref: ref __self_0_0,
                            data: ref __self_0_1,
                        } => {
                            let mut debug_trait_builder = f.debug_struct("Resource");
                            let _ = debug_trait_builder.field("load_pref", &&(*__self_0_0));
                            let _ = debug_trait_builder.field("data", &&(*__self_0_1));
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::clone::Clone for Resource {
                #[inline]
                fn clone(&self) -> Resource {
                    match *self {
                        Resource {
                            load_pref: ref __self_0_0,
                            data: ref __self_0_1,
                        } => Resource {
                            load_pref: ::std::clone::Clone::clone(&(*__self_0_0)),
                            data: ::std::clone::Clone::clone(&(*__self_0_1)),
                        },
                    }
                }
            }
            impl Resource {
                pub fn url<'a>(&'a self) -> &'a str {
                    return match &self.data {
                        ResourceData::StyleJson { ref url } => &url,
                        ResourceData::SourceJson { ref url } => &url,
                        _ => {
                            {
                                ::rt::begin_panic(
                                    "explicit panic",
                                    &("rmaps/src/map/storage/resource.rs", 47u32, 17u32),
                                )
                            }
                        }
                    };
                }
                pub fn style(url: String) -> Resource {
                    Resource {
                        load_pref: LoadPreference::Any,
                        data: ResourceData::StyleJson { url: url },
                    }
                }
                pub fn source(url: String) -> Resource {
                    Resource {
                        load_pref: LoadPreference::Any,
                        data: ResourceData::SourceJson { url: url },
                    }
                }
            }
        }
        pub mod response {
            use prelude::*;
            pub struct Response {
                pub resource: super::resource::Resource,
                pub data: Vec<u8>,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::fmt::Debug for Response {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match *self {
                        Response {
                            resource: ref __self_0_0,
                            data: ref __self_0_1,
                        } => {
                            let mut debug_trait_builder = f.debug_struct("Response");
                            let _ = debug_trait_builder.field("resource", &&(*__self_0_0));
                            let _ = debug_trait_builder.field("data", &&(*__self_0_1));
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
        }
        pub mod local {
            use super::*;
            use prelude::*;
            use std::io::Read;
            pub struct LocalFileSource {}
            impl ::actix::Actor for LocalFileSource {
                type Context = ::actix::Context<Self>;
            }
            pub struct LocalFileSourceAddr {
                pub addr: ::actix::Addr<Syn, LocalFileSource>,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::clone::Clone for LocalFileSourceAddr {
                #[inline]
                fn clone(&self) -> LocalFileSourceAddr {
                    match *self {
                        LocalFileSourceAddr {
                            addr: ref __self_0_0,
                        } => LocalFileSourceAddr {
                            addr: ::std::clone::Clone::clone(&(*__self_0_0)),
                        },
                    }
                }
            }
            impl LocalFileSource {
                pub fn new() -> LocalFileSource {
                    LocalFileSource {}
                }
                pub fn spawn() -> LocalFileSourceAddr {
                    LocalFileSourceAddr {
                        addr: start_in_thread(|| Self::new()),
                    }
                }
            }
            impl FileSource for LocalFileSource {
                fn can_handle(&self, res: Resource) -> bool {
                    ::io::_print(::std::fmt::Arguments::new_v1_formatted(
                        &["Local can handle ", "\n"],
                        &match (&res.url(),) {
                            (arg0,) => [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Debug::fmt)],
                        },
                        &[::std::fmt::rt::v1::Argument {
                            position: ::std::fmt::rt::v1::Position::At(0usize),
                            format: ::std::fmt::rt::v1::FormatSpec {
                                fill: ' ',
                                align: ::std::fmt::rt::v1::Alignment::Unknown,
                                flags: 0u32,
                                precision: ::std::fmt::rt::v1::Count::Implied,
                                width: ::std::fmt::rt::v1::Count::Implied,
                            },
                        }],
                    ));
                    return res.url().starts_with("local://") || res.url().starts_with("file://");
                }
                fn get(&mut self, res: Resource) -> ResponseFuture<ResResponse, Error> {
                    Box::new({
                        ::rt::begin_panic(
                            "not yet implemented",
                            &("rmaps/src/map/storage/local.rs", 41u32, 1u32),
                        )
                    })
                }
            }
            impl FileSourceAddr for LocalFileSourceAddr {
                fn can_handle(
                    &self,
                    res: Resource,
                ) -> Box<Future<Item = bool, Error = ::actix::MailboxError>> {
                    let data = Msg_FileSource_can_handle(res);
                    Box::new(self.addr.send(data))
                }
                fn can_handle_async(&self, res: Resource) {
                    let data = Msg_FileSource_can_handle(res);
                    self.addr.do_send(data)
                }
                fn get(
                    &self,
                    res: Resource,
                ) -> Box<
                    Future<
                        Item = ResponseFuture<ResResponse, Error>,
                        Error = ::actix::MailboxError,
                    >,
                > {
                    let data = Msg_FileSource_get(res);
                    Box::new(self.addr.send(data))
                }
                fn get_async(&self, res: Resource) {
                    let data = Msg_FileSource_get(res);
                    self.addr.do_send(data)
                }
            }
            impl ::actix::Handler<Msg_FileSource_can_handle> for LocalFileSource {
                type Result = bool;
                fn handle(
                    &mut self,
                    msg: Msg_FileSource_can_handle,
                    ctx: &mut Self::Context,
                ) -> bool {
                    self.can_handle(msg.0)
                }
            }
            impl ::actix::Handler<Msg_FileSource_get> for LocalFileSource {
                type Result = ResponseFuture<ResResponse, Error>;
                fn handle(
                    &mut self,
                    msg: Msg_FileSource_get,
                    ctx: &mut Self::Context,
                ) -> ResponseFuture<ResResponse, Error> {
                    self.get(msg.0)
                }
            }
        }
        pub mod network {
            use super::*;
            use actix_web::client;
            use prelude::*;
            pub struct NetworkFileSource {}
            impl ::actix::Actor for NetworkFileSource {
                type Context = ::actix::Context<Self>;
            }
            pub struct NetworkFileSourceAddr {
                pub addr: ::actix::Addr<Syn, NetworkFileSource>,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::std::clone::Clone for NetworkFileSourceAddr {
                #[inline]
                fn clone(&self) -> NetworkFileSourceAddr {
                    match *self {
                        NetworkFileSourceAddr {
                            addr: ref __self_0_0,
                        } => NetworkFileSourceAddr {
                            addr: ::std::clone::Clone::clone(&(*__self_0_0)),
                        },
                    }
                }
            }
            impl FileSource for NetworkFileSource {
                fn can_handle(&self, res: Resource) -> bool {
                    let url = res.url();
                    return url.starts_with("http://") || url.starts_with("https://");
                }
                fn get(
                    &mut self,
                    res: Resource,
                ) -> Box<Future<Item = ResResponse, Error = ::common::prelude::Error>>
                {
                    let fut = client::get(res.url().clone())
                        .finish()
                        .unwrap()
                        .send()
                        .map_err(|x| {
                            {
                                ::rt::begin_panic_fmt(
                                    &::std::fmt::Arguments::new_v1_formatted(
                                        &["Retrieval failed "],
                                        &match (&x,) {
                                            (arg0,) => [::std::fmt::ArgumentV1::new(
                                                arg0,
                                                ::std::fmt::Display::fmt,
                                            )],
                                        },
                                        &[::std::fmt::rt::v1::Argument {
                                            position: ::std::fmt::rt::v1::Position::At(0usize),
                                            format: ::std::fmt::rt::v1::FormatSpec {
                                                fill: ' ',
                                                align: ::std::fmt::rt::v1::Alignment::Unknown,
                                                flags: 0u32,
                                                precision: ::std::fmt::rt::v1::Count::Implied,
                                                width: ::std::fmt::rt::v1::Count::Implied,
                                            },
                                        }],
                                    ),
                                    &("rmaps/src/map/storage/network.rs", 10u32, 1u32),
                                )
                            };
                            ()
                        })
                        .map(move |data| {
                            ::io::_print(::std::fmt::Arguments::new_v1_formatted(
                                &["Response: ", "\n"],
                                &match (&data,) {
                                    (arg0,) => {
                                        [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Debug::fmt)]
                                    }
                                },
                                &[::std::fmt::rt::v1::Argument {
                                    position: ::std::fmt::rt::v1::Position::At(0usize),
                                    format: ::std::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::std::fmt::rt::v1::Alignment::Unknown,
                                        flags: 0u32,
                                        precision: ::std::fmt::rt::v1::Count::Implied,
                                        width: ::std::fmt::rt::v1::Count::Implied,
                                    },
                                }],
                            ));
                            super::ResResponse {
                                resource: res,
                                data: <[_]>::into_vec(box []),
                            }
                        });
                    Box::new(fut)
                }
            }
            impl FileSourceAddr for NetworkFileSourceAddr {
                fn can_handle(
                    &self,
                    res: Resource,
                ) -> Box<Future<Item = bool, Error = ::actix::MailboxError>> {
                    let data = Msg_FileSource_can_handle(res);
                    Box::new(self.addr.send(data))
                }
                fn can_handle_async(&self, res: Resource) {
                    let data = Msg_FileSource_can_handle(res);
                    self.addr.do_send(data)
                }
                fn get(
                    &self,
                    res: Resource,
                ) -> Box<
                    Future<
                        Item = Box<Future<Item = ResResponse, Error = ::common::prelude::Error>>,
                        Error = ::actix::MailboxError,
                    >,
                > {
                    let data = Msg_FileSource_get(res);
                    Box::new(self.addr.send(data))
                }
                fn get_async(&self, res: Resource) {
                    let data = Msg_FileSource_get(res);
                    self.addr.do_send(data)
                }
            }
            impl ::actix::Handler<Msg_FileSource_can_handle> for NetworkFileSource {
                type Result = bool;
                fn handle(
                    &mut self,
                    msg: Msg_FileSource_can_handle,
                    ctx: &mut Self::Context,
                ) -> bool {
                    self.can_handle(msg.0)
                }
            }
            impl ::actix::Handler<Msg_FileSource_get> for NetworkFileSource {
                type Result = Box<Future<Item = ResResponse, Error = ::common::prelude::Error>>;
                fn handle(
                    &mut self,
                    msg: Msg_FileSource_get,
                    ctx: &mut Self::Context,
                ) -> Box<Future<Item = ResResponse, Error = ::common::prelude::Error>>
                {
                    self.get(msg.0)
                }
            }
            impl NetworkFileSource {
                fn new() -> Self {
                    return NetworkFileSource {};
                }
                pub fn spawn() -> NetworkFileSourceAddr {
                    NetworkFileSourceAddr {
                        addr: start_in_thread(|| NetworkFileSource::new()),
                    }
                }
            }
        }
        pub use self::resource::*;
        pub use self::response::Response as ResResponse;
        pub trait FileSource {
            fn can_handle(&self, res: Resource) -> bool;
            fn get(&mut self, res: Resource) -> actix::ResponseFuture<ResResponse, Error>;
        }
        pub trait FileSourceAddr {
            fn can_handle(
                &self,
                res: Resource,
            ) -> Box<Future<Item = bool, Error = ::actix::MailboxError>>;
            fn can_handle_async(&self, res: Resource);
            fn get(
                &self,
                res: Resource,
            ) -> Box<
                Future<
                    Item = actix::ResponseFuture<ResResponse, Error>,
                    Error = ::actix::MailboxError,
                >,
            >;
            fn get_async(&self, res: Resource);
        }
        pub struct Msg_FileSource_can_handle(pub Resource);
        pub struct Msg_FileSource_get(pub Resource);
        impl ::actix::Message for Msg_FileSource_can_handle {
            type Result = bool;
        }
        impl ::actix::Message for Msg_FileSource_get {
            type Result = actix::ResponseFuture<ResResponse, Error>;
        }
        pub struct DefaultFileSource {
            sources: Vec<Box<FileSourceAddr + Send + 'static>>,
        }
        impl ::actix::Actor for DefaultFileSource {
            type Context = ::actix::Context<Self>;
        }
        pub struct DefaultFileSourceAddr {
            pub addr: ::actix::Addr<Syn, DefaultFileSource>,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::std::clone::Clone for DefaultFileSourceAddr {
            #[inline]
            fn clone(&self) -> DefaultFileSourceAddr {
                match *self {
                    DefaultFileSourceAddr {
                        addr: ref __self_0_0,
                    } => DefaultFileSourceAddr {
                        addr: ::std::clone::Clone::clone(&(*__self_0_0)),
                    },
                }
            }
        }
        impl DefaultFileSource {
            fn get(&mut self, res: Resource) -> Box<Future<Item = ResResponse, Error = Error>> {
                for a in self.sources.iter() {
                    ::io::_print(::std::fmt::Arguments::new_v1(
                        &["Checking URL compatibility\n"],
                        &match () {
                            () => [],
                        },
                    ));
                    if a.can_handle(res.clone()).wait().unwrap() {
                        return a.get(res);
                    }
                }
                {
                    ::rt::begin_panic(
                        "not yet implemented",
                        &("rmaps/src/map/storage/mod.rs", 23u32, 1u32),
                    )
                }
            }
        }
        pub struct Msg_DefaultFileSource_get(pub Resource);
        impl ::actix::Message for Msg_DefaultFileSource_get {
            type Result = Box<Future<Item = ResResponse, Error = Error>>;
        }
        impl DefaultFileSourceAddr {
            pub fn get(
                &self,
                res: Resource,
            ) -> Box<
                Future<
                    Item = Box<Future<Item = ResResponse, Error = Error>>,
                    Error = ::actix::MailboxError,
                >,
            > {
                let data = Msg_DefaultFileSource_get(res);
                Box::new(self.addr.send(data))
            }
            pub fn get_async(&self, res: Resource) {
                let data = Msg_DefaultFileSource_get(res);
                self.addr.do_send(data)
            }
        }
        impl ::actix::Handler<Msg_DefaultFileSource_get> for DefaultFileSource {
            type Result = Box<Future<Item = ResResponse, Error = Error>>;
            fn handle(
                &mut self,
                msg: Msg_DefaultFileSource_get,
                ctx: &mut Self::Context,
            ) -> Box<Future<Item = ResResponse, Error = Error>> {
                self.get(msg.0)
            }
        }
        impl DefaultFileSource {
            pub fn new() -> Self {
                DefaultFileSource {
                    sources: <[_]>::into_vec(box [
                        Box::new(local::LocalFileSource::spawn()),
                        Box::new(network::NetworkFileSource::spawn()),
                    ]),
                }
            }
            pub fn spawn() -> DefaultFileSourceAddr {
                DefaultFileSourceAddr {
                    addr: start_in_thread::<DefaultFileSource, _>(|| DefaultFileSource::new()),
                }
            }
        }
    }
    use map::layers::Layer;
    pub struct MapView {
        addr: Addr<Unsync, MapViewImpl>,
        sys: SystemRunner,
    }
    impl MapView {
        pub fn new<F: glium::backend::Facade + Clone + 'static>(f: &F) -> Self {
            let mut sys = System::new("Map");
            let _impl = MapViewImpl::new(f);
            let addr = _impl.start();
            return MapView { sys, addr };
        }
        pub fn do_run<R>(&mut self, f: impl FnOnce(Addr<Unsync, MapViewImpl>) -> R) -> R {
            let addr = self.addr.clone();
            let res = self
                .sys
                .run_until_complete(::common::futures::future::lazy(|| Ok::<R, !>(f(addr))));
            self.sys.pulse();
            res.unwrap()
        }
        pub fn render(&mut self, surface: glium::Frame) {
            self.do_run(|add| {
                add.do_send(MapMethodArgs::Render(surface));
            });
        }
        pub fn set_style_url(&mut self, url: &str) {
            self.do_run(|add| {
                add.do_send(MapMethodArgs::SetStyleUrl(url.into()));
            });
        }
    }
    pub struct MapViewImpl {
        addr: Option<MapViewImplAddr>,
        facade: Box<glium::backend::Facade>,
        style: Option<style::Style>,
        layers: Vec<layers::LayerHolder>,
        source: storage::DefaultFileSourceAddr,
    }
    pub struct MapViewImplAddr {
        pub addr: ::actix::Addr<Syn, MapViewImpl>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for MapViewImplAddr {
        #[inline]
        fn clone(&self) -> MapViewImplAddr {
            match *self {
                MapViewImplAddr {
                    addr: ref __self_0_0,
                } => MapViewImplAddr {
                    addr: ::std::clone::Clone::clone(&(*__self_0_0)),
                },
            }
        }
    }
    impl Actor for MapViewImpl {
        type Context = Context<MapViewImpl>;
        fn started(&mut self, ctx: &mut <Self as Actor>::Context) {
            self.addr = Some(MapViewImplAddr {
                addr: ctx.address(),
            })
        }
    }
    impl MapViewImpl {
        pub fn new<F: glium::backend::Facade + Clone + 'static>(f: &F) -> Self {
            let src_add = storage::DefaultFileSource::spawn();
            let m = MapViewImpl {
                addr: None,
                facade: Box::new((*f).clone()),
                style: None,
                layers: <[_]>::into_vec(box []),
                source: src_add,
            };
            return m;
        }
        pub fn set_style(&mut self, style: style::Style) {
            self.layers.clear();
            self.layers = layers::parse_style_layers(self.facade.deref(), &style);
            ::io::_print(::std::fmt::Arguments::new_v1_formatted(
                &["Layers : ", "\n"],
                &match (&style,) {
                    (arg0,) => [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Debug::fmt)],
                },
                &[::std::fmt::rt::v1::Argument {
                    position: ::std::fmt::rt::v1::Position::At(0usize),
                    format: ::std::fmt::rt::v1::FormatSpec {
                        fill: ' ',
                        align: ::std::fmt::rt::v1::Alignment::Unknown,
                        flags: 0u32,
                        precision: ::std::fmt::rt::v1::Count::Implied,
                        width: ::std::fmt::rt::v1::Count::Implied,
                    },
                }],
            ));
            self.style = Some(style);
        }
        pub fn set_style_url(&mut self, url: &str) {
            ::io::_print(::std::fmt::Arguments::new_v1_formatted(
                &["Setting style url : ", "\n"],
                &match (&url,) {
                    (arg0,) => [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Debug::fmt)],
                },
                &[::std::fmt::rt::v1::Argument {
                    position: ::std::fmt::rt::v1::Position::At(0usize),
                    format: ::std::fmt::rt::v1::FormatSpec {
                        fill: ' ',
                        align: ::std::fmt::rt::v1::Alignment::Unknown,
                        flags: 0u32,
                        precision: ::std::fmt::rt::v1::Count::Implied,
                        width: ::std::fmt::rt::v1::Count::Implied,
                    },
                }],
            ));
            let resource = storage::Resource::style(url.into());
            self.source
                .get_async(resource, Box::new(self.addr.clone().unwrap()));
        }
        pub fn render(&mut self, target: &mut glium::Frame) {
            for l in self.layers.iter_mut() {
                l.render(target);
            }
        }
    }
    use self::storage::*;
    pub enum MapMethodArgs {
        Render(glium::Frame),
        SetStyleUrl(String),
    }
    impl Message for MapMethodArgs {
        type Result = ();
    }
    impl Handler<MapMethodArgs> for MapViewImpl {
        type Result = ();
        fn handle(
            &mut self,
            mut msg: MapMethodArgs,
            ctx: &mut Self::Context,
        ) -> <Self as Handler<MapMethodArgs>>::Result {
            match msg {
                MapMethodArgs::Render(mut frame) => {
                    self.render(&mut frame);
                    frame.finish();
                }
                MapMethodArgs::SetStyleUrl(url) => self.set_style_url(&url),
            };
        }
    }
}
use prelude::*;
pub fn init() {
    use common::prelude::*;
    ::std::thread::spawn(move || {
        let sys = actix::System::new("test");
        sys.run();
    });
}
