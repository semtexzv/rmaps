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

pub mod props;
pub mod bucket;

use self::props::{FillFeatureProperties, FillLayerProperties};
use self::bucket::*;


#[derive(Debug)]
pub struct FillLayer {
    style_layer: style::FillLayer,
    pub shader_program: glium::Program,
    pub pattern_program: Option<glium::Program>,
    pub properties: FillLayerProperties,
    pub layout: (UniformPropertyLayout, FeaturePropertyLayout),
}

impl layers::WithSource for FillLayer {
    fn source_name(&self) -> Option<&str> {
        self.style_layer.common.source.as_ref().map(Deref::deref)
    }
}

impl layers::BucketLayer for FillLayer {
    type Bucket = FillBucket;

    fn new_tile(&mut self, display: &Display, data: &Rc<data::TileData>) -> Result<Option<Self::Bucket>> {
        if (Some(&data.source) == self.style_layer.common.source.as_ref()) {
            if let Some(ref source_layer) = self.style_layer.common.source_layer {
                return Ok(FillBucket::new(display, data.clone(), &source_layer)?);
            }
        }

        Ok(None)
    }

    fn eval_layer(&mut self, params: &render::EvaluationParams) -> Result<()> {
        let evaluator = PropertiesEvaluator::only_zoom(params.zoom);
        self.properties.eval(&self.style_layer, &evaluator)?;

        // println!("Props : {:?}", self.properties);
        Ok(())
    }


    fn eval_bucket(&mut self, params: &render::EvaluationParams, bucket: &mut Self::Bucket) -> Result<()> {
        let evaluator = PropertiesEvaluator::only_zoom(params.zoom);
        bucket.properties.eval(&self.style_layer, &evaluator)?;

        UniformPropertyBinder::bind(&self.layout.0, &bucket.properties, &self.style_layer, &mut bucket.uniforms)?;

        bucket.feature_data.clear();

        let features = &mut bucket.features;

        let _: Result<()> = FeaturePropertyBinder::with(&self.layout.1, &mut bucket.feature_data, |binder| {
            for (id, data) in features.iter_mut() {
                let evaluator = PropertiesEvaluator::only_zoom(params.zoom).with_feature(&data.feature);
                data.props.eval(&self.style_layer, &evaluator)?;

                data.props.accept(&self.style_layer, binder);
            }
            Ok(())
        });

        bucket.eval_dirty = false;
        bucket.upload_dirty = true;
        Ok(())
    }

    fn render_bucket(&mut self, params: &mut render::RenderParams, coord: UnwrappedTileCoords, bucket: &Self::Bucket) -> Result<()> {
        let tile_matrix = Mercator::tile_to_world(coord);
        let matrix = params.camera.projection() * params.camera.view() * tile_matrix;
        let matrix: [[f32; 4]; 4] = matrix.into();
        let u_t: [f32; 4] = Default::default();


        if let Some(pattern) = self.properties.pattern.get() {

            if let Some((sprite, texture)) = params.atlas.get_sprite_data(&pattern) {
                let tl = (sprite.x as f32, sprite.y as f32);
                let br = (tl.0 + sprite.width as f32, tl.1 + sprite.height as f32);
                let a = uniform! {
                    u_matrix : matrix,
                    feature_data_ubo :  &bucket.feature_data.data,
                    u_image : texture,
                    u_tex_scale : 128f32,
                    u_pattern_tl : [sprite.x,sprite.y],
                    u_pattern_br : [50f32,50f32],
                    u_texsize : (300f32,300f32),

                };
                let mut uniforms = MergeUniforms(
                    &bucket.uniforms,
                    &a,
                );
                let draw_params = glium::DrawParameters {
                    blend: glium::Blend::alpha_blending(),
                    ..Default::default()
                };

                let buffers = bucket.pos_vbo.as_ref().unwrap();
                let indices = bucket.last_ibo.as_ref().unwrap();

                (params.frame).draw(buffers, indices, self.pattern_program.as_ref().unwrap(), &uniforms, &draw_params)?;
            } else {
                error!("Pattern icon : {} not found",pattern);
            }
            // render as pattern
        } else {
            // Render as Color fill

            let a = uniform! {
                u_matrix : matrix,
                feature_data_ubo :  &bucket.feature_data.data,
            };
            let mut uniforms = MergeUniforms(
                &bucket.uniforms,
                &a,
            );

            let draw_params = glium::DrawParameters {
                blend: glium::Blend::alpha_blending(),
                ..Default::default()
            };

            let buffers = bucket.pos_vbo.as_ref().unwrap();
            let indices = bucket.last_ibo.as_ref().unwrap();

             (params.frame).draw(buffers, indices, &self.shader_program, &uniforms, &draw_params)?;
        }

        Ok(())
    }
}

impl FillLayer {
    pub fn parse(f: &glium::backend::Facade, layer: style::FillLayer) -> Self {
        let (uni, feat) = ::map::render::property::PropertyLayoutBuilder::build::<FillFeatureProperties>(&layer);
        //println!("Style : {:?}", layer);

        let pattern_program = layer.paint.pattern.as_ref().map(|_| {
            layer_program!(f,"fill-pattern", &uni, &feat).unwrap()
        });

        //trace!("Fill layer layout:\n  uniforms: {:?},\n  features: {:?}", uni, feat);
        let shader_program = layer_program!(f,"fill", &uni, &feat);

        FillLayer {
            layout: (uni, feat),
            style_layer: layer,
            properties: Default::default(),
            shader_program: shader_program.unwrap(),
            pattern_program,
        }
    }
}