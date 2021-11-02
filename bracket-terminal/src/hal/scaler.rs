use glutin::dpi::LogicalSize;

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

pub struct ScreenScaler {
    pub desired_gutter: u32,
    pub physical_size: (u32, u32),
    pub scale_factor: f32,
    gutter_left: u32,
    gutter_right: u32,
    gutter_top: u32,
    gutter_bottom: u32,
    available_width: u32,
    available_height: u32,
}

impl ScreenScaler {
    pub fn default() -> Self {
        Self {
            desired_gutter: 0,
            physical_size: (0, 0),
            scale_factor: 1.0,
            gutter_left: 0,
            gutter_right: 0,
            gutter_top: 0,
            gutter_bottom: 0,
            available_width: 0,
            available_height: 0,
        }
    }

    pub fn new(desired_gutter: u32, desired_width: u32, desired_height: u32) -> Self {
        let mut result = Self {
            desired_gutter,
            physical_size: (desired_width, desired_height),
            scale_factor: 1.0,
            gutter_left: 0,
            gutter_right: 0,
            gutter_top: 0,
            gutter_bottom: 0,
            available_width: 0,
            available_height: 0,
        };
        result.recalculate_coordinates();
        result
    }

    pub fn new_window_size(&self) -> LogicalSize<u32> {
        LogicalSize::new(
            self.physical_size.0 + self.desired_gutter,
            self.physical_size.1 + self.desired_gutter,
        )
    }

    pub fn change_logical_size(&mut self, width: u32, height: u32, scale: f32) {
        self.scale_factor = scale;
        self.physical_size.0 = (width as f32 * scale) as u32;
        self.physical_size.1 = (height as f32 * scale) as u32;
        self.recalculate_coordinates();
    }

    pub fn change_physical_size(&mut self, width: u32, height: u32, scale: f32) {
        self.scale_factor = scale;
        self.physical_size.0 = width;
        self.physical_size.1 = height;
        self.recalculate_coordinates();
    }

    fn recalculate_coordinates(&mut self) {
        let total_gutter = (self.desired_gutter as f32 * self.scale_factor) as u32;
        let half_gutter = total_gutter / 2;
        if total_gutter % 2 == 0 {
            self.gutter_left = half_gutter;
            self.gutter_right = half_gutter;
            self.gutter_top = half_gutter;
            self.gutter_bottom = half_gutter;
        } else {
            self.gutter_left = half_gutter;
            self.gutter_right = half_gutter+1;
            self.gutter_top = half_gutter;
            self.gutter_bottom = half_gutter+1;
        }

        self.available_width = self.physical_size.0 - total_gutter;
        self.available_height = self.physical_size.1 - total_gutter;
    }

    pub fn pixel_to_screen(&self, x: u32, y: u32) -> (f32, f32) {
        (
            ((x as f32 / self.physical_size.0 as f32) * 2.0) - 1.0,
            ((y as f32 / self.physical_size.1 as f32) * 2.0) - 1.0,
        )
    }

    pub fn top_left_pixel(&self) -> (f32, f32) {
        self.pixel_to_screen(self.gutter_left, self.gutter_top)
    }

    pub fn bottom_right_pixel(&self) -> (f32, f32) {
        self.pixel_to_screen(
            self.physical_size.0 - self.gutter_right,
            self.physical_size.1 - self.gutter_bottom
        )
    }

    pub fn calc_step(&self, width: u32, height: u32, scale: f32) -> (f32, f32) {
        let (lx, ty) = self.top_left_pixel();
        let (rx, by) = self.bottom_right_pixel();
        (
            ((rx - lx) * scale) / width as f32,
            ((by - ty) * scale) / height as f32,
        )
    }
}