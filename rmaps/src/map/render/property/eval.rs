use ::prelude::*;
use map::render::EvaluationParams;
use super::*;

pub struct PropertiesEvaluator<'a> {
    zoom: Option<f32>,
    feature: Option<&'a ::mvt::Feature>,
    pub modified: bool,
}

impl<'a, 'b : 'a> From<&'b EvaluationParams> for PropertiesEvaluator<'a> {
    fn from(p: &'b EvaluationParams) -> Self {
        PropertiesEvaluator {
            zoom: Some(p.zoom),
            feature: None,
            modified: false,
        }
    }
}

impl<'a> PropertiesVisitor for PropertiesEvaluator<'a> {
    fn visit_base_mut<T: Propertable, Z: Bool, F: Bool>(&mut self, v: &mut Property<T, Z, F>, name: &str, style: &StyleProp<T>) {
        self.modified |= self.evaluate(v, style).unwrap();
    }

    fn visit_gpu_mut<T: GpuPropertable, Z: Bool, F: Bool>(&mut self, v: &mut Property<T, Z, F>, name: &str, style: &StyleProp<T>) {
        self.modified |= self.evaluate(v, style).unwrap();
    }
}

impl<'a> PropertiesEvaluator<'a> {
    pub fn only_zoom(zoom: f32) -> Self {
        return PropertiesEvaluator {
            zoom: Some(zoom as _),
            feature: None,
            modified: false,
        };
    }

    pub fn new(zoom: f32, feature: &'a ::mvt::Feature) -> Self {
        return PropertiesEvaluator {
            zoom: Some(zoom as _),
            feature: Some(feature),
            modified: false,
        };
    }
    pub fn with_feature(mut self, feature: &'a ::mvt::Feature) -> Self {
        self.feature = Some(feature);
        self
    }
    pub fn evaluate<T: Propertable, Z: Bool, F: Bool>(&self, prop: &mut Property<T, Z, F>, expr: &::map::style::StyleProp<T>) -> Result<bool> {
        let mut zoom_allowed = Z::VALUE;
        let mut feature_allowed = F::VALUE;

        match expr {
            ::map::style::StyleProp::Value(v) => {
                return Ok(prop.set(v.clone()));
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

                if e.is_feature() && ctx.feature_data.is_none() {
                    return Ok(false);
                }

                let res = e.eval(&ctx).unwrap();
                return Ok(prop.set(T::try_from(res).unwrap()));
            }
        }
        Ok(false)
    }
}