use prelude::*;


#[derive(Debug, Clone)]
pub struct TileRequestData {
    pub template: String,
    pub coords: TileCoords,
    pub source: String,
}

#[derive(Debug, Clone)]
pub struct StyleRequestData {
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct SourceRequestData {
    pub url: String,
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
pub enum RequestData {
    Tile(TileRequestData),
    StyleJson(StyleRequestData),
    SourceJson(SourceRequestData),
}

#[derive(Debug, Clone)]
pub struct Request {
    pub load_pref: LoadPreference,
    pub data: RequestData,
}

impl Request {
    pub fn url<'a>(&'a self) -> String {
        return match &self.data {
            RequestData::StyleJson(StyleRequestData { ref url, .. }) => url.to_string(),
            RequestData::SourceJson(SourceRequestData { ref url, .. }) => url.to_string(),
            RequestData::Tile(TileRequestData { ref template, ref coords, .. }) => {
                template
                    .replace("{x}", &format!("{}", coords.x))
                    .replace("{y}", &format!("{}", coords.y))
                    .replace("{z}", &format!("{}", coords.z))
            }
            _ => {
                panic!()
            }
        };
    }
    pub fn style(url: String) -> Request {
        Request {
            load_pref: LoadPreference::Any,
            data: RequestData::StyleJson(
                StyleRequestData {
                    url: url,
                }
            ),
        }
    }
    pub fn source(url: String) -> Request {
        Request {
            load_pref: LoadPreference::Any,
            data: RequestData::SourceJson(
                SourceRequestData {
                    url: url,
                }
            ),
        }
    }

    pub fn tile(src_id: String, url_template: String, coords: TileCoords) -> Request {
        Request {
            load_pref: LoadPreference::Any,
            data: RequestData::Tile(
                TileRequestData {
                    template: url_template,
                    coords,
                    source: src_id,
                }),
        }
    }

    pub fn is_style(&self) -> bool {
        return if let RequestData::StyleJson(..) = self.data {
            true
        } else {
            false
        };
    }
    pub fn is_source(&self) -> bool {
        return if let RequestData::SourceJson(..) = self.data {
            true
        } else {
            false
        };
    }
    pub fn is_tile(&self) -> bool {
        return if let RequestData::Tile(..) = self.data {
            true
        } else {
            false
        };
    }


    pub fn style_data(&self) -> Option<&StyleRequestData> {
        return match self.data {
            RequestData::StyleJson(ref s) => Some(s),
            _ => None
        };
    }

    pub fn source_data(&self) -> Option<&SourceRequestData> {
        return match self.data {
            RequestData::SourceJson(ref s) => Some(s),
            _ => None,
        };
    }
    pub fn tile_data(&self) -> Option<&TileRequestData> {
        return match self.data {
            RequestData::Tile(ref s) => Some(s),
            _ => None,
        };
    }
}

use prelude::*;

#[derive(Debug, Clone)]
pub struct Resource {
    pub req: Request,
    pub data: Vec<u8>,
}