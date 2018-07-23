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

pub mod eval;
pub mod layout;

pub use self::eval::PropertiesEvaluator;
pub use self::layout::PropertyLayoutBuilder;

use std::convert::{TryFrom, Into};

/// Types that can be Property values
pub trait PropValue: TryFrom<Value, Error=Type> + Into<Value> + Debug + Clone + Default + DescribeType + 'static {}

impl<T: TryFrom<Value, Error=Type> + Into<Value> + Debug + Clone + Default + DescribeType + 'static> PropValue for T {}


pub trait DataDrivenValue: PropValue + Attribute + AsUniformValue {}

impl<T: PropValue + Attribute + AsUniformValue> DataDrivenValue for T {}


pub trait Evaluable {
    type Value: PropValue;
    fn eval(&mut self, expr: &Expr, context: &EvaluationContext) -> bool;
    fn get(&self) -> Self::Value;
    fn set(&mut self, v: Self::Value);
}


pub trait Visitable<T: PropValue> {
    fn visit<V: PropertiesVisitor>(&self, visitor: &mut V);
}


#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct PaintProperty<T: PropValue> {
    val: T,
}

impl<T: PropValue> Evaluable for PaintProperty<T> {
    type Value = T;

    fn eval(&mut self, expr: &Expr, context: &EvaluationContext) -> bool {
        let v = expr.eval(context).unwrap();
        self.val = T::try_from(v).unwrap();
        return true;
    }

    fn get(&self) -> Self::Value {
        self.val.clone()
    }

    fn set(&mut self, v: Self::Value) {
        self.val = v;
    }
}

impl<T: PropValue> Visitable<T> for PaintProperty<T> {
    #[inline]
    fn visit<V: PropertiesVisitor>(&self, visitor: &mut V) {
        visitor.visit_base(self)
    }
}


#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DataDrivenProperty<T: DataDrivenValue> (PaintProperty<T>);

impl<T: DataDrivenValue> Evaluable for DataDrivenProperty<T> {
    type Value = T;

    fn eval(&mut self, expr: &Expr, context: &EvaluationContext) -> bool {
        self.0.eval(expr, context)
    }

    fn get(&self) -> Self::Value {
        self.0.get()
    }

    fn set(&mut self, v: Self::Value) {
        self.0.set(v)
    }
}

impl<T: DataDrivenValue> Visitable<T> for DataDrivenProperty<T> {
    #[inline]
    fn visit<V: PropertiesVisitor>(&self, visitor: &mut V) {
        visitor.visit_gpu(&self);
    }
}


pub trait Properties: Default {
    type SourceLayerType: ::map::style::StyleLayer;

    /// Generates layout structure for shader compilation
    fn accept<V: PropertiesVisitor>(&self, layer: &Self::SourceLayerType, visitor: &mut V);

    /// Evaluates the property values, for this object, using specified evaluator
    fn eval(&mut self, layer: &Self::SourceLayerType, evaluator: &PropertiesEvaluator) -> Result<bool>;
}


pub trait PropertiesVisitor {
    fn visit_base<T: PropValue>(&mut self, v: &PaintProperty<T>);
    fn visit_gpu<T: DataDrivenValue>(&mut self, v: &DataDrivenProperty<T>);

    fn visit<T: PropValue, V: Visitable<T>>(&mut self, name: &str, style_prop: &StyleProp<T>, value_prop: &V, can_zoom: bool, can_feature: bool);
}


use map::render::shaders::{
    UniformPropertyLayout,
    FeaturePropertyLayout,
    PropertyItemLayout,
};

pub const FEATURE_UBO_VECS: usize = 1024;

pub struct FeatureDataUbo {
    pub feature_data: [[f32; 4]],
}

implement_buffer_content!(FeatureDataUbo);
implement_uniform_block!(FeatureDataUbo,feature_data);

pub struct FeaturePropertyData {
    pub data: UniformBuffer<FeatureDataUbo>,
    position: usize,
}


impl FeaturePropertyData {
    pub fn new(d: &glium::backend::Facade) -> Result<Self> {
        Ok(FeaturePropertyData {
            data: UniformBuffer::empty_unsized_dynamic(d, mem::size_of::<[f32; 4]>() * FEATURE_UBO_VECS)?,
            position: 0,
        })
    }

    pub fn clear(&mut self) {
        self.position = 0;
    }

    pub fn map_write(&mut self) -> glium::buffer::Mapping<FeatureDataUbo> {
        self.data.map()
    }

    pub fn push_into<A: Attribute>(map: &mut glium::buffer::Mapping<FeatureDataUbo>, v: A, pos: usize) {
        use std::mem;
        use std::ptr;
        use std::slice;

        assert!(A::get_type().get_size_bytes() <= mem::size_of::<f32>() * 4, "Size is : {:?}", A::get_type());
        assert!(mem::size_of::<A>() == A::get_type().get_size_bytes());
        if pos >= FEATURE_UBO_VECS {
            panic!("Too many attributes, TODO");
        }

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
            map.feature_data[pos] = (*slice);
        };
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
        // trace!("Uniform propery binder : got {:?} uniforms", binder.data.values.len());
        Ok(())
    }
}

fn fixup<T: AsUniformValue>(t: T) -> UniformValue<'static> {
// Yaaay, hacks....
    unsafe { ::std::mem::transmute(t.as_uniform_value()) }
}


impl<'a> PropertiesVisitor for UniformPropertyBinder<'a> {
    #[inline]
    fn visit_base<T: PropValue>(&mut self, v: &PaintProperty<T>) {}

    #[inline]
    fn visit_gpu<T: PropValue + Attribute + AsUniformValue>(&mut self, v: &DataDrivenProperty<T>) {
        let x = v.get().clone();
        let u = fixup(x);
        self.last_val = Some(u);//.unwrap();
    }

    #[inline]
    fn visit<T: PropValue, V: Visitable<T>>(&mut self, name: &str, style_prop: &StyleProp<T>, value_prop: &V, can_zoom: bool, can_feature: bool) {
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
    map: glium::buffer::Mapping<'a, FeatureDataUbo>,
    start_size: usize,
    pos: usize,
    push: bool,
}

impl<'a> FeaturePropertyBinder<'a> {
    fn new(layout: &'a FeaturePropertyLayout, data: &'a mut FeaturePropertyData) -> Self {
        FeaturePropertyBinder {
            layout,
            start_size: data.position,
            pos: data.position,
            map: data.map_write(),
            push: false,
        }
    }

    #[inline]
    pub fn with<R,F: FnOnce(&mut FeaturePropertyBinder) -> R>(layout: &FeaturePropertyLayout, data: &mut FeaturePropertyData, fun: F) -> R {
        let (pos, r) = {
            let mut binder = Self::new(layout, data);

            let r = fun(&mut binder);

            (binder.pos, r)
        };


        data.position = pos;
        r
    }
}

impl<'a> PropertiesVisitor for FeaturePropertyBinder<'a> {
    #[inline]
    fn visit_base<T: PropValue>(&mut self, v: &PaintProperty<T>) {}

    #[inline]
    fn visit_gpu<T: PropValue + Attribute + AsUniformValue>(&mut self, v: &DataDrivenProperty<T>) {
        if self.push {
            FeaturePropertyData::push_into(&mut self.map, v.get(), self.pos);
            self.pos += 1;
            self.push = false;
        }
    }

    #[inline]
    fn visit<T: PropValue, V: Visitable<T>>(&mut self, name: &str, style_prop: &StyleProp<T>, value_prop: &V, can_zoom: bool, can_feature: bool) {
        if self.layout.is_feature(name) {
            self.push = true;
        }
        value_prop.visit(self)
    }
}