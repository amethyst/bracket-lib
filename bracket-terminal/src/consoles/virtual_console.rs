//! A virtual console exists to store large amounts of arbitrary text,
//! which can then be "windowed" into actual consoles.

use crate::prelude::{
    string_to_cp437, to_cp437, BTerm, CharacterTranslationMode, ColoredTextSpans, Console,
    DrawBatch, FontCharType, TextAlign, Tile, XpLayer,
};
use bracket_color::prelude::*;
use bracket_geometry::prelude::{Point, Rect};
use std::any::Any;

pub struct VirtualConsole {
    pub width: u32,
    pub height: u32,

    pub tiles: Vec<Tile>,

    pub extra_clipping: Option<Rect>,
    pub translation: CharacterTranslationMode,
}

impl VirtualConsole {
    /// Creates a new virtual console of arbitrary dimensions.
    pub fn new(dimensions: Point) -> Self {
        let num_tiles: usize = (dimensions.x * dimensions.y) as usize;
        let mut console = VirtualConsole {
            width: dimensions.x as u32,
            height: dimensions.y as u32,
            tiles: Vec::with_capacity(num_tiles),
            extra_clipping: None,
            translation: CharacterTranslationMode::Codepage437,
        };
        for _ in 0..num_tiles {
            console.tiles.push(Tile {
                glyph: 0,
                fg: RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                bg: RGBA::from_f32(0.0, 0.0, 0.0, 1.0),
            });
        }
        console
    }

    /// Creates a new virtual console from a blob of text.
    /// Useful if you want to scroll through manuals!
    pub fn from_text(text: &str, width: usize) -> Self {
        let raw_lines = text.split('\n');
        let mut lines: Vec<String> = Vec::new();
        for line in raw_lines {
            let mut newline: String = String::from("");

            line.chars().for_each(|c| {
                newline.push(c);
                if newline.len() > width {
                    lines.push(newline.clone());
                    newline.clear();
                }
            });
            lines.push(newline.clone());
        }

        let num_tiles: usize = width * lines.len();
        let mut console = VirtualConsole {
            width: width as u32,
            height: lines.len() as u32,
            tiles: Vec::with_capacity(num_tiles),
            extra_clipping: None,
            translation: CharacterTranslationMode::Codepage437,
        };
        //println!("{}x{}", console.width, console.height);

        for _ in 0..num_tiles {
            console.tiles.push(Tile {
                glyph: 0,
                fg: RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                bg: RGBA::from_f32(0.0, 0.0, 0.0, 1.0),
            });
        }

        for (i, line) in lines.iter().enumerate() {
            console.print(0, i as i32, &line);
        }

        console
    }

    /// Send a portion of the Virtual Console to a physical console, specifying both source and destination
    /// rectangles and the target console.
    pub fn print_sub_rect(&self, source: Rect, dest: Rect, target: &mut BTerm) {
        target.set_clipping(Some(dest));
        for y in dest.y1..dest.y2 {
            let source_y = y + source.y1 - dest.y1;
            for x in dest.x1..dest.x2 {
                let source_x = x + source.x1 - dest.x1;
                if let Some(idx) = self.try_at(source_x, source_y) {
                    let t = self.tiles[idx];
                    if t.glyph > 0 {
                        target.set(x, y, t.fg, t.bg, t.glyph);
                    }
                }
            }
        }
        target.set_clipping(None);
    }

    /// Send a portion of the Virtual Console to a render batch, specifying both source and destination
    /// rectangles and the target batch.
    pub fn batch_sub_rect(&self, source: Rect, dest: Rect, target: &mut DrawBatch) {
        target.set_clipping(Some(dest));
        for y in dest.y1..dest.y2 {
            let source_y = y + source.y1 - dest.y1;
            for x in dest.x1..dest.x2 {
                let source_x = x + source.x1 - dest.x1;
                if let Some(idx) = self.try_at(source_x, source_y) {
                    let t = self.tiles[idx];
                    if t.glyph > 0 {
                        target.set(Point::new(x, y), ColorPair::new(t.fg, t.bg), t.glyph);
                    }
                }
            }
        }
        target.set_clipping(None);
    }
}

impl Console for VirtualConsole {
    fn get_char_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn resize_pixels(&mut self, _width: u32, _height: u32) {
        // Ignored
    }

    /// Translate an x/y into an array index.
    fn at(&self, x: i32, y: i32) -> usize {
        (((self.height - 1 - y as u32) * self.width) + x as u32) as usize
    }

    /// Clears the screen.
    fn cls(&mut self) {
        for tile in &mut self.tiles {
            tile.glyph = 32;
            tile.fg = RGBA::from_f32(1.0, 1.0, 1.0, 1.0);
            tile.bg = RGBA::from_f32(0.0, 0.0, 0.0, 1.0);
        }
    }

    /// Clears the screen with a background color.
    fn cls_bg(&mut self, background: RGBA) {
        for tile in &mut self.tiles {
            tile.glyph = 32;
            tile.fg = RGBA::from_f32(1.0, 1.0, 1.0, 1.0);
            tile.bg = background;
        }
    }

    /// Prints a string at x/y.
    fn print(&mut self, mut x: i32, y: i32, output: &str) {
        let bytes = match self.translation {
            CharacterTranslationMode::Codepage437 => string_to_cp437(output),
            CharacterTranslationMode::Unicode => {
                output.chars().map(|c| c as FontCharType).collect()
            }
        };
        for glyph in bytes {
            if let Some(idx) = self.try_at(x, y) {
                self.tiles[idx].glyph = glyph;
            }
            x += 1;
        }
    }

    /// Prints a string at x/y, with foreground and background colors.
    fn print_color(&mut self, mut x: i32, y: i32, fg: RGBA, bg: RGBA, output: &str) {
        let bytes = match self.translation {
            CharacterTranslationMode::Codepage437 => string_to_cp437(output),
            CharacterTranslationMode::Unicode => {
                output.chars().map(|c| c as FontCharType).collect()
            }
        };
        for glyph in bytes {
            if let Some(idx) = self.try_at(x, y) {
                self.tiles[idx].glyph = glyph;
                self.tiles[idx].bg = bg;
                self.tiles[idx].fg = fg;
            }
            x += 1;
        }
    }

    /// Sets a single cell in the console
    fn set(&mut self, x: i32, y: i32, fg: RGBA, bg: RGBA, glyph: FontCharType) {
        if let Some(idx) = self.try_at(x, y) {
            self.tiles[idx].glyph = glyph;
            self.tiles[idx].fg = fg;
            self.tiles[idx].bg = bg;
        }
    }

    /// Sets a single cell in the console's background
    fn set_bg(&mut self, x: i32, y: i32, bg: RGBA) {
        if let Some(idx) = self.try_at(x, y) {
            self.tiles[idx].bg = bg;
        }
    }

    /// Draws a box, starting at x/y with the extents width/height using CP437 line characters
    fn draw_box(&mut self, sx: i32, sy: i32, width: i32, height: i32, fg: RGBA, bg: RGBA) {
        crate::prelude::draw_box(self, sx, sy, width, height, fg, bg);
    }

    /// Draws a box, starting at x/y with the extents width/height using CP437 line characters
    fn draw_hollow_box(&mut self, sx: i32, sy: i32, width: i32, height: i32, fg: RGBA, bg: RGBA) {
        crate::prelude::draw_hollow_box(self, sx, sy, width, height, fg, bg);
    }

    /// Draws a box, starting at x/y with the extents width/height using CP437 double line characters
    fn draw_box_double(&mut self, sx: i32, sy: i32, width: i32, height: i32, fg: RGBA, bg: RGBA) {
        crate::prelude::draw_box_double(self, sx, sy, width, height, fg, bg);
    }

    /// Draws a box, starting at x/y with the extents width/height using CP437 double line characters
    fn draw_hollow_box_double(
        &mut self,
        sx: i32,
        sy: i32,
        width: i32,
        height: i32,
        fg: RGBA,
        bg: RGBA,
    ) {
        crate::prelude::draw_hollow_box_double(self, sx, sy, width, height, fg, bg);
    }

    /// Fills a rectangle with the specified rendering information
    fn fill_region(&mut self, target: Rect, glyph: FontCharType, fg: RGBA, bg: RGBA) {
        target.for_each(|point| {
            self.set(point.x, point.y, fg, bg, glyph);
        });
    }

    /// Draws a horizontal progress bar
    fn draw_bar_horizontal(
        &mut self,
        sx: i32,
        sy: i32,
        width: i32,
        n: i32,
        max: i32,
        fg: RGBA,
        bg: RGBA,
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
        fg: RGBA,
        bg: RGBA,
    ) {
        crate::prelude::draw_bar_vertical(self, sx, sy, height, n, max, fg, bg);
    }

    /// Prints text, centered to the whole console width, at vertical location y.
    fn print_centered(&mut self, y: i32, text: &str) {
        self.print(
            (self.width as i32 / 2) - (text.to_string().len() as i32 / 2),
            y,
            text,
        );
    }

    /// Prints text in color, centered to the whole console width, at vertical location y.
    fn print_color_centered(&mut self, y: i32, fg: RGBA, bg: RGBA, text: &str) {
        self.print_color(
            (self.width as i32 / 2) - (text.to_string().len() as i32 / 2),
            y,
            fg,
            bg,
            text,
        );
    }

    /// Prints text, centered to the whole console width, at vertical location y.
    fn print_centered_at(&mut self, x: i32, y: i32, text: &str) {
        self.print(x - (text.to_string().len() as i32 / 2), y, text);
    }

    /// Prints text in color, centered to the whole console width, at vertical location y.
    fn print_color_centered_at(&mut self, x: i32, y: i32, fg: RGBA, bg: RGBA, text: &str) {
        self.print_color(x - (text.to_string().len() as i32 / 2), y, fg, bg, text);
    }

    /// Prints text right-aligned
    fn print_right(&mut self, x: i32, y: i32, text: &str) {
        let len = text.len() as i32;
        let actual_x = x - len;
        self.print(actual_x, y, text);
    }

    /// Prints colored text right-aligned
    fn print_color_right(&mut self, x: i32, y: i32, fg: RGBA, bg: RGBA, text: &str) {
        let len = text.len() as i32;
        let actual_x = x - len;
        self.print_color(actual_x, y, fg, bg, text);
    }

    /// Print a colorized string with the color encoding defined inline.
    /// For example: printer(1, 1, "#[blue]This blue text contains a #[pink]pink#[] word")
    /// You can get the same effect with a TextBlock, but this can be easier.
    /// Thanks to doryen_rs for the idea.
    fn printer(
        &mut self,
        x: i32,
        y: i32,
        output: &str,
        align: TextAlign,
        background: Option<RGBA>,
    ) {
        let bg = if let Some(bg) = background {
            bg
        } else {
            RGBA::from_f32(0.0, 0.0, 0.0, 1.0)
        };

        let split_text = ColoredTextSpans::new(output);

        let mut tx = match align {
            TextAlign::Left => x,
            TextAlign::Center => x - (split_text.length as i32 / 2),
            TextAlign::Right => x - split_text.length as i32,
        };
        for span in split_text.spans.iter() {
            let fg = span.0;
            for ch in span.1.chars() {
                self.set(
                    tx,
                    y,
                    fg,
                    bg,
                    match self.translation {
                        CharacterTranslationMode::Codepage437 => to_cp437(ch),
                        CharacterTranslationMode::Unicode => ch as FontCharType,
                    },
                );
                tx += 1;
            }
        }
    }

    /// Saves the layer to an XpFile structure
    fn to_xp_layer(&self) -> XpLayer {
        let mut layer = XpLayer::new(self.width as usize, self.height as usize);

        for y in 0..self.height {
            for x in 0..self.width {
                let cell = layer.get_mut(x as usize, y as usize).unwrap();
                let idx = self.at(x as i32, y as i32);
                cell.ch = u32::from(self.tiles[idx].glyph);
                cell.fg = self.tiles[idx].fg.to_xp();
                cell.bg = self.tiles[idx].bg.to_xp();
            }
        }

        layer
    }

    /// Sets an offset to total console rendering, useful for layers that
    /// draw between tiles. Offsets are specified as a percentage of total
    /// character size; so -0.5 will offset half a character to the left/top.
    fn set_offset(&mut self, _x: f32, _y: f32) {
        panic!("Unsupported on virtual consoles.");
    }

    fn set_scale(&mut self, _scale: f32, _center_x: i32, _center_y: i32) {
        panic!("Unsupported on virtual consoles.");
    }

    fn get_scale(&self) -> (f32, i32, i32) {
        (1.0, 0, 0)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    /// Permits the creation of an arbitrary clipping rectangle. It's a really good idea
    /// to make sure that this rectangle is entirely valid.
    fn set_clipping(&mut self, clipping: Option<Rect>) {
        self.extra_clipping = clipping;
    }

    /// Returns the current arbitrary clipping rectangle, None if there isn't one.
    fn get_clipping(&self) -> Option<Rect> {
        self.extra_clipping
    }

    /// Sets ALL tiles foreground alpha (only tiles that exist, in sparse consoles).
    fn set_all_fg_alpha(&mut self, alpha: f32) {
        self.tiles.iter_mut().for_each(|t| t.fg.a = alpha);
    }

    /// Sets ALL tiles background alpha (only tiles that exist, in sparse consoles).
    fn set_all_bg_alpha(&mut self, alpha: f32) {
        self.tiles.iter_mut().for_each(|t| t.bg.a = alpha);
    }

    /// Sets ALL tiles foreground alpha (only tiles that exist, in sparse consoles).
    fn set_all_alpha(&mut self, fg: f32, bg: f32) {
        self.tiles.iter_mut().for_each(|t| {
            t.fg.a = fg;
            t.bg.a = bg;
        });
    }

    /// Sets the character translation mode
    fn set_translation_mode(&mut self, mode: CharacterTranslationMode) {
        self.translation = mode;
    }

    /// Sets the character size of the terminal
    fn set_char_size(&mut self, _width: u32, _height: u32) {
        panic!("Not implemented.");
    }

    // Clears the dirty bit
    fn clear_dirty(&mut self) { }
}
