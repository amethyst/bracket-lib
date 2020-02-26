use crate::prelude::{string_to_cp437, Console, XpLayer};
use bracket_color::prelude::{XpColor, RGB};
use bracket_geometry::prelude::Rect;
use std::any::Any;

/// Internal storage structure for sparse tiles.
pub struct SparseTile {
    pub idx: usize,
    pub glyph: u8,
    pub fg: RGB,
    pub bg: RGB,
}

/// A sparse console. Rather than storing every cell on the screen, it stores just cells that have
/// data.
pub struct SparseConsole {
    pub width: u32,
    pub height: u32,

    pub tiles: Vec<SparseTile>,
    pub is_dirty: bool,

    // To handle offset tiles for people who want thin walls between tiles
    pub offset_x: f32,
    pub offset_y: f32,

    pub scale: f32,
    pub scale_center: (i32, i32),
}

impl SparseConsole {
    /// Initializes the console.
    pub fn init(width: u32, height: u32) -> Box<SparseConsole> {
        // Console backing init
        let new_console = SparseConsole {
            width,
            height,
            tiles: Vec::with_capacity((width * height) as usize),
            is_dirty: true,
            offset_x: 0.0,
            offset_y: 0.0,
            scale: 1.0,
            scale_center: (width as i32 / 2, height as i32 / 2),
        };

        Box::new(new_console)
    }
}

impl Console for SparseConsole {
    fn get_char_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn resize_pixels(&mut self, _width: u32, _height: u32) {
        self.is_dirty = true;
    }

    /// Translates x/y to an index entry. Not really useful.
    fn at(&self, x: i32, y: i32) -> usize {
        (((self.height - 1 - y as u32) * self.width) + x as u32) as usize
    }

    /// Clear the screen.
    fn cls(&mut self) {
        self.is_dirty = true;
        self.tiles.clear();
    }

    /// Clear the screen. Since we don't HAVE a background, it doesn't use it.
    fn cls_bg(&mut self, _background: RGB) {
        self.is_dirty = true;
        self.tiles.clear();
    }

    /// Prints a string to an x/y position.
    fn print(&mut self, x: i32, y: i32, output: &str) {
        self.is_dirty = true;

        let bounds = self.get_char_size();
        let bytes = string_to_cp437(output);

        self.tiles.extend(
            bytes
                .into_iter()
                .enumerate()
                .filter(|(i, _)| (*i as i32 + x) < bounds.0 as i32)
                .map(|(i, glyph)| {
                    let idx =
                        (((bounds.1 - 1 - y as u32) * bounds.0) + (x + i as i32) as u32) as usize;
                    SparseTile {
                        idx,
                        glyph,
                        fg: RGB::from_f32(1.0, 1.0, 1.0),
                        bg: RGB::from_f32(0.0, 0.0, 0.0),
                    }
                }),
        );
    }

    /// Prints a string to an x/y position, with foreground and background colors.
    fn print_color(&mut self, x: i32, y: i32, fg: RGB, bg: RGB, output: &str) {
        self.is_dirty = true;

        let bounds = self.get_char_size();
        let bytes = string_to_cp437(output);
        self.tiles.extend(
            bytes
                .into_iter()
                .enumerate()
                .filter(|(i, _)| (*i as i32 + x) < bounds.0 as i32)
                .map(|(i, glyph)| {
                    let idx =
                        (((bounds.1 - 1 - y as u32) * bounds.0) + (x + i as i32) as u32) as usize;
                    SparseTile { idx, glyph, fg, bg }
                }),
        );
    }

    /// Sets a single cell in the console
    fn set(&mut self, x: i32, y: i32, fg: RGB, bg: RGB, glyph: u8) {
        self.is_dirty = true;
        if let Some(idx) = self.try_at(x, y) {
            self.tiles.push(SparseTile { idx, glyph, fg, bg });
        }
    }

    /// Sets a single cell in the console's background
    fn set_bg(&mut self, x: i32, y: i32, bg: RGB) {
        if let Some(idx) = self.try_at(x, y) {
            self.is_dirty = true;
            self.tiles[idx].bg = bg;
        }
    }

    /// Draws a box, starting at x/y with the extents width/height using CP437 line characters
    fn draw_box(&mut self, sx: i32, sy: i32, width: i32, height: i32, fg: RGB, bg: RGB) {
        crate::prelude::draw_box(self, sx, sy, width, height, fg, bg);
    }

    /// Draws a box, starting at x/y with the extents width/height using CP437 double line characters
    fn draw_box_double(&mut self, sx: i32, sy: i32, width: i32, height: i32, fg: RGB, bg: RGB) {
        crate::prelude::draw_box_double(self, sx, sy, width, height, fg, bg);
    }

    /// Draws a box, starting at x/y with the extents width/height using CP437 line characters
    fn draw_hollow_box(&mut self, sx: i32, sy: i32, width: i32, height: i32, fg: RGB, bg: RGB) {
        crate::prelude::draw_hollow_box(self, sx, sy, width, height, fg, bg);
    }

    /// Draws a box, starting at x/y with the extents width/height using CP437 double line characters
    fn draw_hollow_box_double(
        &mut self,
        sx: i32,
        sy: i32,
        width: i32,
        height: i32,
        fg: RGB,
        bg: RGB,
    ) {
        crate::prelude::draw_hollow_box_double(self, sx, sy, width, height, fg, bg);
    }

    /// Fills a rectangle with the specified rendering information
    fn fill_region(&mut self, target: Rect, glyph: u8, fg: RGB, bg: RGB) {
        target.for_each(|point| {
            self.set(point.x, point.y, fg, bg, glyph);
        });
    }

    fn get(&self, x: i32, y: i32) -> Option<(&u8, &RGB, &RGB)> {
        let idx = self.at(x, y);
        for t in self.tiles.iter().filter(|t| t.idx == idx) {
            return Some((&t.glyph, &t.fg, &t.bg));
        }
        None
    }

    /// Draws a horizontal progress bar
    fn draw_bar_horizontal(
        &mut self,
        sx: i32,
        sy: i32,
        width: i32,
        n: i32,
        max: i32,
        fg: RGB,
        bg: RGB,
    ) {
        crate::prelude::draw_bar_horizontal(self, sx, sy, width, n, max, fg, bg);
    }

    /// Draws a vertical progress bar
    fn draw_bar_vertical(
        &mut self,
        sx: i32,
        sy: i32,
        height: i32,
        n: i32,
        max: i32,
        fg: RGB,
        bg: RGB,
    ) {
        crate::prelude::draw_bar_vertical(self, sx, sy, height, n, max, fg, bg);
    }

    /// Prints text, centered to the whole console width, at vertical location y.
    fn print_centered(&mut self, y: i32, text: &str) {
        self.is_dirty = true;
        self.print(
            (self.width as i32 / 2) - (text.to_string().len() as i32 / 2),
            y,
            text,
        );
    }

    /// Prints text in color, centered to the whole console width, at vertical location y.
    fn print_color_centered(&mut self, y: i32, fg: RGB, bg: RGB, text: &str) {
        self.is_dirty = true;
        self.print_color(
            (self.width as i32 / 2) - (text.to_string().len() as i32 / 2),
            y,
            fg,
            bg,
            text,
        );
    }

    /// Saves the layer to an XpFile structure
    fn to_xp_layer(&self) -> XpLayer {
        let mut layer = XpLayer::new(self.width as usize, self.height as usize);

        // Clear all to transparent
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = layer.get_mut(x as usize, y as usize).unwrap();
                cell.bg = XpColor::TRANSPARENT;
            }
        }

        for c in &self.tiles {
            let x = c.idx % self.width as usize;
            let y = c.idx / self.width as usize;
            let cell = layer.get_mut(x as usize, y as usize).unwrap();
            cell.ch = u32::from(c.glyph);
            cell.fg = c.fg.to_xp();
            cell.bg = c.bg.to_xp();
        }

        layer
    }

    /// Sets an offset to total console rendering, useful for layers that
    /// draw between tiles. Offsets are specified as a percentage of total
    /// character size; so -0.5 will offset half a character to the left/top.
    fn set_offset(&mut self, x: f32, y: f32) {
        self.is_dirty = true;
        self.offset_x = x * (2.0 / self.width as f32);
        self.offset_y = y * (2.0 / self.height as f32);
    }

    fn set_scale(&mut self, scale: f32, center_x: i32, center_y: i32) {
        self.is_dirty = true;
        self.scale = scale;
        self.scale_center = (center_x, center_y);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
