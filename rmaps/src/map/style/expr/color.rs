use ::prelude::*;

use super::*;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum _RgbMarker {
    #[serde(rename = "rgb")]
    _Mark
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum _RgbaMarker {
    #[serde(rename = "rgba")]
    _Mark
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum _ToRgbaMarker {
    #[serde(rename = "to-rgba")]
    _Mark,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum ColorExpr {
    Rgb(_RgbMarker, BaseExpr, BaseExpr, BaseExpr),
    Rgba(_RgbaMarker, BaseExpr, BaseExpr, BaseExpr, BaseExpr),
    ToRgba(_ToRgbaMarker, BaseExpr),
}

impl Expr for ColorExpr {
    fn is_zoom(&self) -> bool {
        match self {
            ColorExpr::Rgb(_, a, b, c) => a.is_zoom() || b.is_zoom() || c.is_zoom(),
            ColorExpr::Rgba(_, a, b, c, d) => a.is_zoom() || b.is_zoom() || c.is_zoom() || d.is_zoom(),
            ColorExpr::ToRgba(_, x) => x.is_zoom(),
        }
    }

    fn is_feature(&self) -> bool {
        match self {
            ColorExpr::Rgb(_, a, b, c) => a.is_feature() || b.is_feature() || c.is_feature(),
            ColorExpr::Rgba(_, a, b, c, d) => a.is_feature() || b.is_feature() || c.is_feature() || d.is_feature(),
            ColorExpr::ToRgba(_, x) => x.is_feature(),
        }
    }

    fn eval(&self, ctx: &EvaluationContext) -> ExprResult {
        match self {
            ColorExpr::Rgb(_, a, b, c) => {
                let ea = expect_num(a.eval(ctx)?)?;
                let eb = expect_num(b.eval(ctx)?)?;
                let ec = expect_num(c.eval(ctx)?)?;

                return Ok(ExprVal::Color(Color::new(ea as f32, eb as f32, ec as f32, 1.)));
            }
            ColorExpr::Rgba(_, a, b, c, d) => {

                let ea = expect_num(a.eval(ctx)?)?;
                let eb = expect_num(b.eval(ctx)?)?;
                let ec = expect_num(c.eval(ctx)?)?;
                let ed = expect_num(d.eval(ctx)?)?;

                return Ok(ExprVal::Color(Color::new(ea as f32, eb as f32, ec as f32, ed as f32)));
            }
            ColorExpr::ToRgba(_, x) => {
                let ea = expect_color(x.eval(ctx)?)?;
                return Ok(ExprVal::List(
                    vec![
                        ea.r().into(),
                        ea.g().into(),
                        ea.b().into(),
                        ea.a().into(),
                    ]
                ));
            }
        }
    }
}
