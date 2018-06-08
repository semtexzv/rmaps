use prelude::*;

pub mod background;
pub mod raster;
pub mod fill;

pub trait Layer : Sized + Debug {
    fn render<S: glium::Surface>(&mut self, surface: &mut S) -> Result<()>;
}



#[derive(Debug)]
pub enum LayerHolder{
    Background(background::BackgroundLayer),
    Raster(raster::RasterLayer),
    Fill(fill::FillLayer)
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


pub fn parse_style_layers(facade : &glium::backend::Facade, style : &super::style::Style) -> Vec<LayerHolder> {
    let mut res = vec![];
    for l in style.layers.iter() {
        match l {
            StyleLayer::Background(l) => {
                res.push(LayerHolder::Background(background::BackgroundLayer::parse(l.clone())))
            },
            StyleLayer::Fill(l) => {
                res.push(LayerHolder::Fill(fill::FillLayer::parse(facade,l.clone())))
            }
            StyleLayer::Raster(l) => {
                res.push(LayerHolder::Raster(raster::RasterLayer::parse(l.clone())))
            }
            _ => {

            }
        }
    }
    res
}