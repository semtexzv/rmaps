#![allow(unused_unsafe, dead_code, unused_variables, unused_imports)]
pub extern crate common;

pub extern crate prost;
#[macro_use]
pub extern crate prost_derive;
pub extern crate prost_types;

pub extern crate bytes;

pub extern crate quick_protobuf;

pub use common::geometry::Value;

pub mod vector_tile;
pub mod tile2;

pub use vector_tile::tile::GeomType;


#[derive(Debug, Clone)]
pub struct Tile {
    pub layers: Vec<Layer>,
}

impl From<vector_tile::Tile> for Tile {
    fn from(vt: vector_tile::Tile) -> Self {
        return Tile {
            layers: vt.layers.into_iter().map(|x| x.into()).collect()
        };
    }
}

use std::sync::Arc;

use std::rc::Rc;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Layer {
    pub version: u32,
    pub name: String,
    pub features: Vec<Feature>,
    pub tags: Arc<LayerTags>,
    pub extent: u32,

}

#[derive(Debug)]
pub struct LayerTags {
    pub keys: Vec<String>,
    pub key_idxs: BTreeMap<String, usize>,
    pub values: Vec<Value>,
}

impl LayerTags {
    fn parse(layer: &vector_tile::tile::Layer) -> Self {
        LayerTags {
            keys: layer.keys.clone(),
            key_idxs: layer.keys.iter().enumerate().map(|(i, k)| (k.clone(), i)).collect(),
            values: layer.values.iter().map(|a| Value::from(a.clone())).collect(),
        }
    }
    fn get(&self, key: &str) -> Option<&Value> {
        if let Some(k) = self.key_idxs.get(key) {
            Some(&self.values[*k])
        } else {
            None
        }
    }
}

impl From<vector_tile::tile::Layer> for Layer {
    fn from(vl: vector_tile::tile::Layer) -> Self {
        let tags = Arc::new(LayerTags::parse(&vl));
        return Layer {
            features: vl.features.iter().map(|x| Feature::parse(&vl, tags.clone(), x)).collect(),
            version: vl.version,
            tags,
            name: vl.name,
            extent: vl.extent.unwrap_or(4096),
        };
    }
}


#[derive(Debug, Clone)]
pub struct Feature {
    pub id: u64,
    pub typ: GeomType,
    pub tags: Arc<LayerTags>,
    pub geom: Vec<Vec<[i32; 2]>>,
}

impl From<vector_tile::tile::Value> for Value {
    fn from(v: vector_tile::tile::Value) -> Self {
        if let Some(s) = v.string_value {
            return Value::String(s);
        }

        if let Some(v) = v.float_value {
            return Value::Float(v as _);
        }

        if let Some(v) = v.double_value {
            return Value::Float(v as _);
        }

        if let Some(v) = v.int_value.or(v.sint_value) {
            return Value::Int(v as _);
        }

        if let Some(v) = v.uint_value {
            return Value::UInt(v as _);
        }

        if let Some(v) = v.bool_value {
            return Value::Bool(v as _);
        }
        panic!()
    }
}

#[derive(Debug, Clone)]
pub enum GeomData {
    Points(Vec<[u32; 2]>),
    Lines(Vec<Vec<[u32; 2]>>),
    Polys(Vec<Vec<[u32; 2]>>),
}

pub const COMMAND_MOVE_TO: u32 = 1;
pub const COMMAND_LINE_TO: u32 = 2;
pub const COMMAND_CLOSE_PATH: u32 = 7;

pub fn decode_zigzag(v: u32) -> i32 {
    (v >> 1) as i32 ^ (-((v & 1) as i32))
}

fn parse_geometry(typ: GeomType, data: &[u32]) -> Vec<Vec<[i32; 2]>> {
    let mut cursor = [0, 0];
    let mut pos = 0;

    let mut geometry = vec![];
    let mut ring = vec![];

    while pos < data.len() {
        match (data[pos] & 0x7, data[pos] >> 3, typ) {
            (COMMAND_MOVE_TO, count, _) => {
                for i in 0..count {
                    let base = (pos + 1 + i as usize * 2) as usize;
                    cursor[0] += decode_zigzag(data[base + 0]);
                    cursor[1] += decode_zigzag(data[base + 1]);
                    if !ring.is_empty() {
                        geometry.push(::std::mem::replace(&mut ring, vec![]));
                    }

                    ring.push(cursor);
                }
                pos += (1 + 2 * count) as usize;
            }
            (COMMAND_LINE_TO, count, GeomType::Linestring) | (COMMAND_LINE_TO, count, GeomType::Polygon) => {
                for i in 0..count {
                    let base = (pos + 1 + i as usize * 2) as usize;
                    cursor[0] += decode_zigzag(data[base + 0]);
                    cursor[1] += decode_zigzag(data[base + 1]);
                    ring.push(cursor);
                }
                pos += (1 + 2 * count) as usize;
            }
            (COMMAND_CLOSE_PATH, 1, GeomType::Polygon) => {
                pos += 1;
                let first = ring[0];
                //  ring.push(first);
            }
            (command, count, typ) => {
                panic!("Invalid comand : {:?}, count : {:?}, type : {:?}", command, count, typ);
            }
        };
    }

    if !ring.is_empty() {
        geometry.push(::std::mem::replace(&mut ring, vec![]));
    }
    //geometry.reverse();

    return geometry;
}

impl Feature {
    fn parse(vl: &vector_tile::tile::Layer, tags: Arc<LayerTags>, f: &vector_tile::tile::Feature) -> Feature {
        let typ = GeomType::from_i32(f.type_.unwrap_or(0)).unwrap_or(GeomType::Unknown);


        return Feature {
            id: f.id.unwrap_or(0),
            tags,
            typ,
            geom: parse_geometry(typ, &f.geometry),
        };
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        let idx = self.tags.key_idxs.get(name);
        idx.and_then(|id| self.tags.values.get(*id))
    }
}


#[test]
fn test_decode() {
    use prost::Message;
    let tile = ::vector_tile::Tile::decode(&include_bytes!("../test.mvt")[..]).unwrap();

    let t2 = Tile::from(tile);
    unsafe {
        panic!("{:#?}", t2);
    }
}
