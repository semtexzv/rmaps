use prelude::*;

use map::{
    style,
    render::{
        self,
        layers::{
            self, Layer, BucketLayer, Bucket,
        },
        EvaluationParams, RendererParams, RenderParams,
        property::*,
    },
    tiles::{
        self, TileData, DecodedTileData,
    },
};

#[derive(Debug, Clone, Copy, Vertex)]
pub struct RasterVertex {
    pub pos: [f32; 2],
    pub tex: [f32; 2],
}

#[derive(Debug)]
pub struct RasterBucket {
    pub texture: glium::texture::Texture2d,
    pub vbo: glium::VertexBuffer<RasterVertex>,

}

impl Bucket for RasterBucket {}

#[derive(Debug)]
pub struct RasterLayer {
    style_layer: style::RasterLayer,
    shader_program: Rc<glium::Program>,
    _feature_data : FeaturePropertyData,
}

impl layers::WithSource for RasterLayer {
    fn source_name(&self) -> Option<&str> {
        self.style_layer.common.source.as_ref().map(Deref::deref)
    }
}

impl BucketLayer for RasterLayer {
    type Bucket = RasterBucket;

    fn new_tile(&mut self, display: &Display, data: &Rc<TileData>) -> Result<Option<<Self as BucketLayer>::Bucket>> {
        println!("New raster tile");
        let tiles::RasterTileData { ref image, dims } = data.data.unwrap_raster();

        let raw = glium::texture::RawImage2d::from_raw_rgba_reversed(image, *dims);

        let texture = glium::texture::Texture2d::new(display, raw).unwrap();
        let vertices = &[
            RasterVertex {
                pos: [0., 0.],
                tex: [0., 1.],
            },
            RasterVertex {
                pos: [EXTENT, 0.],
                tex: [1., 1.],
            },
            RasterVertex {
                pos: [EXTENT, EXTENT],
                tex: [1., 0.],
            },
            RasterVertex {
                pos: [0., EXTENT],
                tex: [0., 0.],
            },
        ];

        let vbo = glium::VertexBuffer::new(display, vertices).unwrap();
        return Ok(Some(RasterBucket {
            texture,
            vbo,
        }));
    }

    fn eval_bucket(&mut self, params: &EvaluationParams, bucket: &mut <Self as BucketLayer>::Bucket) -> Result<()> {
        Ok(())
    }

    fn render_bucket(&mut self, params: &mut RenderParams, coords: UnwrappedTileCoords, bucket: &<Self as BucketLayer>::Bucket) -> Result<()> {
        //println!("Rendering Tile : {:?}", coords);
        let tile_matrix = Mercator::tile_to_world(coords);
        let matrix = params.camera.projection() * params.camera.view() * tile_matrix;
        let matrix: [[f32; 4]; 4] = matrix.into();

        let uniforms = uniform! {
                u_matrix : matrix,
                u_texture : &bucket.texture,
                feature_data_ubo: &self._feature_data,
            };

        let draw_params = glium::DrawParameters {
            blend: glium::Blend::alpha_blending(),
            ..Default::default()
        };

        (params.frame).draw(&bucket.vbo, glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan), &self.shader_program, &uniforms, &draw_params)?;
        Ok(())
    }
}

impl layers::LayerNew for RasterLayer {
    type StyleLayer = style::RasterLayer;

    fn new(facade: &Display, style_layer: &<Self as layers::LayerNew>::StyleLayer) -> Self {
        let shader_program = layer_program!(facade, "raster-simple", & Default::default(), & Default::default()).unwrap();

        return RasterLayer {
            style_layer: style_layer.clone(),
            shader_program,
            _feature_data : FeaturePropertyData::new(facade).unwrap(),
        };
    }
}
