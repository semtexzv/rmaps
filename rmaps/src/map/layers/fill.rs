use prelude::*;

use ::map::style;

#[derive(Debug)]
pub struct FillLayer {
    style_layer: style::FillLayer,
    shader_program: glium::Program,
}

impl super::Layer for FillLayer {
    fn render<S: glium::Surface>(&mut self, surface: &mut S) -> Result<()> {
        unimplemented!()
    }
}

impl FillLayer {
    pub fn parse(f : &glium::backend::Facade, layer: style::FillLayer) -> Self {

        let mut shader_program = program!(f,
            100 es => {
                vertex: include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../shaders/fill.vert.glsl")),
                fragment: include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../shaders/fill.frag.glsl")),
            }
        );

        FillLayer {
            style_layer: layer,
            shader_program : shader_program.unwrap(),
        }
    }
}