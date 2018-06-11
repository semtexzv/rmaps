use prelude::*;

#[derive(Debug, Clone, Copy, Hash)]
pub struct TileCoords {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Clone)]
pub enum LoadPreference {
    None,
    Cache,
    Network,
    CacheOnly,
    NetworkOnly,
    Any,
}

#[derive(Debug, Clone)]
pub enum ResourceData {
    Tile {
        template: String,
        ratio: i32,
        coords: TileCoords,
    },
    StyleJson {
        url: String,
    },
    SourceJson {
        url: String,
    },
}

#[derive(Debug, Clone)]
pub struct Resource {
    pub load_pref: LoadPreference,
    pub data: ResourceData,
}

impl Resource {
    pub fn url<'a>(&'a self) -> &'a str {
        return match &self.data {
            ResourceData::StyleJson  { ref url } => &url,
            ResourceData::SourceJson { ref url } => &url,
            _ => {
                panic!()
            }
        };
    }
    pub fn style(url: String) -> Resource {
        Resource {
            load_pref: LoadPreference::Any,
            data: ResourceData::StyleJson {
                url: url
            },
        }
    }
    pub fn source(url: String) -> Resource {
        Resource {
            load_pref: LoadPreference::Any,
            data: ResourceData::SourceJson {
                url: url
            },
        }
    }
}