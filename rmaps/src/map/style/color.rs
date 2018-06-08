use prelude::*;
use css_color_parser::{
    self,
    Color as CssColor,
};

use common::serde::{
    self,
    Deserializer,
    Deserialize,
};

#[derive(Debug, Clone)]
pub struct Color(pub CssColor);

impl<'de> serde::Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de> {
        use std::str::FromStr;

        let data: String = Deserialize::deserialize(deserializer)?;
        let color = CssColor::from_str(&data).map_err(|_| serde::de::Error::custom("Invalid color"))?;
        Ok(Color(color))
    }
}

impl Color {
    pub fn to_rgba(&self) -> [f32; 4] {
        return [
            self.0.r as f32 / 255f32,
            self.0.g as f32 / 255f32,
            self.0.b as f32 / 255f32,
            self.0.a
        ];
    }
}