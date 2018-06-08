use prelude::*;

pub mod layers;
pub mod style;

use map::layers::Layer;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

implement_vertex!(Vertex, position, color);

pub struct MapView {
    style: Option<style::Style>,
    layers : Vec<layers::LayerHolder>
}

impl MapView {
    pub fn new<F: glium::backend::Facade>(f: &F) -> Result<Self> {
        let vbo = VertexBuffer::new(f, &[
            Vertex { position: [-0.5, -0.5], color: [0.0, 1.0, 0.0] },
            Vertex { position: [0.0, 0.5], color: [0.0, 0.0, 1.0] },
            Vertex { position: [0.5, -0.5], color: [1.0, 0.0, 0.0] },
        ])?;

        let ibo = glium::IndexBuffer::new(f, PrimitiveType::TrianglesList,
                                          &[0u16, 1, 2])?;

        let program = program!(f,
            100 es => {
                vertex: include_str!("../../../shaders/raster_simple.vert.glsl"),
                fragment: include_str!("../../../shaders/raster_simple.frag.glsl"),
            }
        )?;

        return Ok(MapView {
            style: None,
            layers : vec![]

        });
    }

    pub fn set_style(&mut self, style: style::Style) {
        self.layers.clear();
        self.layers = layers::parse_style_layers(&style);
        println!("Layers : {:#?}", self.layers);
        self.style = Some(style);
    }
    pub fn render<S: glium::Surface>(&mut self, target: &mut S) {

        for l in self.layers.iter_mut() {
            l.render(target);
        }
        /*
        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ]
        };

        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target.draw(&self.vbo, &self.ibo, &self.program, &uniforms, &Default::default()).unwrap();


        //surface.clear(None, Some((1f32, 0.0, 0.0, 0.0)), false, Some(0f32), Some(0));
        */
    }
}