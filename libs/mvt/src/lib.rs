#![allow(unused_unsafe, dead_code, unused_variables, unused_imports)]
pub extern crate geometry;

pub extern crate quick_protobuf;

pub extern crate bytes;

pub extern crate failure;


use std::sync::Arc;

use std::rc::Rc;
use std::collections::BTreeMap;


pub use geometry::Value;

pub mod pb;

pub use pb::GeomType;
pub use failure::Error;


pub fn decode(data: &[u8]) -> ::std::result::Result<Tile, Error> {
    let mut acc = pb::BufRefAccess {
        r: &mut quick_protobuf::BytesReader::from_bytes(data),
        buf: &data,
    };
    let mut vis = Tile::default();
    pb::TileAccess::accept(&mut acc, &mut vis)?;
    Ok(vis)
}

#[derive(Debug, Default, Clone)]
pub struct Tile {
    pub layers: Vec<Layer>,
}

impl pb::TileVisitor for Tile {
    fn visit_layer(&mut self, acc: &mut impl pb::LayerAccess) -> pb::Result {
        let mut l = LayerVis::default();
        acc.accept(&mut l)?;

        self.layers.push(l.into());
        Ok(())
    }
}

#[derive(Debug, Default, Clone)]
pub struct Layer {
    pub version: u32,
    pub name: String,
    pub features: Vec<Feature>,
    pub tags: Arc<LayerTags>,
    pub extent: u32,
}

#[derive(Debug, Default)]
struct LayerVis {
    pub version: u32,
    pub name: String,
    pub features: Vec<Feature>,
    pub tags: LayerTags,
    pub extent: u32,
}

impl Into<Layer> for LayerVis {
    fn into(mut self) -> Layer {
        let tags = Arc::new(self.tags);
        for f in self.features.iter_mut() {
            f.tags = Some(tags.clone());
        }
        Layer {
            version: self.version,
            name: self.name,
            features: self.features,
            tags,
            extent: self.extent,
        }
    }
}

impl pb::LayerVisitor for LayerVis {
    fn visit_version(&mut self, version: u32) {
        self.version = version;
    }

    fn visit_name(&mut self, name: &str) {
        self.name = name.to_owned()
    }

    fn visit_feature(&mut self, acc: &mut impl pb::FeatureAccess) -> pb::Result {
        let mut f = Feature::default();
        acc.accept(&mut f)?;
        self.features.push(f);
        Ok(())
    }

    fn visit_key(&mut self, key: &str) {
        let i = self.tags.keys.len();
        self.tags.keys.push(key.to_owned());
        self.tags.key_idxs.insert(key.to_owned(), i);
    }

    fn visit_value(&mut self, acc: &mut impl pb::ValueAccess) -> pb::Result {
        struct Vis {
            v: Value,
        }
        impl pb::ValueVisitor for Vis {
            fn visit_string_value(&mut self, v: &str) {
                self.v = Value::String(v.to_owned())
            }

            fn visit_float_value(&mut self, v: f32) {
                self.v = Value::Float(v as _);
            }

            fn visit_double_value(&mut self, v: f64) {
                self.v = Value::Float(v as _);
            }

            fn visit_int_value(&mut self, v: i64) {
                self.v = Value::Int(v as _);
            }

            fn visit_uint_value(&mut self, v: u64) {
                self.v = Value::Int(v as _);
            }

            fn visit_sint_value(&mut self, v: i64) {
                self.v = Value::Int(v as _);
            }

            fn visit_bool_value(&mut self, v: bool) {
                self.v = Value::Bool(v);
            }
        }
        let mut vis = Vis {
            v: Value::default()
        };

        acc.accept(&mut vis)?;
        self.tags.values.push(vis.v);
        Ok(())
    }

    fn visit_extent(&mut self, extent: u32) {
        self.extent = extent;
    }
}

#[derive(Debug, Default, Clone)]
pub struct LayerTags {
    pub keys: Vec<String>,
    pub key_idxs: BTreeMap<String, usize>,
    pub values: Vec<Value>,
}

impl LayerTags {
    fn get(&self, key: &str) -> Option<&Value> {
        if let Some(k) = self.key_idxs.get(key) {
            Some(&self.values[*k])
        } else {
            None
        }
    }
}


#[derive(Debug, Default, Clone)]
pub struct Feature {
    pub id: u64,
    pub typ: GeomType,
    pub tag_pairs: BTreeMap<usize, usize>,
    pub tags: Option<Arc<LayerTags>>,
    pub geometry: Vec<Vec<[f32; 2]>>,
}

impl Feature {
    pub fn get(&self, name: &str) -> Option<&Value> {
        if let Some(tags) = self.tags.as_ref() {
            if let Some(key_idx) = tags.key_idxs.get(name) {
                if let Some(val_idx) = self.tag_pairs.get(key_idx) {
                    return tags.values.get(*val_idx);
                }
            }
        }
        None
    }
    pub fn has(&self, name: &str) -> bool {
        return self.get(name).is_some();
    }
}

impl pb::FeatureVisitor for Feature {
    fn visit_id(&mut self, id: u64) {
        self.id = id;
    }

    fn visit_tags(&mut self, tags: &[u32]) {
        for p in tags.chunks(2) {
            if let &[a, b] = p {
                self.tag_pairs.insert(a as _, b as _);
            }
        }
    }

    fn visit_geom_type(&mut self, geom_type: GeomType) {
        self.typ = geom_type;
    }

    fn visit_geometry(&mut self, geometry: &[u32]) {
        self.geometry = parse_geometry(self.typ, &geometry);
    }
}


pub trait GeometryAccess {
    fn accept(&mut self, visitor: &mut impl GeometryVisitor);
}

pub trait GeometryVisitor {
    fn move_to(&mut self, x: i32, y: i32);
    fn line_to(&mut self, x: i32, y: i32);
    fn close_path(&mut self);
}

#[derive(Default, Debug, Clone)]
pub struct PolygonGeometryVisitor {
    cursor: [f32; 2],
    rings: Vec<Vec<[f32; 2]>>,
    ring: Vec<[f32; 2]>,
}

impl GeometryVisitor for PolygonGeometryVisitor {
    #[inline(always)]
    fn move_to(&mut self, x: i32, y: i32) {
        self.cursor[0] += x as f32;
        self.cursor[1] += y as f32;
    }

    #[inline(always)]
    fn line_to(&mut self, x: i32, y: i32) {
        if self.ring.is_empty() {
            self.ring.push(self.cursor);
        }
        self.cursor[0] += x as f32;
        self.cursor[1] += y as f32;
        self.ring.push(self.cursor);
    }

    #[inline(always)]
    fn close_path(&mut self) {
        self.rings.push(std::mem::replace(&mut self.ring, vec![]))
    }
}

#[derive(Default, Debug, Clone)]
pub struct LineGeometryVisitor {
    cursor: [f32; 2],
    points: Vec<[f32; 2]>,
    lines: Vec<Vec<[f32; 2]>>,
}

impl GeometryVisitor for LineGeometryVisitor {
    #[inline(always)]
    fn move_to(&mut self, x: i32, y: i32) {
        if !self.points.is_empty() {
            self.lines.push(std::mem::replace(&mut self.points, vec![]))
        }
        self.cursor[0] += x as f32;
        self.cursor[1] += y as f32;
    }

    #[inline(always)]
    fn line_to(&mut self, x: i32, y: i32) {
        if self.points.is_empty() {
            self.points.push(self.cursor);
        }
        self.cursor[0] += x as f32;
        self.cursor[1] += y as f32;
        self.points.push(self.cursor);
    }

    #[inline(always)]
    fn close_path(&mut self) {
        if !self.points.is_empty() {
            self.lines.push(std::mem::replace(&mut self.points, vec![]))
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct PointGeometryVisitor {
    cursor: [f32; 2],
    pub points: Vec<[f32; 2]>,
}

impl GeometryVisitor for PointGeometryVisitor {
    #[inline(always)]
    fn move_to(&mut self, x: i32, y: i32) {
        self.cursor[0] += x as f32;
        self.cursor[1] += y as f32;
        self.points.push(self.cursor);
    }

    #[inline(always)]
    fn line_to(&mut self, x: i32, y: i32) {
        unimplemented!()
    }

    #[inline(always)]
    fn close_path(&mut self) {
        unimplemented!()
    }
}


pub const COMMAND_MOVE_TO: u32 = 1;
pub const COMMAND_LINE_TO: u32 = 2;
pub const COMMAND_CLOSE_PATH: u32 = 7;

#[inline(always)]
pub fn decode_zigzag(v: u32) -> i32 {
    (v >> 1) as i32 ^ (-((v & 1) as i32))
}

pub struct BufGeometryAccess<'a> {
    typ: GeomType,
    pos: usize,
    data: &'a [u32],
}

impl<'a> GeometryAccess for BufGeometryAccess<'a> {
    fn accept(&mut self, visitor: &mut impl GeometryVisitor) {
        while self.pos < self.data.len() {
            match (self.data[self.pos] & 0x7, self.data[self.pos] >> 3) {
                (COMMAND_MOVE_TO, count) => {
                    for i in 0..count {
                        let base = (self.pos + 1 + i as usize * 2) as usize;
                        visitor.move_to(decode_zigzag(self.data[base + 0]), decode_zigzag(self.data[base + 1]));
                    }
                    self.pos += (1 + 2 * count) as usize;
                }
                (COMMAND_LINE_TO, count) => {
                    for i in 0..count {
                        let base = (self.pos + 1 + i as usize * 2) as usize;
                        visitor.line_to(decode_zigzag(self.data[base + 0]), decode_zigzag(self.data[base + 1]));
                    }
                    self.pos += (1 + 2 * count) as usize;
                }
                (COMMAND_CLOSE_PATH, 1) => {
                    visitor.close_path();
                    self.pos += 1;
                }
                (command, count) => {
                    panic!("Invalid comand : {:?}, count : {:?}", command, count);
                }
            }
        };
    }
}

fn parse_geometry(typ: GeomType, data: &[u32]) -> Vec<Vec<[f32; 2]>> {
    let mut acc = BufGeometryAccess {
        typ,
        pos: 0,
        data,
    };
    return match typ {
        GeomType::Point => {
            let mut vis = PointGeometryVisitor::default();
            acc.accept(&mut vis);
            vec![vis.points]
        }
        GeomType::LineString => {
            let mut vis = LineGeometryVisitor::default();
            acc.accept(&mut vis);
            vis.close_path();

            vis.lines
        }
        GeomType::Polygon => {
            let mut vis = PolygonGeometryVisitor::default();
            acc.accept(&mut vis);
            vis.rings
        }
        _ => {
            vec![]
        }
    };
}

pub fn perform_selftest() -> Tile {
    let mut data = include_bytes!("../test.mvt");
    decode(data).unwrap()
}