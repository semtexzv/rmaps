use prelude::*;

use ::map::style;


#[derive(Debug)]
pub struct RasterLayer {
    style_layer : style::RasterLayer,
}

impl super::Layer for RasterLayer {

    fn render<S: glium::Surface>(&mut self, surface: &mut S) -> Result<()> {
        //unimplemented!()
        Ok(())
    }
}

impl RasterLayer {
    pub fn parse(layer: style::RasterLayer) -> Self {
        return RasterLayer {
            style_layer: layer,
        };
    }
}