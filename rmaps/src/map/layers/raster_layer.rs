use prelude::*;

#[derive(Debug)]
pub struct RasterLayer {

}

impl super::Layer for RasterLayer {

    fn render<S: glium::Surface>(&mut self, surface: &mut S) -> Result<()> {
        unimplemented!()
    }
}