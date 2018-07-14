use super::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Zoom {}

parse! {Zoom as expected;
    "zoom" => {
       Ok(Zoom{})
    }
}
impl Expression for Zoom {
    fn is_zoom(&self) -> bool {
        true
    }

    fn is_feature(&self) -> bool {
        false
    }

    fn eval(&self, ctx: &EvaluationContext) -> ExprResult {
        Ok(Value::Num(ctx.zoom.unwrap() as _))
    }
}