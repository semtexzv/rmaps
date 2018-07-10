use prelude::*;

use map::style;
use map::layers;
use map::tiles::data;
use map::render;
use super::*;
use super::property::*;


#[derive(Debug, Clone, Properties)]
#[properties(FillLayer)]
pub struct FillLayerProperties {
    #[property(name = "antialias", nofeature)]
    antialias: BaseProperty<bool>,
    #[property(layout, nozoom, nofeature)]
    visibility: BaseProperty<style::Visibility>,
}


/*
#[derive(Debug, Clone, FeatureProperties)]
#[properties(BackgroundLayer)]
pub struct FillFeatureProperties {
    #[property(name = "opacity")]
    opacity: BaseProperty<f32>,
    #[property(nozoom, nofeature)]
    color: BaseProperty<Color>,
}
*/

#[repr(C)]
#[derive(Debug, Clone, Copy, Vertex, Default)]
pub struct FillVertexProperties {
    col: Color,
    opacity: f32,
}


#[derive(Debug)]
pub struct FillBucket {
    pub features: BTreeMap<u64, (usize, usize)>,

    pub vertices: Vec<Vertex>,
    pub properties: Vec<FillVertexProperties>,

    pub pos_vbo: VertexBuffer<Vertex>,
    pub prop_vbo: VertexBuffer<FillVertexProperties>,
    pub last_ibo: IndexBuffer<u16>,
}

#[derive(Debug)]
pub struct FillLayer {
    style_layer: style::FillLayer,
    shader_program: glium::Program,
}

impl layers::Layer for FillLayer {
    fn render_begin(&mut self, params: &mut render::RenderParams) {}

    fn render_tile(&mut self, params: &mut render::RenderParams, coord: TileCoords, bucket: &render::RenderBucket) -> Result<()> {
        if let render::RenderBucket::Fill(ref bucket) = bucket {
            let tile_matrix = Mercator::tile_to_internal_matrix(&coord);
            let matrix = params.projection * params.view * tile_matrix;
            let matrix: [[f32; 4]; 4] = matrix.into();
            let uniforms = uniform! {
                u_matrix : matrix,

            };

            let draw_params = glium::DrawParameters {
                blend: glium::Blend::alpha_blending(),
                ..Default::default()
            };

            (params.frame).draw((&bucket.pos_vbo, &bucket.prop_vbo), &bucket.last_ibo, &self.shader_program, &uniforms, &draw_params)?;
        }
        Ok(())
    }

    fn render_end(&mut self, params: &mut render::RenderParams) {}

    fn uses_source(&mut self, source: &str) -> bool {
        Some(source) == self.style_layer.common.source.as_ref().map(|x| x.deref())
    }


    fn create_bucket(&mut self, display: &Display, data: &data::TileData) -> Result<render::RenderBucket> {
        let mut vertices: Vec<Vertex> = vec![];
        let mut indices = vec![];
        let mut index_ranges: BTreeMap<u64, (usize, usize)> = BTreeMap::new();


        if let data::DecodedTileData::Vector(ref vec) = data.data {
            if let Some(layer) = vec.layers.iter().find(|x| Some(&x.name) == self.style_layer.common.source_layer.as_ref()) {
                let mult = EXTENT as f32 / layer.extent as f32;

                for f in layer.features.iter() {
                    let mut tess = ::tess2::Tessellator::new();
                    if f.typ == ::mapbox_tiles::GeomType::Polygon {
                        let polys = &f.geom;
                        for ring in polys.iter() {
                            tess.add_poly(ring.iter());
                        }
                        if let Ok(res) = tess.tessellate_nonzero() {
                            let vertices_begin = vertices.len();
                            let indices_begin = indices.len();

                            for v in res.vertices.iter() {
                                vertices.push(Vertex {
                                    pos: [v[0] * mult, v[1] * mult]
                                })
                            }

                            for i in res.indices.iter() {
                                indices.push(vertices_begin as u16 + *i as u16);
                            }

                            index_ranges.insert(f.id, (indices_begin, res.indices.len()));
                        }
                    }
                }

                let mut properties: Vec<FillVertexProperties> = vertices.iter().map(|v| FillVertexProperties::default()).collect();


                return Ok(
                    render::RenderBucket::Fill(
                        FillBucket {
                            features: index_ranges,
                            pos_vbo: glium::VertexBuffer::new(display, &vertices).unwrap(),
                            prop_vbo: glium::VertexBuffer::new(display, &properties).unwrap(),

                            vertices,
                            properties,

                            last_ibo: glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap(),
                        }
                    )
                );
            }
        }

        return Ok(render::RenderBucket::NoOp);
    }
}


impl FillLayer {
    pub fn parse(f: &glium::backend::Facade, layer: style::FillLayer) -> Self {
        let shader_program = rmaps_program!(f,"fill");

        FillLayer {
            style_layer: layer,
            shader_program: shader_program.unwrap(),
        }
    }
}