use prelude::*;

use ::map::style;

#[derive(Debug)]
pub struct BackgroundLayer {
    style_layer: style::BackgroundLayer,
}

impl super::Layer for BackgroundLayer {
    fn render<S: glium::Surface>(&mut self, surface: &mut S) -> Result<()> {
        let color = self.style_layer.paint.color.eval();
        let c = color.to_rgba();
        surface.clear_color(c[0], c[1], c[2], c[3]);

        Ok(())
    }
}

impl BackgroundLayer {
    pub fn parse(layer: style::BackgroundLayer) -> Self {
        return BackgroundLayer {
            style_layer: layer,
        };
    }
}