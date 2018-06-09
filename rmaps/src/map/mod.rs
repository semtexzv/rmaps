use prelude::*;

pub mod render;
pub mod layers;
pub mod style;
pub mod storage;

use map::layers::Layer;
use act::Actor;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

implement_vertex!(Vertex, position, color);

pub struct MapView {
    facade: Box<glium::backend::Facade>,
    style: Option<style::Style>,
    layers: Vec<layers::LayerHolder>,
    source: storage::FileSource,
}


impl MapView {
    pub fn new<F: glium::backend::Facade + Clone + 'static>(f: &F) -> Result<Self> {
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

        let source = storage::FileSource::new();

        //source.spawn_thread();
        return Ok(MapView {
            facade: Box::new((*f).clone()),
            style: None,
            layers: vec![],
            source: source,
        });
    }

    pub fn set_style(&mut self, style: style::Style) {
        self.layers.clear();
        self.layers = layers::parse_style_layers(self.facade.deref(), &style);
        println!("Layers : {:#?}", self.layers);
        self.style = Some(style);
    }
    pub fn render<S: glium::Surface>(&mut self, target: &mut S) {
        self.source.process_messages();
        for l in self.layers.iter_mut() {
            l.render(target);
        }
        let mut h = self.source.handle().clone();
        h.test();
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