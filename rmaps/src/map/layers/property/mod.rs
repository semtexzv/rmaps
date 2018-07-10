use ::prelude::*;
use map::style::expr::Expr;


#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct BaseProperty<T: Debug + Clone + Default> {
    val: T,
}


impl<T: Debug + Clone + Default> BaseProperty<T> {
    pub fn get(&self) -> T {
        return self.val.clone();
    }
    // Evaulate this property, and check whether it has been changed
    pub fn eval(&mut self, expr: &::map::style::expr::BaseExpr, ctx: &::map::style::expr::EvaluationContext) -> bool {
        use map::style::expr::Expr;
        use std::convert::TryInto;

        let v = expr.eval(ctx).unwrap();
        //self.val = v.try_into().unwrap();
        return true;
    }
    fn set_value(&mut self, v: T) {
        self.val = v;
    }
}

pub struct PropertiesEvaluator {
    zoom: Option<f32>,
}

impl PropertiesEvaluator {
    pub fn only_zoom(zoom: f64) -> Self {
        return PropertiesEvaluator {
            zoom: Some(zoom as _),
        };
    }
    pub fn evaluate<T: Debug + Clone + Default, >(&self, prop: &mut BaseProperty<T>, expr: &::map::style::Function<T>, zoom_allowed: bool, feature_allowed: bool) -> Result<bool> {
        match expr {
            ::map::style::Function::Value(v) => {
                prop.set_value((v.clone()));
                return Ok(false);
            }
            ::map::style::Function::Expr(e) => {
                let ctx = ::map::style::expr::EvaluationContext {
                    zoom: self.zoom,
                    feature_data: None,
                    bindings: ::std::cell::RefCell::new(BTreeMap::new()),
                };
                if e.is_zoom() && !zoom_allowed {
                    bail!("Zoom expression not allowed");
                } else if e.is_feature() && !feature_allowed {
                    bail!("Feature expression not allowed")
                }
                return Ok(prop.eval(e, &ctx));
            }
        }
        Ok(false)
    }
}


pub trait Properties {
    type SourceLayerType: ::map::style::StyleLayer;

    fn eval(&mut self, layer: &Self::SourceLayerType, evaluator: &PropertiesEvaluator) -> Result<bool>;
}

