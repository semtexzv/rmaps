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


use map::render::shaders::{
    UniformPropertyLayout,
    FeaturePropertyLayout,
    PropertyItemLayout,
};
use map::render::property::MergeUniforms;

#[derive(Debug, Clone, Default, Properties)]
#[properties(FillLayer)]
pub struct FillLayerProperties {
    #[property(src_name = "antialias", nofeature)]
    antialias: BaseProp<bool>,
}


#[derive(Debug, Clone, Default, Properties)]
#[properties(FillLayer)]
pub struct FillFeatureProperties {
    #[property(src_name = "opacity")]
    opacity: GpuProp<f32>,
    #[property(src_name = "color")]
    color: GpuProp<Color>,

    #[property(src_name = "translate_anchor", nofeature)]
    anchor: BaseProp<Option<String>>,
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

    pub properties: FillFeatureProperties,
    pub uniforms: UniformPropertyData,

    pub pos_vbo: Option<VertexBuffer<Vertex>>,
    pub last_ibo: Option<IndexBuffer<u16>>,

    pub eval_dirty: bool,
    pub upload_dirty: bool,
}

impl layers::Bucket for FillBucket {
    fn upload(&mut self, display: &Display) -> Result<()> {
        if self.upload_dirty {
            self.last_ibo = Some(IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &self.indices)?);
            self.pos_vbo = Some(VertexBuffer::new(display, &self.vertices)?);

            self.upload_dirty = false;
        }
        Ok(())
    }
}

impl FillBucket {
    fn new(data: Rc<data::TileData>, source_layer: &str) -> Result<Option<Self>> {
        let mut features: BTreeMap<u64, FeatureBucketData> = BTreeMap::new();

        let mut vertices: Vec<Vertex> = vec![];
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


                return Ok(
                    Some(
                        FillBucket {
                            coord: data.coord,
                            features,
                            properties: Default::default(),
                            uniforms: Default::default(),

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

#[derive(Debug)]
pub struct FillLayer {
    style_layer: style::FillLayer,
    shader_program: glium::Program,
    layout: (UniformPropertyLayout, FeaturePropertyLayout),
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
        let evaluator = PropertiesEvaluator::only_zoom(params.zoom);
        bucket.properties.eval(&self.style_layer, &evaluator)?;

        UniformPropertyBinder::bind(&self.layout.0, &bucket.properties, &self.style_layer, &mut bucket.uniforms)?;
        /*
        for (id, data) in bucket.features.iter_mut() {
            let evaluator = PropertiesEvaluator::only_zoom(params.zoom).with_feature(&data.feature);
            data.props.eval(&self.style_layer, &evaluator)?;


            let prop = FillVertexProperties {
                col: data.props.color.get(),
                opacity: data.props.opacity.get(),
            };

            for p in bucket.properties[data.start..data.end].iter_mut() {
                *p = prop;
            }
        }
        */
        bucket.eval_dirty = false;
        bucket.upload_dirty = true;
        Ok(())
    }

    fn render_bucket(&mut self, params: &mut render::RenderParams, bucket: &Self::Bucket) -> Result<()> {
        //println!("{:?}", ::std::mem::size_of::<FillVertexProperties>());
        let tile_matrix = Mercator::tile_to_world(&bucket.coord);
        let matrix = params.projection * params.view * tile_matrix;
        let matrix: [[f32; 4]; 4] = matrix.into();
        let u_t: [f32; 4] = Default::default();

        let a = uniform! {
            u_matrix : matrix,

        };
        let mut uniforms = MergeUniforms(
            &bucket.uniforms,
            &a
        );

        let draw_params = glium::DrawParameters {
            blend: glium::Blend::alpha_blending(),
            ..Default::default()
        };

        let buffers = bucket.pos_vbo.as_ref().unwrap();
        let indices = bucket.last_ibo.as_ref().unwrap();

        (params.frame).draw(buffers, indices, &self.shader_program, &uniforms, &draw_params)?;

        Ok(())
    }
}

impl FillLayer {
    pub fn parse(f: &glium::backend::Facade, layer: style::FillLayer) -> Self {
        let (uni, feat) = ::map::render::property::PropertyLayoutBuilder::build::<FillFeatureProperties>(&layer);
        trace!("Fill layer layout:\n  uniforms: {:?},\n  features: {:?}", uni, feat);
        let shader_program = layer_program!(f,"fill",&uni,&feat);

        FillLayer {
            layout: (uni, feat),
            style_layer: layer,
            shader_program: shader_program.unwrap(),
        }
    }
}