use super::{gui_helpers, hal, rex::XpColor, rex::XpLayer, Console, Font, Shader, RGB};
//use glow::types::*;
use glow::HasContext;
use std::mem;

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

    // Private
    tiles: Vec<SparseTile>,
    is_dirty: bool,

    // To handle offset tiles for people who want thin walls between tiles
    offset_x: f32,
    offset_y: f32,

    backend: hal::SparseConsoleBackend,
}

impl SparseConsole {
    /// Initializes the console.
    pub fn init(width: u32, height: u32, gl: &glow::Context) -> Box<SparseConsole> {
        // Console backing init
        let new_console = SparseConsole {
            width,
            height,
            tiles: Vec::new(),
            is_dirty: true,
            offset_x: 0.0,
            offset_y: 0.0,
            backend: hal::SparseConsoleBackend::new(gl),
        };

        Box::new(new_console)
    }

    fn rebuild_vertices(&mut self, gl: &glow::Context) {
        self.backend.rebuild_vertices(
            gl,
            self.height,
            self.width,
            self.offset_x,
            self.offset_y,
            &self.tiles,
        );
    }
}

impl Console for SparseConsole {
    /// If the console has changed, rebuild the vertex buffer.
    fn rebuild_if_dirty(&mut self, gl: &glow::Context) {
        if self.is_dirty {
            self.rebuild_vertices(gl);
            self.is_dirty = false;
        }
    }

    fn get_char_size(&mut self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn resize_pixels(&mut self, _width: u32, _height: u32) {
        self.is_dirty = true;
    }

    /// Draws the console to OpenGL.
    fn gl_draw(&mut self, font: &Font, shader: &Shader, gl: &glow::Context) {
        self.backend.gl_draw(font, shader, gl, &self.tiles);
        self.is_dirty = false;
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
        let mut idx = self.at(x, y);

        let bytes = super::string_to_cp437(output);

        self.tiles.extend(bytes.into_iter().map(|glyph| {
            let tile = SparseTile {
                idx,
                glyph,
                fg: RGB::from_f32(1.0, 1.0, 1.0),
                bg: RGB::from_f32(0.0, 0.0, 0.0),
            };
            idx += 1;
            tile
        }));
    }

    /// Prints a string to an x/y position, with foreground and background colors.
    fn print_color(&mut self, x: i32, y: i32, fg: RGB, bg: RGB, output: &str) {
        self.is_dirty = true;
        let mut idx = self.at(x, y);

        let bytes = super::string_to_cp437(output);
        self.tiles.extend(bytes.into_iter().map(|glyph| {
            let tile = SparseTile { idx, glyph, fg, bg };
            idx += 1;
            tile
        }));
    }

    /// Sets a single cell in the console
    fn set(&mut self, x: i32, y: i32, fg: RGB, bg: RGB, glyph: u8) {
        let idx = self.at(x, y);
        self.tiles.push(SparseTile { idx, glyph, fg, bg });
    }

    /// Sets a single cell in the console's background
    fn set_bg(&mut self, x: i32, y: i32, bg: RGB) {
        let idx = self.at(x, y);
        self.tiles[idx].bg = bg;
    }

    /// Draws a box, starting at x/y with the extents width/height using CP437 line characters
    fn draw_box(&mut self, sx: i32, sy: i32, width: i32, height: i32, fg: RGB, bg: RGB) {
        gui_helpers::draw_box(self, sx, sy, width, height, fg, bg);
    }

    /// Draws a box, starting at x/y with the extents width/height using CP437 double line characters
    fn draw_box_double(&mut self, sx: i32, sy: i32, width: i32, height: i32, fg: RGB, bg: RGB) {
        gui_helpers::draw_box_double(self, sx, sy, width, height, fg, bg);
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
        gui_helpers::draw_bar_horizontal(self, sx, sy, width, n, max, fg, bg);
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
        gui_helpers::draw_bar_vertical(self, sx, sy, height, n, max, fg, bg);
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
        self.offset_x = x * (2.0 / self.width as f32);
        self.offset_y = y * (2.0 / self.height as f32);
    }
}
