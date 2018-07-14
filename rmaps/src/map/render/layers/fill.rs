use prelude::*;

use super::Vertex;
use map::{
    style,
    render::{
        self,
        layers::{
            self, Layer,
        },
        property::*,
    },
    tiles::data::{
        self
    },
};


#[derive(Debug, Clone, Properties)]
#[properties(FillLayer)]
pub struct FillLayerProperties {
    #[property(name = "antialias", nofeature)]
    antialias: BaseProperty<bool>,
}


#[derive(Debug, Clone, Default, Properties)]
#[properties(FillLayer)]
pub struct FillFeatureProperties {
    #[property(name = "opacity")]
    opacity: BaseProperty<f32>,
    #[property(name = "color")]
    color: BaseProperty<Color>,
}


#[repr(C)]
#[derive(Debug, Clone, Copy, Vertex, Default)]
pub struct FillVertexProperties {
    col: Color,
    opacity: f32,
}

#[derive(Debug)]
pub struct FeatureBucketData {
    pub feature: ::mvt::Feature,
    pub props: FillFeatureProperties,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub struct FillBucket {
    pub coord: TileCoords,
    pub features: BTreeMap<u64, FeatureBucketData>,

    pub indices: Vec<u16>,
    pub vertices: Vec<Vertex>,
    pub properties: Vec<FillVertexProperties>,

    pub pos_vbo: Option<VertexBuffer<Vertex>>,
    pub prop_vbo: Option<VertexBuffer<FillVertexProperties>>,
    pub last_ibo: Option<IndexBuffer<u16>>,

    pub dirty: bool,
}

impl layers::Bucket for FillBucket {
    fn upload(&mut self, display: &Display) -> Result<()> {
        if self.dirty {
            self.last_ibo = Some(IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &self.indices)?);
            self.pos_vbo = Some(VertexBuffer::new(display, &self.vertices)?);
            self.prop_vbo = Some(VertexBuffer::new(display, &self.properties)?);

            self.dirty = false;
        }
        Ok(())
    }
}

impl FillBucket {
    fn new(data: Rc<data::TileData>, source_layer: &str) -> Result<Option<Self>> {
        let mut features: BTreeMap<u64, FeatureBucketData> = BTreeMap::new();

        let mut vertices: Vec<Vertex> = vec![];
        let mut properties: Vec<_> = vec![];
        let mut indices: Vec<u16> = vec![];

        if let data::DecodedTileData::Vector(ref vec) = data.data {
            if let Some(layer) = vec.layers.iter().find(|x| &x.name == source_layer) {
                let mult = EXTENT as f32 / layer.extent as f32;

                for f in layer.features.iter() {
                    // TODO, perform filtering here
                    if f.typ == ::mvt::GeomType::Polygon {
                        let geom = &f.geom;

                        let g: Vec<Vec<[f32; 2]>> = f.geom.iter().map(|r| r.iter().map(|p| [p[0] as _, p[1] as _]).collect()).collect();

                        let zer = 0 as _;
                        let ext = layer.extent as _;
                        let sq = vec![vec![
                            [zer, zer],
                            [ext, zer],
                            [ext, ext],
                            [zer, ext],
                        ]];

                        if let Ok(res) = ::tess2::intersect(&g, &sq) {
                            let vertices_begin = vertices.len();
                            let indices_begin = indices.len();


                            for v in res.vertices.iter() {
                                vertices.push(Vertex {
                                    pos: [v[0] as f32 * mult, v[1] as f32 * mult]
                                })
                            }

                            //assert!(res.iter().all(|&i| i < vert_count && i >= 0));


                            for i in res.indices.iter() {
                                indices.push(vertices_begin as u16 + *i as u16);
                            }

                            features.insert(f.id, FeatureBucketData {
                                feature: f.clone(),
                                props: Default::default(),
                                start: vertices_begin,
                                end: vertices.len(),
                            });
                        }
                    }
                }


                return Ok(
                    Some(
                        FillBucket {
                            coord: data.coord,
                            features,

                            vertices,
                            properties,
                            indices,
                            dirty: true,

                            pos_vbo: None,
                            prop_vbo: None,
                            last_ibo: None,
                        }
                    )
                );
            }
        }

        return Ok(None);
    }
}


#[derive(Debug)]
pub struct FillLayer {
    style_layer: style::FillLayer,
    shader_program: glium::Program,
}

impl layers::BucketLayer for FillLayer {
    type Bucket = FillBucket;

    fn new_tile(&mut self, display: &Display, data: &Rc<data::TileData>) -> Result<Option<Self::Bucket>> {
        if (Some(&data.source) == self.style_layer.common.source.as_ref()) {
            if let Some(ref source_layer) = self.style_layer.common.source_layer {
                return Ok(FillBucket::new(data.clone(), &source_layer)?);
            }
        }

        Ok(None)
    }

    fn eval_bucket(&mut self, params: &render::EvaluationParams, bucket: &mut Self::Bucket) -> Result<()> {
        for (id, data) in bucket.features.iter_mut() {
            let should = if bucket.properties.is_empty() {
                bucket.properties.resize(bucket.vertices.len(), Default::default());
                true
            } else {
                false
            };

            let evaluator = PropertiesEvaluator::only_zoom(params.zoom).with_feature(&data.feature);
            data.props.eval(&self.style_layer, &evaluator)?;
            bucket.dirty = true;

            let prop = FillVertexProperties {
                col: data.props.color.get(),
                opacity: data.props.opacity.get(),
            };

            for p in bucket.properties[data.start..data.end].iter_mut() {
                *p = prop;
            }
        }
        Ok(())
    }

    fn render_bucket(&mut self, params: &mut render::RenderParams, bucket: &Self::Bucket) -> Result<()> {
        //println!("{:?}", ::std::mem::size_of::<FillVertexProperties>());
        let tile_matrix = Mercator::tile_to_internal_matrix(&bucket.coord);
        let matrix = params.projection * params.view * tile_matrix;
        let matrix: [[f32; 4]; 4] = matrix.into();
        let u_t: [f32; 4] = Default::default();
        let uniforms = uniform! {
                u_matrix : matrix,
                u_t : u_t,

            };

        let draw_params = glium::DrawParameters {
            blend: glium::Blend::alpha_blending(),
            ..Default::default()
        };

        let buffers = (bucket.pos_vbo.as_ref().unwrap(), bucket.prop_vbo.as_ref().unwrap());
        let indices = bucket.last_ibo.as_ref().unwrap();

        (params.frame).draw(buffers, indices, &self.shader_program, &uniforms, &draw_params)?;

        Ok(())
    }
}


/*
impl layers::Layer for FillLayer {
    fn render_begin(&mut self, params: &mut render::RenderParams) {}

    fn eval_bucket(&mut self, params: &mut render::RenderParams, tile: TileCoords, bucket: &mut render::RenderBucket) -> Result<()> {
        if let render::RenderBucket::Fill(ref mut bucket) = bucket {
            let mut modified = false;
            for (id, data) in bucket.features.iter_mut() {
                let evaluator = PropertiesEvaluator::only_zoom(params.zoom);
                if data.props.eval(&self.style_layer, &evaluator)? || true {
                    modified = true;

                    for p in bucket.properties[data.start..data.end].iter_mut() {
                        *p = FillVertexProperties {
                            col: data.props.color.get(),
                            opacity: data.props.opacity.get(),
                        };
                    }
                }
            }
            if (modified) {
                bucket.prop_vbo = glium::VertexBuffer::new(params.disp, &bucket.properties).unwrap();
            }
        }

        Ok(())
    }


    fn render_tile(&mut self, params: &mut render::RenderParams, coord: TileCoords, bucket: &render::RenderBucket) -> Result<()> {
        if let render::RenderBucket::Fill(ref bucket) = bucket {
            //println!("{:?}", ::std::mem::size_of::<FillVertexProperties>());
            let tile_matrix = Mercator::tile_to_internal_matrix(&coord);
            let matrix = params.projection * params.view * tile_matrix;
            let matrix: [[f32; 4]; 4] = matrix.into();
            let u_t: [f32; 4] = Default::default();
            let uniforms = uniform! {
                u_matrix : matrix,
                u_t : u_t,

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
        let mut index_ranges: BTreeMap<u64, FeatureBucketData> = BTreeMap::new();


        if let data::DecodedTileData::Vector(ref vec) = data.data {
            if let Some(layer) = vec.layers.iter().find(|x| Some(&x.name) == self.style_layer.common.source_layer.as_ref()) {
                let mult = EXTENT as f32 / layer.extent as f32;

                for f in layer.features.iter() {
                    let mut tess = ::tess2::Tessellator::new();
                    if f.typ == ::mvt::GeomType::Polygon {
                        let polys = &f.geom;
                        for ring in polys.iter() {
                            tess.add_poly(ring.iter()).unwrap();
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

                            index_ranges.insert(f.id, FeatureBucketData {
                                feature: f.clone(),
                                props: Default::default(),
                                start: vertices_begin,
                                end: vertices_begin + res.vertices.len(),
                            });
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
*/

impl FillLayer {
    pub fn parse(f: &glium::backend::Facade, layer: style::FillLayer) -> Self {
        let shader_program = rmaps_program!(f,"fill");

        FillLayer {
            style_layer: layer,
            shader_program: shader_program.unwrap(),
        }
    }
}