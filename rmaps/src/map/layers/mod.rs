use prelude::*;

pub mod background_layer;
pub mod raster_layer;

pub trait Layer : Sized + Debug {
    fn render<S: glium::Surface>(&mut self, surface: &mut S) -> Result<()>;
}

#[derive(Debug)]
pub enum LayerHolder{
    Background(background_layer::BackgroundLayer),
    Raster(raster_layer::RasterLayer),
}
impl Layer for LayerHolder {
    fn render<S: glium::Surface>(&mut self, surface: &mut S) -> Result<()> {
        match self {
            LayerHolder::Background(b) => b.render(surface),
            LayerHolder::Raster(r) => r.render(surface),
            _ =>   {
                Ok(())
            }
        }
    }
}

use super::style::*;


pub fn parse_style_layers(style : &super::style::Style) -> Vec<LayerHolder> {
    let mut res = vec![];
    for l in style.layers.iter() {
        match l {
            StyleLayer::Background(l) => {
                res.push(LayerHolder::Background(background_layer::BackgroundLayer::parse(l.clone())))
            }
            _ => {

            }
        }
    }
    res
}