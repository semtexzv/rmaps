pub extern crate common;

pub extern crate prost;
#[macro_use]
pub extern crate prost_derive;
pub extern crate prost_types;

pub extern crate bytes;


pub mod vector_tile;

pub use vector_tile::tile::GeomType;


#[derive(Debug)]
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

#[derive(Debug)]
pub struct Layer {
    pub version: u32,
    pub name: String,
    pub features: Vec<Feature>,
    pub extent: u32,

}

impl From<vector_tile::tile::Layer> for Layer {
    fn from(vl: vector_tile::tile::Layer) -> Self {
        return Layer {
            features: vl.features.iter().map(|x| Feature::parse(&vl, x)).collect(),
            version: vl.version,
            name: vl.name,
            extent: vl.extent.unwrap_or(4096),
        };
    }
}


use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Feature {
    pub id: u64,
    pub typ: GeomType,
    pub tags: BTreeMap<String, Value>,
    pub geom: GeomData,
}

#[derive(Debug)]
pub enum Value {
    String(String),
    Float(f64),
    Int(i64),
    Uint(u64),
    Bool(bool),
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
            return Value::Uint(v as _);
        }

        if let Some(v) = v.bool_value {
            return Value::Bool(v as _);
        }
        panic!()
    }
}

#[derive(Debug)]
pub enum GeomData {
    Points(Vec<[u32; 2]>),
    Lines(Vec<Vec<[u32; 2]>>),
    Polys(Vec<Vec<[u32; 2]>>),
}

fn parse_command_geometry(typ: GeomType, geo: &[u32]) -> GeomData {
    let mut cursor = [0, 0];
    let mut all_geoms = vec![];
    let mut current_geom = vec![];
    let mut data = &geo[..];


    if data.is_empty() {
        panic!();
    }
    while data.len() >= 1 {
        match (data[0] & 0x7, data[0] >> 3) {
            // MoveTo command
            (1, count) => {
                match typ {
                    GeomType::Linestring | GeomType::Polygon => {
                        if !current_geom.is_empty() {
                            all_geoms.push(::std::mem::replace(&mut current_geom, vec![]));
                        }
                    }
                    _ => {}
                }
                for i in 0..count {
                    cursor[0] += data[1 + i as usize];
                    cursor[1] += data[2 + i as usize];
                    current_geom.push(cursor);
                }
                data = &data[1 + 2 * count as usize..];
            }
            // LineTo command
            (2, count) => {
                for i in 0..count {
                    cursor[0] += data[1 + i as usize];
                    cursor[1] += data[2 + i as usize];
                    current_geom.push(cursor);
                }
                data = &data[1 + 2 * count as usize..];
            }
            (7, _) => {
                data = &data[1..];
            }
            _ => {}
        }
    }

    if !current_geom.is_empty() {
        all_geoms.push(::std::mem::replace(&mut current_geom, vec![]));
    }

    return match typ {
        GeomType::Point => {
            GeomData::Points(all_geoms.remove(0))
        }
        GeomType::Linestring => {
            GeomData::Lines(all_geoms)
        }
        GeomType::Polygon => {
            GeomData::Polys(all_geoms)
        }
        _ => {
            panic!("Unknown geometry type");
        }
    };
}

impl Feature {
    fn parse(vl: &vector_tile::tile::Layer, f: &vector_tile::tile::Feature) -> Feature {
        let mut tags = BTreeMap::new();

        for l in f.tags.chunks(2) {
            if let [a, b] = l {
                tags.insert(vl.keys[*a as usize].clone(), Value::from(vl.values[*b as usize].clone()));
            }
        }

        let typ = GeomType::from_i32(f.type_.unwrap_or(0)).unwrap_or(GeomType::Unknown);


        return Feature {
            id: f.id.unwrap_or(0),
            tags,
            typ,
            geom: parse_command_geometry(typ, &f.geometry),
        };
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
