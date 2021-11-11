use winit::dpi::LogicalSize;

#[cfg(any(target_os = "windows", target_os = "macos"))]
pub(crate) fn default_gutter_size() -> u32 {
    // Testing showed that an 8-pixel gutter is enough to fix
    // Big Sur and Windows 11.
    8
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
pub(crate) fn default_gutter_size() -> u32 {
    // Testing showed that an 8-pixel gutter is enough to fix
    // Big Sur and Windows 11.
    0
}


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
    pub smooth_gutter_x: u32,
    pub smooth_gutter_y: u32,
    pub physical_size: (u32, u32),
    pub logical_size: (u32, u32),
    pub scale_factor: f32,
    pub gutter_left: u32,
    pub gutter_right: u32,
    pub gutter_top: u32,
    pub gutter_bottom: u32,
    pub available_width: u32,
    pub available_height: u32,
    aspect_ratio: f32,
    resized: bool,
}

impl ScreenScaler {
    pub fn default() -> Self {
        Self {
            desired_gutter: 0,
            smooth_gutter_x: 0,
            smooth_gutter_y: 0,
            physical_size: (0, 0),
            logical_size: (0, 0),
            scale_factor: 1.0,
            gutter_left: 0,
            gutter_right: 0,
            gutter_top: 0,
            gutter_bottom: 0,
            available_width: 0,
            available_height: 0,
            aspect_ratio: 1.0,
            resized: true,
        }
    }

    pub fn new(desired_gutter: u32, desired_width: u32, desired_height: u32) -> Self {
        let mut result = Self {
            desired_gutter,
            smooth_gutter_x: 0,
            smooth_gutter_y: 0,
            physical_size: (desired_width, desired_height),
            logical_size: (desired_width, desired_height),
            scale_factor: 1.0,
            gutter_left: 0,
            gutter_right: 0,
            gutter_top: 0,
            gutter_bottom: 0,
            available_width: 0,
            available_height: 0,
            aspect_ratio: desired_height as f32 / desired_width as f32,
            resized: true,
        };
        result.recalculate_coordinates();
        result
    }

    pub fn new_window_size(&mut self) -> LogicalSize<u32> {
        self.aspect_ratio = (self.physical_size.1 + self.desired_gutter) as f32 / (self.physical_size.0 + self.desired_gutter) as f32;
        self.logical_size = self.physical_size;
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

    pub fn change_physical_size_smooth(&mut self, width: u32, height: u32, scale: f32, max_font: (u32, u32)) {
        self.scale_factor = scale;
        self.physical_size.0 = width;
        self.physical_size.1 = height;

        let mut desired_y = (width as f32 * self.aspect_ratio) as u32;
        desired_y -= desired_y % max_font.1;

        if desired_y < height {
            self.smooth_gutter_y = height - desired_y;
        } else {
            let mut desired_x = (height as f32 / self.aspect_ratio) as u32;
            desired_x -= desired_x % max_font.0;
            self.smooth_gutter_x = width - desired_x;
        }

        self.recalculate_coordinates();
    }

    fn recalculate_coordinates(&mut self) {
        let total_gutter = (self.desired_gutter as f32 * self.scale_factor) as u32;
        let half_gutter = total_gutter / 2;

        let (extra_left, extra_right) = if self.smooth_gutter_x % 2 == 0 {
            (self.smooth_gutter_x/2, self.smooth_gutter_x/2)
        } else {
            ((self.smooth_gutter_x/2)+1, self.smooth_gutter_x/2)
        };
        let (extra_top, extra_bottom) = if self.smooth_gutter_y % 2 == 0 {
            (self.smooth_gutter_y/2, self.smooth_gutter_y/2)
        } else {
            ((self.smooth_gutter_y/2)+1, self.smooth_gutter_y/2)
        };

        if total_gutter % 2 == 0 {
            self.gutter_left = half_gutter + extra_left;
            self.gutter_right = half_gutter + extra_right;
            self.gutter_top = half_gutter + extra_top;
            self.gutter_bottom = half_gutter + extra_bottom;
        } else {
            self.gutter_left = half_gutter + extra_left;
            self.gutter_right = half_gutter+1 + extra_right;
            self.gutter_top = half_gutter + extra_top;
            self.gutter_bottom = half_gutter+1 + extra_bottom;
        }

        self.available_width = self.physical_size.0 - (total_gutter + extra_left + extra_right);
        self.available_height = self.physical_size.1 - (total_gutter + extra_top + extra_bottom);
        self.resized = true;
    }

    pub fn pixel_to_screen(&self, x: u32, y: u32) -> (f32, f32) {
        (
            ((x as f32 / self.logical_size.0 as f32) * 2.0) - 1.0,
            ((y as f32 / self.logical_size.1 as f32) * 2.0) - 1.0,
        )
    }

    pub fn top_left_pixel(&self) -> (f32, f32) {
        self.pixel_to_screen(0, 0)
    }

    pub fn bottom_right_pixel(&self) -> (f32, f32) {
        self.pixel_to_screen(
            self.logical_size.0,
            self.logical_size.1
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

    pub fn get_resized_and_reset(&mut self) -> bool {
        let result = self.resized;
        self.resized = false;
        result
    }

    fn pixel_to_screen_physical(&self, x: u32, y: u32) -> (f32, f32) {
        (
            ((x as f32 / self.physical_size.0 as f32) * 2.0) - 1.0,
            ((y as f32 / self.physical_size.1 as f32) * 2.0) - 1.0,
        )
    }

    pub fn get_backing_buffer_output_coordinates(&self) -> (f32, f32, f32, f32) {
        let (left, bottom) = self.pixel_to_screen_physical(
            self.gutter_left,
            self.gutter_top,
        );
        let (right, top) = self.pixel_to_screen_physical(
            self.physical_size.0 - self.gutter_right,
            self.physical_size.1 - self.gutter_bottom,
        );
        (left, right, top, bottom)
    }
}
