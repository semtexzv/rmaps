use ::prelude::*;
use map::{
    render::{
        layers::{
            self, FeatureVertex, FeatureBucketData,
        },
        property::*,
    },
    tiles,
};
use super::props::*;

#[derive(Debug, Clone, Copy, Vertex)]
pub struct LineVertex {
    #[glium(attr = "pos")]
    pub pos: [f32; 2],
    #[glium(attr = "feature")]
    pub feature: u16,
    #[glium(attr = "normal")]
    normal: [f32; 2],
}

#[derive(Debug)]
pub struct LineBucket {
    pub features: BTreeMap<u64, FeatureBucketData<LineFeatureProperties>>,

    pub indices: Vec<u16>,
    pub vertices: Vec<LineVertex>,

    pub properties: LineFeatureProperties,
    pub uniforms: UniformPropertyData,
    pub feature_data: FeaturePropertyData,

    pub pos_vbo: Option<VertexBuffer<LineVertex>>,
    pub last_ibo: Option<IndexBuffer<u16>>,

    pub eval_dirty: bool,
    pub upload_dirty: bool,
}

impl LineBucket {
    pub fn new(d: &Display, data: Rc<tiles::TileData>, layer_common: &::map::style::LayerCommon) -> Result<Option<Self>> {
        let mut features: BTreeMap<u64, FeatureBucketData<LineFeatureProperties>> = BTreeMap::new();

        let mut vertices = vec![];
        let mut indices: Vec<u16> = vec![];

        let source_layer = layer_common.source_layer.as_ref().map(|x| x.as_str()).unwrap_or("");

        let vec = data.data.unwrap_vector();

        let layer = vec.layers.iter().find(|x| &x.layer.name == source_layer);
        let layer = match layer {
            Some(l) => l,
            None => return Ok(None)
        };

        for (idx, f) in layer.layer.features.iter().enumerate()
            .filter(|(idx, feature)| {
                ::map::style::filter::FilterEvaluator::satisfies_opt(feature, &layer_common.filter)
            }) {
            if f.typ == ::mvt::GeomType::LineString {
                let g = &f.geometry;

                let vertices_begin = vertices.len();
                let indices_begin = indices.len();


                let mut num_vertices = 0;
                for l in g.iter() {
                    for v in l.windows(2) {
                        if let [a, b] = v {
                            let dir = cgmath::vec2(b[0] - a[0], b[1] - a[1]);
                            let dir = dir / dir.magnitude();

                            let n1 = [-dir.y, dir.x];
                            let n2 = [-n1[0], -n1[1]];

                            vertices.push(LineVertex {
                                pos: *a,
                                feature: features.len() as u16,
                                normal: n1,
                            });
                            vertices.push(LineVertex {
                                pos: *a,
                                feature: features.len() as u16,
                                normal: n2,
                            });
                            vertices.push(LineVertex {
                                pos: *b,
                                feature: features.len() as u16,
                                normal: n1,
                            });
                            vertices.push(LineVertex {
                                pos: *b,
                                feature: features.len() as u16,
                                normal: n2,
                            });
                        }
                        let current = (vertices_begin + num_vertices) as u16;
                        indices.push(current);
                        indices.push(current + 2);
                        indices.push(current + 1);

                        indices.push(current + 1);
                        indices.push(current + 2);
                        indices.push(current + 3);
                        num_vertices += 4;
                    }
                }


                features.insert(f.id, FeatureBucketData {
                    feature: f.clone(),
                    props: Default::default(),
                    start: vertices_begin,
                    end: vertices.len(),
                });
            }
        }


        return Ok(
            Some(LineBucket {
                features,
                properties: Default::default(),
                uniforms: Default::default(),
                feature_data: FeaturePropertyData::new(d)?,

                vertices,
                indices,
                eval_dirty: true,
                upload_dirty: true,

                pos_vbo: None,
                last_ibo: None,
            }));
    }
}

impl layers::Bucket for LineBucket {
    fn upload(&mut self, display: &Display) -> Result<()> {
        if self.upload_dirty {
            if self.last_ibo.is_none() {
                self.last_ibo = Some(IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &self.indices)?);
            }
            if self.pos_vbo.is_none() {
                self.pos_vbo = Some(VertexBuffer::new(display, &self.vertices)?);
            }

            //self.feature_data.upload(display)?;

            self.upload_dirty = false;
        }
        Ok(())
    }
}