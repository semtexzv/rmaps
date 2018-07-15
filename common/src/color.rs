use prelude::*;

use css_color_parser::{
    self,
    Color as CssColor,
};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Color(pub [f32; 4]);

impl Default for Color {
    fn default() -> Self {
        Color([0., 0., 0., 1.])
    }
}

impl ::std::ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
            self.0[3] + rhs.0[3],
        ])
    }
}

impl ::std::ops::Sub<Color> for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Self::Output {
        Color([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
            self.0[3] - rhs.0[3],
        ])
    }
}

impl ::std::ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        let rhs = rhs as f32;
        Color([
            self.0[0] * rhs,
            self.0[1] * rhs,
            self.0[2] * rhs,
            self.0[3] * rhs,
        ])
    }
}

unsafe impl glium::vertex::Attribute for Color {
    fn get_type() -> glium::vertex::AttributeType {
        glium::vertex::AttributeType::F32F32F32F32
    }
}

impl glium::uniforms::AsUniformValue for Color {
    fn as_uniform_value(&self) -> glium::uniforms::UniformValue {
        glium::uniforms::UniformValue::Vec4(self.0)
    }
}

impl ::std::str::FromStr for Color {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let color = CssColor::from_str(&s)?;
        Ok(Color([color.r as f32 / 255., color.g as f32 / 255., color.b as f32 / 255., color.a]))
    }
}

impl<'de> serde::Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error> where
        D: Deserializer<'de> {
        use std::str::FromStr;

        let data: String = Deserialize::deserialize(deserializer)?;
        let color = CssColor::from_str(&data).map_err(|_| serde::de::Error::custom("Invalid color"))?;
        Ok(Color([color.r as f32 / 255., color.g as f32 / 255., color.b as f32 / 255., color.a]))
    }
}

impl Color {
    pub fn new(r: impl Into<f32>, g: impl Into<f32>, b: impl Into<f32>, a: impl Into<f32>) -> Self {
        return Color([
            r.into(),
            g.into(),
            b.into(),
            a.into()
        ]);
    }
    pub fn to_rgba(&self) -> [f32; 4] {
        return self.0;
    }

    pub fn r(&self) -> f32 {
        self.0[0]
    }

    pub fn g(&self) -> f32 {
        self.0[1]
    }

    pub fn b(&self) -> f32 {
        self.0[2]
    }

    pub fn a(&self) -> f32 {
        self.0[3]
    }
}