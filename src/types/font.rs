use rusttype::{Scale, PositionedGlyph, Point, point};

/// Represents a font.
///
/// To obtain a `Font`, use the [`use_font_data`](crate::hooks::use_font_data) hook.
pub struct Font {
    rusttype_font: rusttype::Font<'static>,
}

impl Font {
    pub fn from_data(ttf_data:&'static[u8])->Self {
        let f=rusttype::Font::try_from_bytes(ttf_data).unwrap();

        Self {
            rusttype_font: f
        }
    }

    fn get_glyph_advance(&self, c: char, s: Scale) -> (f32, f32) {
        let g = self.rusttype_font.glyph(c).scaled(s);
        let h = g.h_metrics().advance_width;
        let v_metrics = self.rusttype_font.v_metrics(s);
        let v = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;
        (h, v)
    }

    /// Get width in pixels of a string of rendered text.
    pub fn get_str_width(&self, str: &str, size: f32) -> f32 {
        let mut w: f32 = 0.0;
        let s = Scale::uniform(size);

        for c in str.chars() {
            let (adv_x, _adv_y) = self.get_glyph_advance(c, s);
            w += adv_x;
        }

        w
    }

    pub fn create_glyphs<'a>(&self, str: &str, x: f32, y: f32, size: f32)
            ->Vec<PositionedGlyph<'a>> {
        let mut v = Vec::new();
        let mut p:Point<f32>=rusttype::point(x,y);
        let s:Scale=rusttype::Scale::uniform(size);

        for c in str.chars() {
            let base_glyph = self.rusttype_font.glyph(c);
            v.push(base_glyph.scaled(s).positioned(p));

            let (adv_x, _adv_y) = self.get_glyph_advance(c, s);
            p = point(p.x + adv_x, p.y);
        }

        v
    }

    pub fn baseline(&self, size:f32)->f32 {
        let s = Scale::uniform(size);
        let v_metrics = self.rusttype_font.v_metrics(s);
        size + v_metrics.descent
    }
}
