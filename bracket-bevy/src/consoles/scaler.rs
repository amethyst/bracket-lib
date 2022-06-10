pub(crate) struct FontScaler {
    chars_per_row: u16,
    n_rows: u16,
    font_height_pixels: (f32, f32),
}

impl FontScaler {
    pub(crate) fn new(chars_per_row: u16, n_rows: u16, font_height_pixels: (f32, f32)) -> Self {
        Self {
            chars_per_row,
            n_rows,
            font_height_pixels,
        }
    }

    pub(crate) fn texture_coords(&self, glyph: u16) -> [f32; 4] {
        let half_x_pixel = (1.0 / (self.chars_per_row as f32 * self.font_height_pixels.0)) / 4.0;
        let half_y_pixel = (1.0 / (self.n_rows as f32 * self.font_height_pixels.1)) / 4.0;
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

pub(crate) struct ScreenScaler {
    pub(crate) screen: (f32, f32),
    desired_gutter: f32,
    x_gutter: f32,
    y_gutter: f32,
}

impl Default for ScreenScaler {
    fn default() -> Self {
        let desired_gutter = default_gutter_size();
        Self {
            screen: (0.0, 0.0),
            desired_gutter,
            x_gutter: desired_gutter / 2.0,
            y_gutter: desired_gutter / 2.0,
        }
    }
}

impl ScreenScaler {
    pub(crate) fn new(desired_gutter: f32) -> Self {
        Self {
            screen: (0.0, 0.0),
            desired_gutter,
            x_gutter: desired_gutter / 2.0,
            y_gutter: desired_gutter / 2.0,
        }
    }

    pub(crate) fn set_screen_size(&mut self, width: f32, height: f32) {
        self.screen = (width, height);
        self.x_gutter = self.desired_gutter / 2.0;
        self.y_gutter = self.desired_gutter / 2.0;
    }

    pub(crate) fn recalculate(
        &mut self,
        terminal_pixel_size: (f32, f32),
        largest_font: (f32, f32),
    ) {
        let aspect_ratio = terminal_pixel_size.0 / terminal_pixel_size.1;
        let perfect_height =
            (self.screen.0 / aspect_ratio) - (self.screen.1 as u32 % largest_font.1 as u32) as f32;
        if perfect_height < self.screen.1 {
            self.y_gutter = self.screen.1 - perfect_height;
        } else {
            let perfect_width = (self.screen.1 * aspect_ratio)
                - (self.screen.0 as u32 % largest_font.0 as u32) as f32;
            self.x_gutter = self.screen.0 - perfect_width;
        }
    }

    pub(crate) fn top_left(&self) -> (f32, f32) {
        (
            0.0 - (self.screen.0 / 2.0) + (self.x_gutter as f32 / 2.0),
            0.0 - (self.screen.1 / 2.0) + (self.y_gutter as f32 / 2.0),
        )
    }

    pub(crate) fn calc_step(&self, width: usize, height: usize) -> (f32, f32) {
        (
            (self.screen.0 - self.x_gutter as f32) / width as f32,
            (self.screen.1 - self.y_gutter as f32) / height as f32,
        )
    }

    pub(crate) fn available_size(&self) -> (f32, f32) {
        (self.screen.0 - self.x_gutter, self.screen.1 - self.y_gutter)
    }

    pub(crate) fn calc_mouse_position(
        &self,
        pos: (f32, f32),
        width: usize,
        height: usize,
    ) -> (usize, usize) {
        let step = self.calc_step(width, height);
        let step_pos = (
            (pos.0 / step.0) + (width as f32 / 2.0),
            (pos.1 / step.1) + (height as f32 / 2.0),
        );
        (
            usize::clamp(step_pos.0 as usize, 0, width - 1),
            usize::clamp(height - step_pos.1 as usize, 0, height - 1),
        )
    }
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
pub(crate) fn default_gutter_size() -> f32 {
    // Testing showed that an 8-pixel gutter is enough to fix
    // Big Sur and Windows 11.
    8.0
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
pub(crate) fn default_gutter_size() -> f32 {
    // Testing showed that an 8-pixel gutter is enough to fix
    // Big Sur and Windows 11.
    0.0
}
