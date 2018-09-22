use ::prelude::*;
use super::*;

use map::render::EvaluationParams;

pub struct PropertiesEvaluator<'a> {
    zoom: Option<f32>,
    feature: Option<&'a ::mvt::Feature>,
}

impl<'a, 'b : 'a> From<&'b EvaluationParams> for PropertiesEvaluator<'a> {
    fn from(p: &'b EvaluationParams) -> Self {
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
    pub fn evaluate<T: PropValue, E: Evaluable<Value=T>>(&self, prop: &mut E, expr: &::map::style::StyleProp<T>, zoom_allowed: bool, feature_allowed: bool) -> Result<bool> {
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
                if e.is_feature() && ctx.feature_data.is_none() {
                    return Ok(true);
                }
                let res = prop.eval(&e.0, &ctx);
                return Ok(res);
            }
        }
        Ok(false)
    }
}