use ::prelude::*;

use ::common::glium::vertex::{AttributeType, Attribute};


/// Layout of single property, can be bound to uniform value
/// or sent in BufferTexture, and looked up based on feature ID
#[derive(Debug, Clone)]
pub struct PropertyItemLayout {
    pub name: String,
    pub format: AttributeType,
    /// Offset in vec4 coordinates
    pub offset: usize,
    pub size: usize,
}

/// Struct that holds information needed to bind per-feature property data into UBO or TBO, this data will be used by shader compilation
/// to find out, which attributes need uniforms generated, and which need special retrieval function generated
#[derive(Debug, Default)]
pub struct FeaturePropertyLayout {
    pub items: Vec<PropertyItemLayout>,
}

impl FeaturePropertyLayout {
    pub fn push(&mut self, prop_name: &str, format: AttributeType) {
        let off = self.items.len();
        self.items.push(PropertyItemLayout {
            name: prop_name.to_string(),
            format,
            offset: off,
            size: 1,
        });
    }
    pub fn is_feature(&self, prop_name: &str) -> bool {
        return self.items.iter().any(|i| i.name == prop_name);
    }

    pub fn size_per_feature(&self) -> usize {
        self.items.last().map(|x| x.offset + x.size).unwrap_or(0)
    }

    pub fn item(&self, prop_name: &str) -> PropertyItemLayout {
        return self.items.iter().filter(|&a| a.name == prop_name).next().unwrap().clone();
    }
}

/// Struct that holds information needed to bind Per-Layer data into shader uniforms, these are static or zoom-dependent
/// properties
#[derive(Debug, Default)]
pub struct UniformPropertyLayout {
    pub items: Vec<PropertyItemLayout>,
}

impl UniformPropertyLayout {
    pub fn push(&mut self, prop_name: &str, format: AttributeType) {
        self.items.push(PropertyItemLayout {
            name: prop_name.to_string(),
            format,
            offset: 0,
            size: 0,
        });
    }
    pub fn is_uniform(&self, prop_name: &str) -> bool {
        return self.items.iter().any(|i| i.name == prop_name);
    }
}

pub struct ShaderProcessor;


use common::regex::{Match, Matches, CaptureMatches, CaptureNames, Captures};

impl ShaderProcessor {
    pub fn uniform_name(prop_name: &str) -> String {
        format!("u_{}", prop_name)
    }


    pub fn get_shader(displ: &glium::backend::Facade, vert: &str, frag: &str, uniforms: &UniformPropertyLayout, features: &FeaturePropertyLayout) -> Result<glium::program::Program> {
        let regex = Regex::new(r"(?xm)

        \#pragma \s+ property \s* : \s* (?P<op>\w+) \s* (?P<type>\w+) \s+ (?P<name>\w+);?

        ").unwrap();

        let custom = format!("#define PER_FEATURE_SIZE float({size})", size = features.size_per_feature());

        let common_defines = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../shaders/_prelude.common.glsl"));

        let vert_prelude = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../shaders/_prelude.vert.glsl"));
        let frag_prelude = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../shaders/_prelude.frag.glsl"));

        let vert = format!("{}\n{}\n{}\n{}\n", vert_prelude, custom, common_defines, vert);
        let frag = format!("{}\n{}\n{}\n{}\n", frag_prelude, custom, common_defines, frag);


        let process = |caps: &Captures| {
            let op = caps.name("op").unwrap().as_str();
            let typ = caps.name("type").unwrap().as_str();
            let name = caps.name("name").unwrap().as_str();

            let is_uniform = uniforms.is_uniform(name);
            let is_feature = features.is_feature(name);

            // Number of vec4
            let feature_size_vec4 =
                assert!(is_uniform ^ is_feature, "Property can't be both uniform and feature at the same time");
            let uniform_name = ShaderProcessor::uniform_name(name);

            let res = match (op, typ, name, is_feature) {
                ("define", typ, name, false) => {
                    format!("uniform {} {};", typ, uniform_name)
                }
                ("define", typ, name, true) => {
                    let item = features.item(name);
                    format!(r#"
                    {typ} get_{name} (float feature) {{
                       return feature_get_{typ}(feature,float({offset}));
                    }}"#, typ = typ, name = name, offset = item.offset)
                }

                // Uniform
                ("init", typ, name, false) => {
                    format!("{typ} {name} = {u_name};", typ = typ, name = name, u_name = uniform_name)
                }
                // Feature
                ("init", typ, name, true) => {
                    format!("{typ} {name} = get_{name}(feature);", typ = typ, name = name)
                }

                (op, _, _, _) => {
                    panic!("Invalid shader pragma operation : `{}`", op);
                }
                _ => {
                    "".into()
                }
            };

            //println!("op: `{}`, typ : `{}`, name : `{}`", op, typ, name);
            res
        };
        let vert_processed = regex.replace_all(&vert, process);


        let frag_processed = regex.replace_all(&frag, process);

        //trace!("Vertex shader processed \n Orig: \n{}\n New : \n{}", vert, vert_processed);
       // trace!("Fragment shader processed \n Orig: \n{}\n New : \n{}", frag, frag_processed);


        // panic!("\nOLD: {}, \nNEW: {}", vert, vert_processed);3

        Ok(glium::Program::from_source(displ, &vert_processed, &frag_processed, None)?)
    }
}