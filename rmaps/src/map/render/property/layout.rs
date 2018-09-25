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
    #[inline(always)]
    fn visit_gpu<T: GpuPropertable, Z: Bool, F: Bool>(&mut self, v: &Property<T, Z, F>, name: &str, style: &StyleProp<T>) {
        let can_zoom = Z::VALUE;
        let can_feature = F::VALUE;

        if !can_zoom && style.is_zoom() {
            panic!("Style not supported, `{}` can't be a zoom property", name);
        }

        if !can_feature && style.is_feature() {
            panic!("Style not supported, `{}` can't be a feature property", name);
        }
        if style.is_feature() {
            self.features.push(name, T::get_type());
        } else {
            self.uniforms.push(name, T::get_type());
        }
    }
}
