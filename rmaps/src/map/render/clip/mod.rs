use ::prelude::*;

use ::map::render::layers::FeatureVertex;

pub struct Clipper {
    program: Rc<Program>,
}

impl Clipper {
    pub fn new(display: &Display) -> Result<Self> {
        Ok(Clipper {
            program: layer_program!(display,"clipper", &Default::default(), &Default::default()).unwrap()
        })
    }
    pub fn apply_mask(&self, cover : &TileCover, params : &mut super::RenderParams) -> Result<()> {
        use glium::draw_parameters::*;

        fn gen_params<'a>(tile: TileCoords) -> glium::DrawParameters<'a> {
            glium::DrawParameters {
                stencil: Stencil {
                    test_clockwise: StencilTest::AlwaysPass,
                    test_counter_clockwise: StencilTest::AlwaysPass,

                    depth_pass_operation_clockwise: StencilOperation::Replace,
                    depth_pass_operation_counter_clockwise: StencilOperation::Replace,

                    pass_depth_fail_operation_clockwise : StencilOperation::Replace,
                    pass_depth_fail_operation_counter_clockwise : StencilOperation::Replace,

                    reference_value_clockwise: tile.id() as _,
                    reference_value_counter_clockwise: tile.id() as _,
                    ..Default::default()
                },
                ..Default::default()
            }
        }

        for coords in cover.tiles().iter() {
            let vertices = &[
                FeatureVertex {
                    pos: [0., 0.],
                    feature: 0,
                },
                FeatureVertex {
                    pos: [EXTENT, 0.],
                    feature: 0,
                },
                FeatureVertex {
                    pos: [EXTENT, EXTENT],
                    feature: 0,
                },

                FeatureVertex {
                    pos: [0.,EXTENT],
                    feature: 0,
                },
            ];

            let vbo = glium::VertexBuffer::new(params.display, vertices).unwrap();

            let tile_matrix = Mercator::tile_to_world(*coords);

            let matrix = params.camera.projection() * params.camera.view() * tile_matrix;
            let matrix: [[f32; 4]; 4] = matrix.into();

            let uniforms = uniform! {
                u_matrix : matrix,
            };

            let draw_params = glium::DrawParameters {
                ..gen_params(coords.wrap())
            };

            let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);


            (params.frame).draw(&vbo, indices, &self.program, &uniforms, &draw_params)?;
        }
        Ok(())
    }
}
