use ::prelude::*;

use map::style::{
    StyleProp,
    expr::{Expression, Expr, val::Value, DescribeType, Type, EvaluationContext},
};

use map::render::shaders::{
    UniformPropertyLayout,
    FeaturePropertyLayout,
    PropertyItemLayout,
};

use super::{
    Propertable,
    GpuPropertable,
    LayerProperties, PaintProperties,
    PropertiesVisitor,
    Property,
};

#[derive(Debug, Default)]
pub struct PropertyLayoutBuilder {
    uniforms: UniformPropertyLayout,
    features: FeaturePropertyLayout,

}


impl PropertyLayoutBuilder {
    pub fn build<P: PaintProperties>(layer: &P::SourceLayerType) -> (UniformPropertyLayout, FeaturePropertyLayout) {
        let mut builder = PropertyLayoutBuilder::default();
        let tmp = P::default();
        tmp.accept(&layer, &mut builder);

        return (builder.uniforms, builder.features);
    }
}

impl PropertiesVisitor for PropertyLayoutBuilder {
    fn visit_layer<T: Propertable>(&mut self, v: &Property<T>, name: &str, style: &StyleProp<T>) {
        panic!("PropertylayoutBuilder should not be used on LayerProperties")
    }

    #[inline(always)]
    fn visit_paint<T: GpuPropertable>(&mut self, v: &Property<T>, name: &str, style: &StyleProp<T>) {
        if style.is_feature() {
            self.features.push(name, T::get_type());
        } else {
            self.uniforms.push(name, T::get_type());
        }
    }
}
