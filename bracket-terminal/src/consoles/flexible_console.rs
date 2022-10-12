use crate::prelude::{
    string_to_cp437, to_cp437, CharacterTranslationMode, ColoredTextSpans, Console, FontCharType,
    TextAlign, XpLayer,
};
use bracket_color::prelude::RGBA;
use bracket_geometry::prelude::{PointF, Rect};
use bracket_rex::prelude::XpColor;
use ultraviolet::Vec2;

use std::any::Any;

/// Internal storage structure for sparse tiles.
pub struct FlexiTile {
    pub position: PointF,
    pub z_order: i32,
    pub glyph: FontCharType,
    pub fg: RGBA,
    pub bg: RGBA,
    pub rotation: f32,
    pub scale: PointF,
}

/// A sparse console. Rather than storing every cell on the screen, it stores just cells that have
/// data.
pub struct FlexiConsole {
    pub width: u32,
    pub height: u32,

    pub tiles: Vec<FlexiTile>,
    pub is_dirty: bool,

    // To handle offset tiles for people who want thin walls between tiles
    pub offset_x: f32,
    pub offset_y: f32,

    pub scale: f32,
    pub scale_center: (i32, i32),

    pub extra_clipping: Option<Rect>,
    pub translation: CharacterTranslationMode,
    pub(crate) needs_resize_internal: bool,
}

impl FlexiConsole {
    /// Initializes the console.
    pub fn init(width: u32, height: u32) -> Box<FlexiConsole> {
        // Console backing initialization
        let new_console = FlexiConsole {
            width,
            height,
            tiles: Vec::with_capacity((width * height) as usize),
            is_dirty: true,
            offset_x: 0.0,
            offset_y: 0.0,
            scale: 1.0,
            scale_center: (width as i32 / 2, height as i32 / 2),
            extra_clipping: None,
            translation: CharacterTranslationMode::Codepage437,
            needs_resize_internal: false,
        };

        Box::new(new_console)
    }

    // Insert a single tile with "fancy" attributes
    #[allow(clippy::too_many_arguments)]
    pub fn set_fancy(
        &mut self,
        position: PointF,
        z_order: i32,
        rotation: f32,
        scale: PointF,
        fg: RGBA,
        bg: RGBA,
        glyph: FontCharType,
    ) {
        self.is_dirty = true;
        let invert_pos = PointF {
            x: position.x,
            y: self.height as f32 - position.y,
        };
        self.tiles.push(FlexiTile {
            position: invert_pos,
            z_order,
            glyph,
            fg,
            bg,
            rotation,
            scale,
        });
    }
}

impl Console for FlexiConsole {
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

        self.tiles.push(FlexiTile {
            glyph: 32,
            fg: RGBA::from_u8(255, 255, 255, 255),
            bg: RGBA::from_u8(0, 0, 0, 255),
            rotation: 0.,
            scale: Vec2::new(0., 0.),
            z_order: 0,
            position: Vec2::new(0., 0.),
        });
    }

    /// Clear the screen. Since we don't HAVE a background, it doesn't use it.
    fn cls_bg(&mut self, _background: RGBA) {
        self.is_dirty = true;
        for tile in &mut self.tiles {
            tile.glyph = 32;
            tile.fg = RGBA::from_u8(255, 255, 255, 255);
            tile.bg = RGBA::from_u8(0, 0, 0, 255);
        }
    }

    /// Prints a string to an x/y position.
    fn print(&mut self, x: i32, y: i32, output: &str) {
        self.is_dirty = true;

        let bytes = match self.translation {
            CharacterTranslationMode::Codepage437 => string_to_cp437(output),
            CharacterTranslationMode::Unicode => {
                output.chars().map(|c| c as FontCharType).collect()
            }
        };

        let h = (self.height - 1) as f32;
        self.tiles
            .extend(bytes.into_iter().enumerate().map(|(i, glyph)| FlexiTile {
                position: PointF {
                    x: i as f32 + x as f32,
                    y: h - y as f32,
                },
                z_order: 0,
                glyph,
                fg: RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                bg: RGBA::from_f32(0.0, 0.0, 0.0, 1.0),
                rotation: 0.0,
                scale: PointF { x: 1.0, y: 1.0 },
            }));
    }

    /// Prints a string to an x/y position, with foreground and background colors.
    fn print_color(&mut self, x: i32, y: i32, fg: RGBA, bg: RGBA, output: &str) {
        self.is_dirty = true;

        let bytes = match self.translation {
            CharacterTranslationMode::Codepage437 => string_to_cp437(output),
            CharacterTranslationMode::Unicode => {
                output.chars().map(|c| c as FontCharType).collect()
            }
        };
        let h = (self.height - 1) as f32;
        self.tiles
            .extend(bytes.into_iter().enumerate().map(|(i, glyph)| FlexiTile {
                z_order: 0,
                position: PointF {
                    x: i as f32 + x as f32,
                    y: h - y as f32,
                },
                glyph,
                fg,
                bg,
                rotation: 0.0,
                scale: PointF { x: 1.0, y: 1.0 },
            }));
    }

    /// Sets a single cell in the console
    fn set(&mut self, x: i32, y: i32, fg: RGBA, bg: RGBA, glyph: FontCharType) {
        self.is_dirty = true;
        if self.try_at(x, y).is_some() {
            let h = (self.height - 1) as f32;
            self.tiles.push(FlexiTile {
                position: PointF {
                    x: x as f32,
                    y: h - y as f32,
                },
                z_order: 0,
                glyph,
                fg,
                bg,
                rotation: 0.0,
                scale: PointF { x: 1.0, y: 1.0 },
            });
        }
    }

    /// Sets a single cell in the console's background
    fn set_bg(&mut self, _x: i32, _y: i32, _bg: RGBA) {
        // Does nothing for this layer type
    }

    /// Draws a box, starting at x/y with the extents width/height using CP437 line characters
    fn draw_box(&mut self, sx: i32, sy: i32, width: i32, height: i32, fg: RGBA, bg: RGBA) {
        crate::prelude::draw_box(self, sx, sy, width, height, fg, bg);
    }

    /// Draws a box, starting at x/y with the extents width/height using CP437 double line characters
    fn draw_box_double(&mut self, sx: i32, sy: i32, width: i32, height: i32, fg: RGBA, bg: RGBA) {
        crate::prelude::draw_box_double(self, sx, sy, width, height, fg, bg);
    }

    /// Draws a box, starting at x/y with the extents width/height using CP437 line characters
    fn draw_hollow_box(&mut self, sx: i32, sy: i32, width: i32, height: i32, fg: RGBA, bg: RGBA) {
        crate::prelude::draw_hollow_box(self, sx, sy, width, height, fg, bg);
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
        self.is_dirty = true;
        self.print(
            (self.width as i32 / 2) - (text.to_string().len() as i32 / 2),
            y,
            text,
        );
    }

    /// Prints text in color, centered to the whole console width, at vertical location y.
    fn print_color_centered(&mut self, y: i32, fg: RGBA, bg: RGBA, text: &str) {
        self.is_dirty = true;
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
        self.is_dirty = true;
        self.print(x - (text.to_string().len() as i32 / 2), y, text);
    }

    /// Prints text in color, centered to the whole console width, at vertical location y.
    fn print_color_centered_at(&mut self, x: i32, y: i32, fg: RGBA, bg: RGBA, text: &str) {
        self.is_dirty = true;
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
            RGBA::from_u8(0, 0, 0, 255)
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

        // Clear all to transparent
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = layer.get_mut(x as usize, y as usize).unwrap();
                cell.bg = XpColor::TRANSPARENT;
            }
        }

        for c in &self.tiles {
            let x = c.position.x as usize;
            let y = c.position.y as usize;
            let cell = layer.get_mut(x as usize, y as usize).unwrap();
            cell.ch = u32::from(c.glyph);
            cell.fg = XpColor::from(c.fg);
            cell.bg = XpColor::from(c.bg);
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

    fn get_scale(&self) -> (f32, i32, i32) {
        (self.scale, self.scale_center.0, self.scale_center.1)
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
    fn set_char_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.needs_resize_internal = true;
    }

    // Clears the dirty bit
    fn clear_dirty(&mut self) {
        self.is_dirty = false;
    }
}
