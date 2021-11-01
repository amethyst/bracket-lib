/// Provides a consistent font to texture coordinates mapping service.
pub struct FontScaler {
    font_dimensions_glyphs: (u16, u16),
    font_dimensions_texture: (f32, f32),
}

pub struct GlyphPosition {
    pub glyph_left: f32,
    pub glyph_right: f32,
    pub glyph_top: f32,
    pub glyph_bottom: f32,
}

impl FontScaler {
    /// Maps a font to scaling information
    pub(crate) fn new(
        font_dimensions_glyphs: (u32, u32),
        font_dimensions_texture: (f32, f32),
    ) -> Self {
        Self {
            font_dimensions_glyphs : (
                font_dimensions_glyphs.0 as u16,
                font_dimensions_glyphs.1 as u16
            ),
            font_dimensions_texture,
        }
    }

    /// Calculates texture font position for a glyph
    pub(crate) fn glyph_position(&self, glyph: u16) -> GlyphPosition {
        let glyph_x = glyph % self.font_dimensions_glyphs.0;
        let glyph_y = self.font_dimensions_glyphs.1 - (glyph / self.font_dimensions_glyphs.0);

        let glyph_left = f32::from(glyph_x) * self.font_dimensions_texture.0;
        let glyph_right = f32::from(glyph_x + 1) * self.font_dimensions_texture.0;
        let glyph_top = f32::from(glyph_y) * self.font_dimensions_texture.1;
        let glyph_bottom = f32::from(glyph_y - 1) * self.font_dimensions_texture.1;

        GlyphPosition {
            glyph_left, glyph_right, glyph_top, glyph_bottom
        }
    }
}