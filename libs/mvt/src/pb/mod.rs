pub use quick_protobuf::Error;

pub type Result = ::std::result::Result<(), Error>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GeomType {
    Unknown = 0,
    Point = 1,
    LineString = 2,
    Polygon = 3,
}

impl Default for GeomType {
    fn default() -> Self {
        GeomType::Unknown
    }
}

impl From<i32> for GeomType {
    fn from(i: i32) -> Self {
        match i {
            0 => GeomType::Unknown,
            1 => GeomType::Point,
            2 => GeomType::LineString,
            3 => GeomType::Polygon,
            _ => Self::default(),
        }
    }
}

impl ToString for GeomType {
    fn to_string(&self) -> String {
        match self {
            GeomType::Unknown => "Unknown".into(),
            GeomType::Point => "Point".into(),
            GeomType::LineString => "LineString".into(),
            GeomType::Polygon => "Polygon".into(),
        }
    }
}

impl<'a> From<&'a str> for GeomType {
    fn from(s: &'a str) -> Self {
        match s {
            "Unknown" => GeomType::Unknown,
            "Point" => GeomType::Point,
            "LineString" => GeomType::LineString,
            "Polygon" => GeomType::Polygon,
            _ => Self::default(),
        }
    }
}

pub trait TileAccess {
    fn accept(&mut self, vis: &mut impl TileVisitor) -> Result;
}

pub trait LayerAccess {
    fn accept(&mut self, vis: &mut impl LayerVisitor) -> Result;
}

pub trait FeatureAccess {
    fn accept(&mut self, vis: &mut impl FeatureVisitor) -> Result;
}

pub trait ValueAccess {
    fn accept(&mut self, vis: &mut impl ValueVisitor) -> Result;
}

pub trait TileVisitor {
    fn visit_layer(&mut self, acc: &mut impl LayerAccess) -> Result;
}


pub trait LayerVisitor {
    fn visit_version(&mut self, version: u32);
    fn visit_name(&mut self, name: &str);
    fn visit_feature(&mut self, acc: &mut impl FeatureAccess) -> Result;
    fn visit_key(&mut self, key: &str);
    fn visit_value(&mut self, acc: &mut impl ValueAccess) -> Result;
    fn visit_extent(&mut self, extent: u32);
}


pub trait ValueVisitor {
    fn visit_string_value(&mut self, v: &str);
    fn visit_float_value(&mut self, v: f32);
    fn visit_double_value(&mut self, v: f64);
    fn visit_int_value(&mut self, v: i64);
    fn visit_uint_value(&mut self, v: u64);
    fn visit_sint_value(&mut self, v: i64);
    fn visit_bool_value(&mut self, v: bool);
}


pub trait FeatureVisitor {
    fn visit_id(&mut self, id: u64);
    fn visit_tags(&mut self, tags: &[u32]);
    fn visit_geom_type(&mut self, geom_type: GeomType);
    fn visit_geometry(&mut self, geometry: &[u32]);
}

use quick_protobuf::BytesReader;

pub struct BufRefAccess<'acc> {
    pub r: &'acc mut BytesReader,
    pub buf: &'acc [u8],
}

impl<'acc> BufRefAccess<'acc> {
    fn read_nested<F: FnMut(&mut BufRefAccess) -> Result>(&mut self, mut fun: F) -> Result {
        self.r.read_len_varint(self.buf, |r, buf| {
            fun(&mut BufRefAccess { r, buf })
        })
    }
}

impl<'acc> TileAccess for BufRefAccess<'acc> {
    fn accept(&mut self, vis: &mut impl TileVisitor) -> Result {
        while !self.r.is_eof() {
            match self.r.next_tag(self.buf) {
                Ok(26) => self.read_nested(|acc: &mut BufRefAccess| vis.visit_layer(acc))?,
                Ok(t) => { self.r.read_unknown(self.buf, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}

impl<'acc> LayerAccess for BufRefAccess<'acc> {
    fn accept(&mut self, vis: &mut impl LayerVisitor) -> Result {
        while !self.r.is_eof() {
            match self.r.next_tag(self.buf) {
                Ok(120) => vis.visit_version(self.r.read_uint32(self.buf)?),
                Ok(10) => vis.visit_name(self.r.read_string(self.buf)?),
                Ok(18) => self.read_nested(|acc| vis.visit_feature(acc))?,
                Ok(26) => vis.visit_key(self.r.read_string(self.buf)?),
                Ok(34) => self.read_nested(|acc| vis.visit_value(acc))?,
                Ok(40) => vis.visit_extent(self.r.read_uint32(self.buf)?),
                Ok(t) => { self.r.read_unknown(self.buf, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}

impl<'acc> FeatureAccess for BufRefAccess<'acc> {
    fn accept(&mut self, vis: &mut impl FeatureVisitor) -> Result {
        while !self.r.is_eof() {
            match self.r.next_tag(self.buf) {
                Ok(8) => vis.visit_id(self.r.read_uint64(self.buf)?),
                Ok(18) => vis.visit_tags(&self.r.read_packed(self.buf, |r, bytes| r.read_uint32(bytes))?),
                Ok(24) => vis.visit_geom_type(self.r.read_enum(self.buf)?),
                Ok(34) => vis.visit_geometry(&self.r.read_packed(self.buf, |r, bytes| r.read_uint32(bytes))?),
                Ok(t) => { self.r.read_unknown(self.buf, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}

impl<'acc> ValueAccess for BufRefAccess<'acc> {
    fn accept(&mut self, vis: &mut impl ValueVisitor) -> Result {
        while !self.r.is_eof() {
            match self.r.next_tag(self.buf) {
                Ok(10) => vis.visit_string_value(self.r.read_string(self.buf)?),
                Ok(21) => vis.visit_float_value(self.r.read_float(self.buf)?),
                Ok(25) => vis.visit_double_value(self.r.read_double(self.buf)?),
                Ok(32) => vis.visit_int_value(self.r.read_int64(self.buf)?),
                Ok(40) => vis.visit_uint_value(self.r.read_uint64(self.buf)?),
                Ok(48) => vis.visit_sint_value(self.r.read_sint64(self.buf)?),
                Ok(56) => vis.visit_bool_value(self.r.read_bool(self.buf)?),
                Ok(t) => { self.r.read_unknown(self.buf, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}