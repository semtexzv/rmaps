use ::prelude::*;

pub type SpriteAtlas = BTreeMap<String, Sprite>;

#[derive(Debug, Clone, Deserialize)]
pub struct Sprite {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    #[serde(rename = "pixelRatio")]
    pub pixel_ratio: i32,
}