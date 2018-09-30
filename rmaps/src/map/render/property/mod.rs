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


/// Types that can be Property values
pub trait Propertable: TryFrom<Value, Error=Type> + Into<Value> + Debug + Clone + Default + DescribeType + PartialEq<Self> + 'static {}

impl<T: TryFrom<Value, Error=Type> + Into<Value> + Debug + Clone + Default + DescribeType + PartialEq<Self> + 'static> Propertable for T {}


pub trait GpuPropertable: Propertable + Attribute + AsUniformValue {}

impl<T: Propertable + Attribute + AsUniformValue> GpuPropertable for T {}

#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Property<T: Propertable> {
    val: Option<T>,
}

impl<T: Propertable> Property<T> {
    pub fn get(&self) -> T {
        self.val.clone().unwrap()
    }

    pub fn set(&mut self, v: T) -> bool {
        if self.val.as_ref() == Some(&v) {
            return false;
        }
        self.val = Some(v);
        return true;
    }
}

/// Structs that implement this trait contain properties, that are used only on CPU side
/// visited properties are not uploaded to the GPU, and most of them can be zoom dependent.
/// However, none of visited properties can be feature dependant, because only one instance
/// of `LayerProperties` struct will be instantiated for each layer
pub trait LayerProperties: Default {
    type SourceLayerType: ::map::style::StyleLayer;
    /// Accept a visitor for immutable traversal of these properties
    fn accept<V: PropertiesVisitor>(&self, layer: &Self::SourceLayerType, visitor: &mut V);
    /// Accept a visitor for mutable traversal of these properties
    fn accept_mut<V: PropertiesVisitor>(&mut self, layer: &Self::SourceLayerType, visitor: &mut V);
}

/// Structs that implement `PaintProperties` contain per feature properties.
/// Visited properties can be zoom and feature dependent, and must be of a format that is uploadable to
/// the GPU. So, no String or Enum  PaintProperties
pub trait PaintProperties: Default {
    type SourceLayerType: ::map::style::StyleLayer;
    /// Accept a visitor for immutable traversal of these properties
    fn accept<V: PropertiesVisitor>(&self, layer: &Self::SourceLayerType, visitor: &mut V);
    /// Accept a visitor for mutable traversal of these properties
    fn accept_mut<V: PropertiesVisitor>(&mut self, layer: &Self::SourceLayerType, visitor: &mut V);
}


/// Visitor that can visit individual properties. It has to have duplicated methods for base properties,
/// and GPU properties, because generic argument "T" contains type information about how can value be uploaded to the GPU
pub trait PropertiesVisitor {
    fn visit_layer<T: Propertable>(&mut self, v: &Property<T>, name: &str, style: &StyleProp<T>) {
        panic!("This visitor should not be used on LayerProperties")
    }
    fn visit_paint<T: GpuPropertable>(&mut self, v: &Property<T>, name: &str, style: &StyleProp<T>) {
        panic!("This visitor should not be used on PaintProperties")
    }

    fn visit_layer_mut<T: Propertable>(&mut self, v: &mut Property<T>, name: &str, style: &StyleProp<T>) {
        panic!("This visitor should not be used on LayerProperties")
    }
    fn visit_paint_mut<T: GpuPropertable>(&mut self, v: &mut Property<T>, name: &str, style: &StyleProp<T>) {
        panic!("This visitor should not be used on PaintProperties")
    }
}

use map::render::shaders::{
    UniformPropertyLayout,
    FeaturePropertyLayout,
    PropertyItemLayout,
};

pub const FEATURE_UBO_VECS: usize = 1024;

#[derive(Clone, Copy)]
pub struct FeatureDataUbo {
    pub feature_data: [[f32; 4]; FEATURE_UBO_VECS],
}

//implement_buffer_content!(FeatureDataUbo);
implement_uniform_block!(FeatureDataUbo,feature_data);

pub struct FeaturePropertyData {
    buffer: UniformBuffer<FeatureDataUbo>,
    data: Box<FeatureDataUbo>,
    position: usize,
}


impl FeaturePropertyData {
    pub fn new(d: &glium::backend::Facade) -> Result<Self> {
        unsafe {
            Ok(FeaturePropertyData {
                buffer: UniformBuffer::empty(d)?,
                data: box mem::zeroed(),
                position: 0,
            })
        }
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
        if self.position >= FEATURE_UBO_VECS {
            panic!("Too many attributes, TODO");
        }
        unsafe {
            let dest = &mut self.data.feature_data[self.position];
            *(dest as *mut _ as *mut A) = v;
            self.position += 1;
        }
    }

    pub fn upload(&mut self) {
        self.buffer.write(&self.data);
    }
}


impl fmt::Debug for FeaturePropertyData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { f.write_str("FeaturePropertyData") }
}


impl<'a> AsUniformValue for &'a FeaturePropertyData {
    #[inline]
    fn as_uniform_value(&self) -> UniformValue {
        UniformValue::Block(self.buffer.as_slice_any(), |_| { Ok(()) })
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { f.write_str("UniformPropertyData") }
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
    pub fn rebind<P: PaintProperties>(layout: &UniformPropertyLayout, props: &P, style: &P::SourceLayerType, data: &mut UniformPropertyData) -> Result<()> {
        data.clear();
        let mut binder = UniformPropertyBinder::new(layout, data);
        props.accept(style, &mut binder);
        Ok(())
    }
}

fn fixup<T: AsUniformValue>(t: T) -> UniformValue<'static> {
    unsafe { ::std::mem::transmute(t.as_uniform_value()) }
}


impl<'a> PropertiesVisitor for UniformPropertyBinder<'a> {
    fn visit_paint<T: GpuPropertable>(&mut self, v: &Property<T>, name: &str, style: &StyleProp<T>) {
        if self.layout.is_uniform(name) {
            let val = fixup(v.get());
            self.data.push(::map::render::shaders::ShaderProcessor::uniform_name(name), val);
        }
    }
}


pub struct FeaturePropertyBinder<'a> {
    layout: &'a FeaturePropertyLayout,
    data: &'a mut FeaturePropertyData,
    start_size: usize,
    pos: usize,
}

impl<'a> FeaturePropertyBinder<'a> {
    fn new(layout: &'a FeaturePropertyLayout, data: &'a mut FeaturePropertyData) -> Self {
        FeaturePropertyBinder {
            layout,
            start_size: data.position,
            pos: data.position,
            data,
        }
    }

    #[inline]
    pub fn with<R, F: FnOnce(&mut FeaturePropertyBinder) -> R>(layout: &FeaturePropertyLayout, data: &mut FeaturePropertyData, fun: F) -> R {
        let r = {
            let mut binder = Self::new(layout, data);
            let r = fun(&mut binder);
            r
        };
        data.upload();
        r
    }
}

impl<'a> PropertiesVisitor for FeaturePropertyBinder<'a> {
    fn visit_paint<T: GpuPropertable>(&mut self, v: &Property<T>, name: &str, style: &StyleProp<T>) {
        if self.layout.is_feature(name) {
            self.data.push(v.get());
        }
    }
}