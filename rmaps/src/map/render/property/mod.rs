use ::prelude::*;
use map::style::expr::{Expression, val::Value, DescribeType, Type};

use std::convert::{TryFrom, Into};


pub trait PropertyValue: TryFrom<Value, Error=Type> + Into<Value> + Debug + Clone + Default + DescribeType {}

impl<T: TryFrom<Value, Error=Type> + Into<Value> + Debug + Clone + Default + DescribeType> PropertyValue for T {}


#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct BaseProperty<T: PropertyValue> {
    val: T,
}


impl<T: PropertyValue> BaseProperty<T> {
    pub fn get(&self) -> T {
        return self.val.clone();
    }
    // Evaulate this property, and check whether it has been changed
    pub fn eval(&mut self, expr: &::map::style::expr::Expr, ctx: &::map::style::expr::EvaluationContext) -> bool {
        use map::style::expr::Expression;
        use std::convert::TryInto;

        let v = expr.eval(ctx).unwrap();
        self.val = T::try_from(v).unwrap();
        return true;
    }
    fn set_value(&mut self, v: T) {
        self.val = v;
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
    pub fn evaluate<T: PropertyValue>(&self, prop: &mut BaseProperty<T>, expr: &::map::style::Function<T>, zoom_allowed: bool, feature_allowed: bool) -> Result<bool> {
        match expr {
            ::map::style::Function::Value(v) => {
                prop.set_value((v.clone()));
                return Ok(false);
            }
            ::map::style::Function::Expr(e) => {
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


pub trait Properties {
    type SourceLayerType: ::map::style::StyleLayer;

    fn eval(&mut self, layer: &Self::SourceLayerType, evaluator: &PropertiesEvaluator) -> Result<bool>;
}

