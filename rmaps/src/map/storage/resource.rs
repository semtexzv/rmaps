use prelude::*;


#[derive(Debug, Clone)]
pub struct TileRequestData {
    pub template: String,
    pub coords: TileCoords,
    pub source: String,
}

#[derive(Debug, Clone)]
pub enum Request {
    Tile(TileRequestData),
    StyleJson(String),
    SourceJson(String, String),
    /// Url should not contain trailing filetype, this will be added by Request implementation
    SpriteJson(String),
    /// Url should not contain trailing filetype, this will be added by Request implementation
    SpriteImage(String),
}

impl Request {
    pub fn url(&self) -> String {
        use super::url::*;
        return match &self {
            Request::StyleJson(ref url) => {
                if is_mapbox_url(url) {
                    normalize_style(url)
                } else {
                    url.to_string()
                }
            }
            Request::SourceJson(_,ref url) => {
                if is_mapbox_url(url) {
                    normalize_source(url)
                } else {
                    url.to_string()
                }
            }
            Request::SpriteImage(ref url) => {
                if is_mapbox_url(url) {
                    normalize_sprite(url, "png")
                } else {
                    format!("{}.png", url)
                }
            }
            Request::SpriteJson(ref url) => {
                if is_mapbox_url(url) {
                    normalize_sprite(url, "json")
                } else {
                    format!("{}.json", url)
                }
            }
            Request::Tile(TileRequestData { ref template, ref coords, .. }) => {
                template
                    .replace("{x}", &format!("{}", coords.x))
                    .replace("{y}", &format!("{}", coords.y))
                    .replace("{z}", &format!("{}", coords.z))
            }
        };
    }
    pub fn style(url: String) -> Request {
        Request::StyleJson(url)
    }
    pub fn source(name : String, url: String) -> Request {
        Request::SourceJson(name ,url)
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


    pub fn style_data(&self) -> Option<&str> {
        return match self {
            Request::StyleJson(ref s) => Some(s),
            _ => None
        };
    }

    pub fn source_data(&self) -> Option<&str> {
        return match self {
            Request::SourceJson(_,ref s) => Some(s),
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
    pub cache_until: u64,
    pub data: Vec<u8>,
}

impl Resource {
    pub fn cacheable(&self) -> bool {
        let url = self.req.url();
        return !url.starts_with("file://") && !url.starts_with("local://");
    }
}