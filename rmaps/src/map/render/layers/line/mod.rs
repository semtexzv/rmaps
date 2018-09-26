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

pub mod bucket;
pub mod props;


#[derive(Debug)]
pub struct LineLayer {
    style_layer: style::LineLayer,
    pub shader_program: Rc<glium::Program>,
    pub properties: props::LineLayerProperties,
    pub layout: (UniformPropertyLayout, FeaturePropertyLayout),
}

impl layers::LayerNew for LineLayer {
    type StyleLayer = style::LineLayer;

    fn new(facade: &Display, style_layer: &<Self as layers::LayerNew>::StyleLayer) -> Self {
        let (uni, feat) = ::map::render::property::PropertyLayoutBuilder::build::<props::LineFeatureProperties>(style_layer);


        let shader_program = layer_program!(facade,"line", &uni, &feat);

        LineLayer {
            layout: (uni, feat),
            style_layer: style_layer.clone(),
            properties: Default::default(),
            shader_program: shader_program.unwrap(),
        }
    }
}

impl layers::WithSource for LineLayer {
    fn source_name(&self) -> Option<&str> {
        self.style_layer.common.source.as_ref().map(Deref::deref)
    }
}


impl layers::BucketLayer for LineLayer {
    type Bucket = bucket::LineBucket;

    fn new_tile(&mut self, display: &Display, data: &Rc<tiles::TileData>) -> Result<Option<Self::Bucket>> {
        if (Some(&data.source) == self.style_layer.common.source.as_ref()) {
            if let Some(ref source_layer) = self.style_layer.common.source_layer {
                return Ok(bucket::LineBucket::new(display, data.clone(), &self.style_layer.common)?);
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

        UniformPropertyBinder::rebind(&self.layout.0, &bucket.properties, &self.style_layer, &mut bucket.uniforms)?;

        bucket.feature_data.clear();

        let features = &mut bucket.features;
        FeaturePropertyBinder::with(&self.layout.1, &mut bucket.feature_data, |binder| {
            for (id, data) in features.iter_mut() {
                let mut evaluator = PropertiesEvaluator::new(params.zoom,&data.feature);
                data.props.accept_mut(&self.style_layer, &mut evaluator);

                data.props.accept(&self.style_layer, binder);
            }
        });

        bucket.eval_dirty = false;
        bucket.upload_dirty = true;
        Ok(())
    }


    fn render_bucket(&mut self, params: &mut render::RenderParams, coord: UnwrappedTileCoords, bucket: &Self::Bucket) -> Result<()> {
        let tile_matrix = Mercator::tile_to_world(coord);


        let custom_uniforms = uniform! {
            u_p_matrix : Into::<[[f32; 4]; 4]>::into(params.camera.projection()),
            u_mv_matrix : Into::<[[f32; 4]; 4]>::into(params.camera.view() * tile_matrix),
            feature_data_ubo: & bucket.feature_data.data,
        };

        let mut uniforms = MergeUniforms(
            &bucket.uniforms,
            &custom_uniforms,
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

        Ok(())
    }
}