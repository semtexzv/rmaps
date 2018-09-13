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
    pub fn new(d: &Display, data: Rc<tiles::TileData>, source_layer: &str) -> Result<Option<Self>> {
        let mut features: BTreeMap<u64, FeatureBucketData> = BTreeMap::new();

        let mut vertices: Vec<Vertex> = vec![];
        let mut indices: Vec<u16> = vec![];

        if let tiles::DecodedTileData::Vector(ref vec) = data.data {
            if let Some(layer) = vec.layers.iter().find(|x| &x.layer.name == source_layer) {
                for f in layer.layer.features.iter() {
                    // TODO, perform filtering here
                    if f.typ == ::mvt::GeomType::Polygon {
                        if let Some(g) = layer.pre_tesselated.get(&f.id) {
                            let vertices_begin = vertices.len();
                            let indices_begin = indices.len();

                            for v in g.vertices.iter() {
                                vertices.push(Vertex {
                                    pos: [v[0] as f32, v[1] as f32],
                                    feature: features.len() as u16,
                                })
                            }

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

                /*
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
                                    pos: [v[0] as f32 * mult, v[1] as f32 * mult],
                                    feature: features.len() as u16,
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
                    */
                //info!("VERTICES : {:?}", vertices);

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