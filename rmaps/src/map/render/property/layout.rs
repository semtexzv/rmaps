use ::prelude::*;

use map::style::{
    StyleProp,
    expr::{Expression, Expr, val::Value, DescribeType, Type, EvaluationContext},
};

use map::render::shaders::{
    UniformPropertyLayout,
    FeaturePropertyLayout,
    PropertyItemLayout
};

use super::{
    Propertable,
    DataDrivenPropertable,
    PaintProperty,
    DataDrivenProperty,
    Properties,
    PropertiesVisitor,
    Visitable,
};

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
    fn visit_base<T: Propertable>(&mut self, v: &PaintProperty<T>) {
        self.last_attr_type = None;
        // Noop, not a property that will be used on GPU
    }

    #[inline]
    fn visit_gpu<T: DataDrivenPropertable>(&mut self, v: &DataDrivenProperty<T>) {
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



        //debug!("Visited : {} ", name);
        //debug!("\t Zoom    : allowed {} , used : {}", can_zoom, style_prop.is_zoom());
        //debug!("\t Feature : allowed {} , used : {}", can_feature, style_prop.is_feature());
        //debug!("\t Attribute type : {:?}", self.last_attr_type);

        if let Some(attr) = self.last_attr_type {
            if style_prop.is_feature() {
                self.features.push(name, attr);
            } else {
                self.uniforms.push(name, attr);
            }
        }
    }
}
