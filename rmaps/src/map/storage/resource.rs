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
pub enum Request {
    Tile(TileRequestData),
    StyleJson(StyleRequestData),
    SourceJson(SourceRequestData),
}

impl Request {
    pub fn url<'a>(&'a self) -> String {
        return match &self {
            Request::StyleJson(StyleRequestData { ref url, .. }) => url.to_string(),
            Request::SourceJson(SourceRequestData { ref url, .. }) => url.to_string(),
            Request::Tile(TileRequestData { ref template, ref coords, .. }) => {
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
        Request::StyleJson(
            StyleRequestData {
                url: url,
            })
    }
    pub fn source(url: String) -> Request {
        Request::SourceJson(
            SourceRequestData {
                url: url,
            }
        )
    }

    pub fn tile(src_id: String, url_template: String, coords: TileCoords) -> Request {
        Request::Tile(
            TileRequestData {
                template: url_template,
                coords,
                source: src_id,
            }
        )
    }

    pub fn is_style(&self) -> bool {
        return if let Request::StyleJson(..) = self {
            true
        } else {
            false
        };
    }
    pub fn is_source(&self) -> bool {
        return if let Request::SourceJson(..) = self {
            true
        } else {
            false
        };
    }
    pub fn is_tile(&self) -> bool {
        return if let Request::Tile(..) = self {
            true
        } else {
            false
        };
    }


    pub fn style_data(&self) -> Option<&StyleRequestData> {
        return match self {
            Request::StyleJson(ref s) => Some(s),
            _ => None
        };
    }

    pub fn source_data(&self) -> Option<&SourceRequestData> {
        return match self {
            Request::SourceJson(ref s) => Some(s),
            _ => None,
        };
    }
    pub fn tile_data(&self) -> Option<&TileRequestData> {
        return match self {
            Request::Tile(ref s) => Some(s),
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