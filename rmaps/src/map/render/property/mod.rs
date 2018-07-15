use ::prelude::*;
use map::style::{
    StyleProp,
    expr::{Expression, Expr, val::Value, DescribeType, Type, EvaluationContext},
};


use ::common::glium::{
    vertex::Attribute,
    uniforms::{
        AsUniformValue,
        Uniforms,
        UniformValue,
        UniformType,
    },
};

use std::convert::{TryFrom, Into};

/// Types that can be Property values
pub trait Propertable: TryFrom<Value, Error=Type> + Into<Value> + Debug + Clone + Default + DescribeType + 'static {}

impl<T: TryFrom<Value, Error=Type> + Into<Value> + Debug + Clone + Default + DescribeType + 'static> Propertable for T {}

pub trait Evaluable {
    type ValueType: Propertable;
    fn eval(&mut self, expr: &Expr, context: &EvaluationContext) -> bool;
    fn get(&self) -> Self::ValueType;
    fn set(&mut self, v: Self::ValueType);
}

pub trait Visitable<T: Propertable> {
    fn visit<V: PropertiesVisitor>(&self, visitor: &mut V);
}


#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct BaseProp<T: Propertable> {
    val: T,
}

impl<T: Propertable> Evaluable for BaseProp<T> {
    type ValueType = T;

    fn eval(&mut self, expr: &Expr, context: &EvaluationContext) -> bool {
        let v = expr.eval(context).unwrap();
        self.val = T::try_from(v).unwrap();
        return true;
    }

    fn get(&self) -> Self::ValueType {
        self.val.clone()
    }

    fn set(&mut self, v: Self::ValueType) {
        self.val = v;
    }
}

impl<T: Propertable> Visitable<T> for BaseProp<T> {
    #[inline]
    fn visit<V: PropertiesVisitor>(&self, visitor: &mut V) {
        visitor.visit_base(self)
    }
}


#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct GpuProp<T: Propertable + glium::vertex::Attribute> (BaseProp<T>);

impl<T: Propertable + glium::vertex::Attribute> Evaluable for GpuProp<T> {
    type ValueType = T;

    fn eval(&mut self, expr: &Expr, context: &EvaluationContext) -> bool {
        self.0.eval(expr, context)
    }

    fn get(&self) -> Self::ValueType {
        self.0.get()
    }

    fn set(&mut self, v: Self::ValueType) {
        self.0.set(v)
    }
}

impl<T: Propertable + Attribute + AsUniformValue> Visitable<T> for GpuProp<T> {
    #[inline]
    fn visit<V: PropertiesVisitor>(&self, visitor: &mut V) {
        visitor.visit_gpu(&self);
    }
}

pub struct PropertiesEvaluator<'a> {
    zoom: Option<f32>,
    feature: Option<&'a ::mvt::Feature>,
}

impl<'a, 'b : 'a> From<&'b super::EvaluationParams> for PropertiesEvaluator<'a> {
    fn from(p: &'b super::EvaluationParams) -> Self {
        PropertiesEvaluator {
            zoom: Some(p.zoom),
            feature: None,
        }
    }
}

impl<'a> PropertiesEvaluator<'a> {
    pub fn only_zoom(zoom: f32) -> Self {
        return PropertiesEvaluator {
            zoom: Some(zoom as _),
            feature: None,
        };
    }

    pub fn new(zoom: f32, feature: &'a ::mvt::Feature) -> Self {
        return PropertiesEvaluator {
            zoom: Some(zoom as _),
            feature: Some(feature),
        };
    }
    pub fn with_feature(mut self, feature: &'a ::mvt::Feature) -> Self {
        self.feature = Some(feature);
        self
    }
    pub fn evaluate<T: Propertable, E: Evaluable<ValueType=T>>(&self, prop: &mut E, expr: &::map::style::StyleProp<T>, zoom_allowed: bool, feature_allowed: bool) -> Result<bool> {
        match expr {
            ::map::style::StyleProp::Value(v) => {
                prop.set((v.clone()));
                return Ok(false);
            }
            ::map::style::StyleProp::Expr(e) => {
                let ctx = ::map::style::expr::EvaluationContext {
                    zoom: self.zoom,
                    feature_data: self.feature,
                    bindings: ::std::cell::RefCell::new(BTreeMap::new()),
                };
                if e.is_zoom() && !zoom_allowed {
                    bail!("Zoom expression not allowed for expression : {:?}", e);
                } else if e.is_feature() && !feature_allowed {
                    bail!("Feature expression not allowed for expression : {:?}", e)
                }
                let res = prop.eval(&e.0, &ctx);
                return Ok(res);
            }
        }
        Ok(false)
    }
}

/// Struct used to build up "Layout" object from style layer properties
pub trait PropertiesVisitor {
    fn visit_base<T: Propertable>(&mut self, v: &BaseProp<T>);
    fn visit_gpu<T: Propertable + Attribute + AsUniformValue>(&mut self, v: &GpuProp<T>);

    fn visit<T: Propertable, V: Visitable<T>>(&mut self, name: &str, style_prop: &StyleProp<T>, value_prop: &V, can_zoom: bool, can_feature: bool);
}


pub trait Properties: Default {
    type SourceLayerType: ::map::style::StyleLayer;

    /// Generates layout structure for shader compilation
    fn accept<V: PropertiesVisitor>(&self, layer: &Self::SourceLayerType, visitor: &mut V);


    /// Evaluates the property values, for this object, using specified evaluator
    fn eval(&mut self, layer: &Self::SourceLayerType, evaluator: &PropertiesEvaluator) -> Result<bool>;
}

use map::render::shaders::{UniformPropertyLayout, FeaturePropertyLayout, PropertyItemLayout};

#[derive(Debug, Default)]
pub struct PropertyLayoutBuilder {
    uniforms: UniformPropertyLayout,
    features: FeaturePropertyLayout,

    last_attr_type: Option<glium::vertex::AttributeType>,
}


impl PropertyLayoutBuilder {
    pub fn build<P: Properties>(layer: &P::SourceLayerType) -> (UniformPropertyLayout, FeaturePropertyLayout) {
        let mut builder = PropertyLayoutBuilder::default();
        let tmp = P::default();
        tmp.accept(&layer, &mut builder);

        return (builder.uniforms, builder.features);
    }
}

impl PropertiesVisitor for PropertyLayoutBuilder {
    #[inline]
    fn visit_base<T: Propertable>(&mut self, v: &BaseProp<T>) {
        self.last_attr_type = None;
        // Noop, not a property that will be used on GPU
    }

    #[inline]
    fn visit_gpu<T: Propertable + glium::vertex::Attribute>(&mut self, v: &GpuProp<T>) {
        self.last_attr_type = Some(T::get_type());
    }

    #[inline]
    fn visit<T: Propertable, V: Visitable<T>>(&mut self, name: &str, style_prop: &StyleProp<T>, value_prop: &V, can_zoom: bool, can_feature: bool) {
        value_prop.visit(self);

        if !can_zoom && style_prop.is_zoom() {
            panic!("Style not supported, `{}` can't be a zoom property", name);
        }

        if !can_feature && style_prop.is_feature() {
            panic!("Style not supported, `{}` can't be a feature property", name);
        }

        debug!("Visited : {} ", name);
        debug!("\t Zoom    : allowed {} , used : {}", can_zoom, style_prop.is_zoom());
        debug!("\t Feature : allowed {} , used : {}", can_feature, style_prop.is_feature());
        debug!("\t Attribute type : {:?}", self.last_attr_type);

        if let Some(attr) = self.last_attr_type {
            if style_prop.is_feature() {
                self.features.push(name, attr);
            } else {
                self.uniforms.push(name, attr);
            }
        }
    }
}

use ::common::glium::uniforms::UniformBuffer;
use ::common::glium::texture::buffer_texture::*;


pub struct FeaturePropertyData {
    pub data: UniformBuffer<[[f32; 4]; 1024]>,
    storage: Box<[[f32; 4]; 1024]>,
    position: usize,

}


impl FeaturePropertyData {
    pub fn new(d: &glium::backend::Facade) -> Result<Self> {
        let len = 2048;
        Ok(FeaturePropertyData {
            data: UniformBuffer::empty_dynamic(d)?,
            storage: Box::new([[0.; 4]; 1024]),
            position: 0,
        })
    }

    pub fn clear(&mut self) {
        self.position = 0;
    }

    pub fn push<A: Attribute>(&mut self, v: A) {
        use std::mem;
        use std::ptr;
        use std::slice;

        assert!(A::get_type().get_size_bytes() <= mem::size_of::<f32>() * 4, "Size is : {:?}", A::get_type());
        assert!(mem::size_of::<A>() == A::get_type().get_size_bytes());

        #[repr(C)]
        struct helper<A: Sized> {
            first: A,
            pad: [f32; 4],
        }
        unsafe {
            let data = helper {
                first: v,
                pad: [0.; 4],
            };
            let ptr = &data as *const helper<A> as *const f32;
            let slice = ptr as *const [f32; 4];
            self.storage[self.position] = (*slice);
            self.position += 1;
        };
    }

    pub fn upload(&mut self, disp: &Display) -> Result<()> {
        self.data.write(&self.storage);
        Ok(())
    }
}

impl fmt::Debug for FeaturePropertyData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FeaturePropertyData")
    }
}


#[derive(Default)]
pub struct UniformPropertyData {
    values: Vec<(String, glium::uniforms::UniformValue<'static>)>
}

impl glium::uniforms::Uniforms for UniformPropertyData {
    fn visit_values<'a, F: FnMut(&str, glium::uniforms::UniformValue<'a>)>(&'a self, mut f: F) {
        for (n, v) in self.values.iter() {
            f(&n, v.clone())
        }
    }
}

impl UniformPropertyData {
    fn new() -> Self {
        UniformPropertyData {
            values: Vec::new()
        }
    }
    fn clear(&mut self) {
        self.values.clear();
    }

    #[inline]
    fn push(&mut self, name: impl Into<String>, val: UniformValue<'static>) {
        self.values.push((name.into(), val));
    }
}

impl fmt::Debug for UniformPropertyData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("UniformPropertyData")
    }
}


pub struct MergeUniforms<'u, A: Uniforms + 'u, B: Uniforms + 'u>(pub &'u A, pub &'u B);


impl<'u, A: Uniforms + 'u, B: Uniforms + 'u> Uniforms for MergeUniforms<'u, A, B> {
    fn visit_values<'a, F: FnMut(&str, UniformValue<'a>)>(&'a self, mut f: F) {
        self.0.visit_values(&mut f);
        self.1.visit_values(f);
    }
}


pub struct UniformPropertyBinder<'a> {
    layout: &'a UniformPropertyLayout,
    data: &'a mut UniformPropertyData,
    last_val: Option<glium::uniforms::UniformValue<'static>>,
}

impl<'a> UniformPropertyBinder<'a> {
    fn new(layout: &'a UniformPropertyLayout, data: &'a mut UniformPropertyData) -> Self {
        UniformPropertyBinder {
            layout,
            data,
            last_val: None,
        }
    }
    #[inline]
    pub fn bind<P: Properties>(layout: &UniformPropertyLayout, props: &P, style: &P::SourceLayerType, data: &mut UniformPropertyData) -> Result<()> {
        data.clear();
        let mut binder = UniformPropertyBinder::new(layout, data);
        props.accept(style, &mut binder);
        trace!("Uniform propery binder : got {:?} uniforms", binder.data.values.len());
        Ok(())
    }
}

fn fixup<T: AsUniformValue>(t: T) -> UniformValue<'static> {
    // Yaaay, hacks....
    unsafe { ::std::mem::transmute(t.as_uniform_value()) }
}


impl<'a> PropertiesVisitor for UniformPropertyBinder<'a> {
    #[inline]
    fn visit_base<T: Propertable>(&mut self, v: &BaseProp<T>) {}

    #[inline]
    fn visit_gpu<T: Propertable + Attribute + AsUniformValue>(&mut self, v: &GpuProp<T>) {
        let x = v.get().clone();
        let u = fixup(x);
        self.last_val = Some(u);//.unwrap();
    }

    #[inline]
    fn visit<T: Propertable, V: Visitable<T>>(&mut self, name: &str, style_prop: &StyleProp<T>, value_prop: &V, can_zoom: bool, can_feature: bool) {
        value_prop.visit(self);

        if let Some(val) = self.last_val.take() {
            if self.layout.is_uniform(name) {
                self.data.push(::map::render::shaders::ShaderProcessor::uniform_name(name), val);
            }
        }
    }
}

pub struct FeaturePropertyBinder<'a> {
    layout: &'a FeaturePropertyLayout,
    data: &'a mut FeaturePropertyData,
    start_size: usize,
    push: bool,
}

impl<'a> FeaturePropertyBinder<'a> {
    fn new(layout: &'a FeaturePropertyLayout, data: &'a mut FeaturePropertyData) -> Self {
        FeaturePropertyBinder {
            layout,
            start_size: data.storage.len(),
            data,
            push: false,
        }
    }
    #[inline]
    pub fn extend<P: Properties>(layout: &FeaturePropertyLayout, props: &P, style: &P::SourceLayerType, data: &mut FeaturePropertyData) {
        let mut binder = Self::new(layout, data);

        props.accept(style, &mut binder);
        trace!("Feature propery binder finished  start : {}, end {}", binder.start_size, binder.data.storage.len());
    }
}

impl<'a> PropertiesVisitor for FeaturePropertyBinder<'a> {
    #[inline]
    fn visit_base<T: Propertable>(&mut self, v: &BaseProp<T>) {}

    #[inline]
    fn visit_gpu<T: Propertable + Attribute + AsUniformValue>(&mut self, v: &GpuProp<T>) {
        if self.push {
            self.data.push(v.get());
            self.push = false;
        }
    }

    #[inline]
    fn visit<T: Propertable, V: Visitable<T>>(&mut self, name: &str, style_prop: &StyleProp<T>, value_prop: &V, can_zoom: bool, can_feature: bool) {
        if self.layout.is_feature(name) {
            self.push = true;
        }
        value_prop.visit(self)
    }
}