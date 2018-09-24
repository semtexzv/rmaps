pub use quick_protobuf::Error;

pub type Result = ::std::result::Result<(), Error>;


pub trait GlyphAccess {
    fn accept(&mut self, vis: &mut impl GlyphVisitor) -> Result;
}

pub trait FontStackAccess {
    fn accept(&mut self, vis: &mut impl FontStackVisitor) -> Result;
}

pub trait GlyphsAccess {
    fn accept(&mut self, vis: &mut impl GlyphsVisitor) -> Result;
}

pub trait GlyphVisitor {
    fn visit_id(&mut self, v: u32);
    fn visit_bitmap(&mut self, v: &[u8]);
    fn visit_width(&mut self, v: u32);
    fn visit_height(&mut self, v: u32);
    fn visit_left(&mut self, v: i32);
    fn visit_top(&mut self, v: i32);
    fn visit_advance(&mut self, v: u32);
}

pub trait FontStackVisitor {
    fn visit_name(&mut self, v: &str);
    fn visit_range(&mut self, v: &str);
    fn visit_glyph(&mut self, acc: &mut impl GlyphAccess) -> Result;
}

pub trait GlyphsVisitor {
    fn visit_fontstack(&mut self, acc: &mut impl FontStackAccess) -> Result;
}

pub use quick_protobuf::{BufRefAccess, BytesReader};

impl<'acc> GlyphAccess for BufRefAccess<'acc> {
    fn accept(&mut self, vis: &mut impl GlyphVisitor) -> Result {
        while !self.r.is_eof() {
            match self.r.next_tag(self.buf) {
                Ok(8) => vis.visit_id(self.r.read_uint32(self.buf)?), // msg.id = r.read_uint32(bytes)?,
                Ok(18) => vis.visit_bitmap(self.r.read_bytes(self.buf)?),// msg.bitmap = Some(r.read_bytes(bytes).map(Cow::Borrowed)?),
                Ok(24) => vis.visit_width(self.r.read_uint32(self.buf)?), //msg.width = r.read_uint32(bytes)?,
                Ok(32) => vis.visit_height(self.r.read_uint32(self.buf)?), //msg.height = r.read_uint32(bytes)?,
                Ok(40) => vis.visit_left(self.r.read_sint32(self.buf)?),//msg.left = r.read_sint32(bytes)?,
                Ok(48) => vis.visit_top(self.r.read_sint32(self.buf)?),//msg.top = r.read_sint32(bytes)?,
                Ok(56) => vis.visit_advance(self.r.read_uint32(self.buf)?),//msg.advance = r.read_uint32(bytes)?,
                Ok(t) => { self.r.read_unknown(self.buf, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}

impl<'acc> FontStackAccess for BufRefAccess<'acc> {
    fn accept(&mut self, vis: &mut impl FontStackVisitor) -> Result {
        while !self.r.is_eof() {
            match self.r.next_tag(self.buf) {
                Ok(10) => vis.visit_name(self.r.read_string(self.buf)?),//msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => vis.visit_range(self.r.read_string(self.buf)?),//msg.range = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => self.read_nested(|acc| vis.visit_glyph(acc))?,//msg.glyphs.push(r.read_message::<mapboxgl::glyphs::glyph>(bytes)?),
                Ok(t) => { self.r.read_unknown(self.buf, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}

impl<'acc> GlyphsAccess for BufRefAccess<'acc> {
    fn accept(&mut self, vis: &mut impl GlyphsVisitor) -> Result {
        while !self.r.is_eof() {
            match self.r.next_tag(self.buf) {
                Ok(10) => self.read_nested(|acc| vis.visit_fontstack(acc))?,
                Ok(t) => { self.r.read_unknown(self.buf, t)?; }
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }
}