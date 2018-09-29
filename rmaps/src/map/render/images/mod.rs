use ::prelude::*;

use map::style::sprite::{
    SpriteAtlas, Sprite,
};

use glium::texture::Texture2d;


pub struct ImageAtlas {
    sprite_atlas: Option<SpriteAtlas>,
    sprite_texture: Option<Texture2d>,
    display: Box<Display>,
}

#[derive(Debug, Clone)]
pub struct ImagePosition {
    pub tl: [f32; 2],
    pub br: [f32; 2],
}

impl ImageAtlas {
    pub fn new(display: &Display) -> Result<Self> {
        Ok(ImageAtlas {
            sprite_atlas: None,
            sprite_texture: None,
            display: box (*display).clone(),
        })
    }

    pub fn set_sprite_atlas(&mut self, atlas: SpriteAtlas) {
        self.sprite_atlas = Some(atlas);
    }

    pub fn set_sprite_texture(&mut self, data: Vec<u8>) {
        let format = image::guess_format(&data).unwrap();
        let decoded = image::load_from_memory_with_format(&data, format).unwrap().to_rgba();
        let dims = decoded.dimensions();
        let raw = glium::texture::RawImage2d::from_raw_rgba_reversed(&decoded, dims);

        let texture = glium::texture::Texture2d::new(self.display.deref(), raw).unwrap();
        self.sprite_texture = Some(texture);
    }

    pub fn atlas_dims(&self) -> [f32; 2] {
        if let Some(ref t) = self.sprite_texture {
            return [t.width() as f32, t.height() as f32];
        }
        return [0., 0.];
    }
    pub fn get_pattern(&self, name: &str) -> Option<(ImagePosition, &Texture2d)> {
        if let (Some(ref s), Some(ref t)) = (&self.sprite_atlas, &self.sprite_texture) {
            let dims = self.atlas_dims();

            return s.get(name).map(|v|
                (ImagePosition {
                    tl: [v.x as f32, dims[1] - (v.y + v.height) as f32],
                    br: [(v.x + v.width) as f32, dims[1] - v.y as f32],
                }, t)
            );
        }

        return None;
    }
}