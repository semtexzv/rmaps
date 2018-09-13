use prelude::*;

use map::{
    style,
    render::{
        self,
        layers::{
            self, Layer,
        },
        property::*,
    },
    tiles::{
        self
    },
};

#[derive(Debug,Clone, Copy, Vertex)]
pub struct RasterVertex {
    pub pos: [f32; 2],
    pub tex: [f32; 2],
}

#[derive(Debug)]
pub struct RasterBucket {
    pub texture: glium::texture::Texture2d,
    pub vbo: glium::VertexBuffer<RasterVertex>,
}

#[derive(Debug)]
pub struct RasterLayer {
    style_layer: style::RasterLayer,
    shader_program: glium::Program,
}

impl Layer for RasterLayer {
    fn new_tile(&mut self, display: &Display, data: &Rc<tiles::TileData>) -> Result<()> {
        Ok(())
    }

    fn prepare(&mut self, params: render::PrepareParams) -> Result<()> {
        unimplemented!()
    }


    fn evaluate(&mut self, params: &render::EvaluationParams) -> Result<()> {
        unimplemented!()
    }


    fn render(&mut self, params: &mut render::RenderParams) -> Result<()> {
        Ok(())
    }
}
/*
impl layers::Layer for RasterLayer {
    fn render_begin(&mut self, params: &mut RenderParams) {}

    fn eval_bucket(&mut self, params: &mut RenderParams, tile: TileCoords, bucket: &mut RenderBucket) -> Result<()> {
        Ok(())
    }


    fn render_tile(&mut self, params: &mut RenderParams, coord: TileCoords, bucket: &RenderBucket) -> Result<()> {
        if let RenderBucket::Raster(ref bucket) = bucket {
            let tile_matrix =  Mercator::tile_to_internal_matrix(&coord);
            let matrix = params.projection * params.view * tile_matrix ;
            let matrix: [[f32; 4]; 4] = matrix.into();
            let uniforms = uniform! {
                u_matrix : matrix,
                u_texture : &bucket.texture

            };

            let draw_params = glium::DrawParameters{
                blend: glium::Blend::alpha_blending(),
                .. Default::default()
            };

            (params.frame).draw(&bucket.vbo, glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan), &self.shader_program, &uniforms, &draw_params)?;
        }
        Ok(())
    }

    fn render_end(&mut self, params: &mut RenderParams) {}

    fn uses_source(&mut self, source: &str) -> bool {
        Some(source) == self.style_layer.common.source.as_ref().map(|x| x.deref())
    }

    fn create_bucket(&mut self, display: &Display, tile: &data::TileData) -> Result<RenderBucket> {
        match tile.data {
            data::DecodedTileData::Raster(data::RasterTileData { ref image, dims }) => {
                let raw = glium::texture::RawImage2d::from_raw_rgba_reversed(image, dims);

                let texture = glium::texture::Texture2d::new(display, raw).unwrap();
                let vertices = &[
                    RasterVertex {
                        pos: [0., 0.],
                        tex: [0., 1.],
                    },
                    RasterVertex {
                        pos: [EXTENT, 0.],
                        tex: [1., 1.],
                    },
                    RasterVertex {
                        pos: [EXTENT, EXTENT],
                        tex: [1., 0.],
                    },
                    RasterVertex {
                        pos: [0., EXTENT],
                        tex: [0., 0.],
                    },
                ];

                let vbo = glium::VertexBuffer::new(display, vertices).unwrap();
                return Ok(RenderBucket::Raster(RasterBucket {
                    texture,
                    vbo,
                }))
            }
            _ => {}
        }
        unimplemented!()
    }
}
*/

impl RasterLayer {
    pub fn parse(f: &glium::backend::Facade, layer: style::RasterLayer) -> Self {
        let shader_program = program!(f,
            100 es => {
                vertex: include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../shaders/raster_simple.vert.glsl")),
                fragment: include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../shaders/raster_simple.frag.glsl")),
            }
        ).unwrap();
        return RasterLayer {
            style_layer: layer,
            shader_program,
        };
    }
}
