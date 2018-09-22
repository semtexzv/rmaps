use ::prelude::*;
use map::render::layers::{
    self, Vertex,
};
use map::render::property::*;

use map::tiles;

use super::props::*;

#[derive(Debug)]
pub struct FeatureBucketData {
    pub feature: ::mvt::Feature,
    pub props: FillFeatureProperties,
    pub start: usize,
    pub end: usize,
}


#[derive(Debug)]
pub struct FillBucket {
    pub features: BTreeMap<u64, FeatureBucketData>,

    pub indices: Vec<u16>,
    pub vertices: Vec<Vertex>,

    pub properties: FillFeatureProperties,
    pub uniforms: UniformPropertyData,
    pub feature_data: FeaturePropertyData,

    pub pos_vbo: Option<VertexBuffer<Vertex>>,
    pub last_ibo: Option<IndexBuffer<u16>>,

    pub eval_dirty: bool,
    pub upload_dirty: bool,
}


impl FillBucket {
    pub fn new(d: &Display, data: Rc<tiles::TileData>, layer_common: &::map::style::LayerCommon) -> Result<Option<Self>> {
        let mut features: BTreeMap<u64, FeatureBucketData> = BTreeMap::new();

        let mut vertices: Vec<Vertex> = vec![];
        let mut indices: Vec<u16> = vec![];

        let source_layer = layer_common.source_layer.as_ref().map(|x| x.as_str()).unwrap_or("");

        let vec = data.data.unwrap_vector();

        if let Some(layer) = vec.layers.iter().find(|x| &x.layer.name == source_layer) {
            for (idx, f) in layer.layer.features.iter().enumerate()
                .filter(|(idx, feature)| {
                    ::map::style::filter::FilterEvaluator::satisfies_opt(feature, &layer_common.filter)
                }) {
                if f.typ == ::mvt::GeomType::Polygon {
                    if let Some(g) = layer.pre_tesselated.get(&idx) {
                        let vertices_begin = vertices.len();
                        let indices_begin = indices.len();

                        for v in g.vertices.iter() {
                            vertices.push(Vertex {
                                pos: [v[0] as f32, v[1] as f32],
                                feature: features.len() as u16,
                            })
                        }

                        assert!(vertices.len() < u16::max_value() as usize, "Layer too big : {:?}", layer);
                        for i in g.indices.iter() {
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
                    }
                )
            );
        }


        return Ok(None);
    }
}


impl layers::Bucket for FillBucket {
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