use prelude::*;

use super::FeatureVertex;
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
    pub shader_program: Rc<glium::Program>,
    pub pattern_program: Option<Rc<glium::Program>>,
    pub properties: FillLayerProperties,
    pub layout: (UniformPropertyLayout, FeaturePropertyLayout),
}

impl layers::WithSource for FillLayer {
    fn source_name(&self) -> Option<&str> {
        self.style_layer.common.source.as_ref().map(Deref::deref)
    }
}

impl layers::LayerNew for FillLayer {
    type StyleLayer = style::FillLayer;

    fn new(facade: &Display, style_layer: &<Self as layers::LayerNew>::StyleLayer) -> Self {
        let (uni, feat) = ::map::render::property::PropertyLayoutBuilder::build::<FillFeatureProperties>(style_layer);

        let pattern_program = style_layer.paint.pattern.as_ref().map(|_| {
            layer_program!(facade,"fill-pattern", &uni, &feat).unwrap()
        });

        let shader_program = layer_program!(facade,"fill", &uni, &feat);

        FillLayer {
            layout: (uni, feat),
            style_layer: style_layer.clone(),
            properties: Default::default(),
            shader_program: shader_program.unwrap(),
            pattern_program,
        }
    }
}

impl layers::BucketLayer for FillLayer {
    type Bucket = FillBucket;

    fn new_tile(&mut self, display: &Display, data: &Rc<tiles::TileData>) -> Result<Option<Self::Bucket>> {
        if (Some(&data.source) == self.style_layer.common.source.as_ref()) {
            if let Some(ref source_layer) = self.style_layer.common.source_layer {
                return Ok(FillBucket::new(display, data.clone(), &self.style_layer.common)?);
            }
        }

        Ok(None)
    }

    fn eval_layer(&mut self, params: &render::EvaluationParams) -> Result<()> {
        let mut evaluator = PropertiesEvaluator::only_zoom(params.zoom);
        self.properties.accept_mut(&self.style_layer, &mut evaluator);

        Ok(())
    }


    fn eval_bucket(&mut self, params: &render::EvaluationParams, bucket: &mut Self::Bucket) -> Result<()> {
        let mut evaluator = PropertiesEvaluator::only_zoom(params.zoom);
        bucket.properties.accept_mut(&self.style_layer, &mut evaluator);

        if evaluator.modified {
            UniformPropertyBinder::rebind(&self.layout.0, &bucket.properties, &self.style_layer, &mut bucket.uniforms)?;
        }

        let features = &mut bucket.features;

        for (id, data) in features.iter_mut() {
            let mut evaluator = PropertiesEvaluator::new(params.zoom, &data.feature);
            data.props.accept_mut(&self.style_layer, &mut evaluator);
            bucket.upload_dirty |= evaluator.modified;
        }

        if bucket.upload_dirty {
            bucket.feature_data.clear();
            FeaturePropertyBinder::with(&self.layout.1, &mut bucket.feature_data, |binder| {
                for (id, data) in features.iter_mut() {
                    data.props.accept(&self.style_layer, binder);
                }
            });
        }

        bucket.eval_dirty = false;
        bucket.upload_dirty = true;
        Ok(())
    }

    fn render_bucket(&mut self, params: &mut render::RenderParams, coord: UnwrappedTileCoords, bucket: &Self::Bucket) -> Result<()> {
        let tile_matrix = Mercator::tile_to_world(coord);
        let matrix = params.camera.projection() * params.camera.view() * tile_matrix;
        let matrix: [[f32; 4]; 4] = matrix.into();

        if let Some(pattern) = self.properties.pattern.get() {
            if let Some((sprite, texture)) = params.atlas.get_pattern(&pattern) {
                use self::glium::uniforms::*;

                let sampler: Sampler<_> = texture.sampled();
                sampler
                    .magnify_filter(MagnifySamplerFilter::Nearest)
                    .minify_filter(MinifySamplerFilter::Nearest);

                let texsize = params.atlas.atlas_dims();

                let a = uniform! {
                    u_matrix : matrix,
                    feature_data_ubo :  &bucket.feature_data.data,
                    u_image : sampler,
                    u_tex_scale : 1024f32,
                    u_pattern_tl : sprite.tl,
                    u_pattern_br : sprite.br,
                    u_texsize : texsize,

                };
                let mut uniforms = MergeUniforms(
                    &bucket.uniforms,
                    &a,
                );

                let sid = coord.id();

                let draw_params = glium::DrawParameters {
                    blend: glium::Blend::alpha_blending(),
                    stencil: glium::draw_parameters::Stencil {
                        test_clockwise: glium::StencilTest::IfEqual { mask: 0xFFFFFFFF },
                        test_counter_clockwise: glium::StencilTest::IfEqual { mask: 0xFFFFFFFF },

                        reference_value_clockwise: sid as _,
                        reference_value_counter_clockwise: sid as _,

                        ..Default::default()
                    },
                    ..Default::default()
                };

                let buffers = bucket.pos_vbo.as_ref().unwrap();
                let indices = bucket.last_ibo.as_ref().unwrap();

                (params.frame).draw(buffers, indices, self.pattern_program.as_ref().unwrap(), &uniforms, &draw_params)?;
            } else {
                error!("Pattern icon : {} not found", pattern);
            }
            // render as pattern
        } else {
            // Render as Color fill

            let a = uniform! {
                u_matrix: matrix,
                feature_data_ubo: & bucket.feature_data.data,
            };
            let mut uniforms = MergeUniforms(
                &bucket.uniforms,
                &a,
            );

            let sid = coord.id();

            let draw_params = glium::DrawParameters {
                blend: glium::Blend::alpha_blending(),
                stencil: glium::draw_parameters::Stencil {
                    test_clockwise: glium::StencilTest::IfEqual { mask: 0xFFFFFFFF },
                    test_counter_clockwise: glium::StencilTest::IfEqual { mask: 0xFFFFFFFF },

                    reference_value_clockwise: sid as _,
                    reference_value_counter_clockwise: sid as _,

                    ..Default::default()
                },
                ..Default::default()
            };

            let buffers = bucket.pos_vbo.as_ref().unwrap();
            let indices = bucket.last_ibo.as_ref().unwrap();

            (params.frame).draw(buffers, indices, &self.shader_program, &uniforms, &draw_params)?;
        }

        Ok(())
    }
}