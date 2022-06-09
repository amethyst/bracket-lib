pub(crate) struct FontScaler {
    chars_per_row: u16,
    n_rows: u16,
    font_height_pixels: (f32, f32),
}

impl FontScaler {
    pub(crate) fn new(chars_per_row: u16, n_rows: u16, font_height_pixels: (f32, f32)) -> Self {
        Self { chars_per_row, n_rows, font_height_pixels }
    }

    pub(crate) fn texture_coords(&self, glyph: u16) -> [f32; 4] {
        let half_x_pixel = (1.0 / (self.chars_per_row as f32 * self.font_height_pixels.0)) / 8.0;
        let half_y_pixel = (1.0 / (self.n_rows as f32 * self.font_height_pixels.1)) / 8.0;
        let base_x = glyph % self.chars_per_row;
        let base_y = glyph / self.n_rows;
        let scale_x = 1.0 / self.chars_per_row as f32;
        let scale_y = 1.0 / self.n_rows as f32;
        [
            (base_x as f32 * scale_x) + half_x_pixel,
            (base_y as f32 * scale_y) + half_y_pixel,
            ((base_x + 1) as f32 * scale_x) - half_x_pixel,
            ((base_y + 1) as f32 * scale_y) - half_y_pixel,
        ]
    }
}