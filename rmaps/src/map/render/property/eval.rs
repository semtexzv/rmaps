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
    fn visit_layer_mut<T: Propertable>(&mut self, v: &mut Property<T>, name: &str, style: &StyleProp<T>) {
        self.modified |= self.evaluate(v, false, style).unwrap();
    }

    fn visit_paint_mut<T: GpuPropertable>(&mut self, v: &mut Property<T>, name: &str, style: &StyleProp<T>) {
        self.modified |= self.evaluate(v, true, style).unwrap();
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

    /*
    pub fn with_feature(mut self, feature: &'a ::mvt::Feature) -> Self {
        self.feature = Some(feature);
        self
    }
    */

    pub fn evaluate<T: Propertable>(&self, prop: &mut Property<T>, can_be_data_driven: bool, expr: &::map::style::StyleProp<T>) -> Result<bool> {
        match expr {
            ::map::style::StyleProp::Value(v) => {
                return Ok(prop.set(v.clone()));
            }
            ::map::style::StyleProp::Expr(e) => {
                // Do not evaluate per-feature expression in per-bucket properties
                // and per-bucket expression in per-feature properties.
                if e.is_feature() != self.feature.is_some() {
                    return Ok(false);
                }
                if e.is_feature() && !can_be_data_driven {
                    bail!("Data driven expression not allowed : {:?}", e)
                }

                if e.is_zoom() && self.zoom.is_none() {
                    bail!("Zoom driven expression not suported : {:?}", e)
                }


                let ctx = ::map::style::expr::EvaluationContext::new(self.zoom,self.feature);
                let res = e.eval(&ctx).unwrap();
                return Ok(prop.set(T::try_from(res).unwrap()));
            }
        }
        Ok(false)
    }
}