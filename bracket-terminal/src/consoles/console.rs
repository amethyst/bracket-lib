use crate::prelude::{FontCharType, XpLayer};
use bracket_color::prelude::RGBA;
use bracket_geometry::prelude::{Point, Rect};
use std::any::Any;

/// The internal storage type for tiles in a simple console.
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Tile {
    pub glyph: FontCharType,
    pub fg: RGBA,
    pub bg: RGBA,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum CharacterTranslationMode {
    Codepage437,
    Unicode,
}

/// Trait that must be implemented by console types.
pub trait Console {
    /// Gets the dimensions of the console in characters
    fn get_char_size(&self) -> (u32, u32);

    // Resizes the viewport
    fn resize_pixels(&mut self, width: u32, height: u32);

    /// Converts an x/y coordinate to a console index number.
    fn at(&self, x: i32, y: i32) -> usize;

    /// Clear the console.
    fn cls(&mut self);

    /// Clear the console to a set background color, if supported.
    fn cls_bg(&mut self, background: RGBA);

    /// Print a string at the specified x/y coordinate.
    fn print(&mut self, x: i32, y: i32, output: &str);

    /// Print a string in color at the specified x/y coordinate, with specified foreground and background.
    fn print_color(&mut self, x: i32, y: i32, fg: RGBA, bg: RGBA, output: &str);

    /// Print a colorized string with the color encoding defined inline.
    /// For example: printer(1, 1, "#[blue]This blue text contains a #[pink]pink#[] word")
    /// You can get the same effect with a TextBlock, but this can be easier.
    /// Thanks to doryen_rs for the idea.
    fn printer(&mut self, x: i32, y: i32, output: &str, align: TextAlign, background: Option<RGBA>);

    /// Sets a single cell to a color/glyph combination.
    fn set(&mut self, x: i32, y: i32, fg: RGBA, bg: RGBA, glyph: FontCharType);

    /// Sets a single cell's background color.
    fn set_bg(&mut self, x: i32, y: i32, bg: RGBA);

    /// Draws a box, starting at x/y with the extents width/height using CP437 line characters
    fn draw_box(&mut self, x: i32, y: i32, width: i32, height: i32, fg: RGBA, bg: RGBA);

    /// Draws a box, starting at x/y with the extents width/height using CP437 line characters,
    /// without filling in the middle
    fn draw_hollow_box(&mut self, x: i32, y: i32, width: i32, height: i32, fg: RGBA, bg: RGBA);

    /// Draws a box, starting at x/y with the extents width/height using CP437 double line characters
    fn draw_box_double(&mut self, x: i32, y: i32, width: i32, height: i32, fg: RGBA, bg: RGBA);

    /// Draws a box, starting at x/y with the extents width/height using CP437 double line characters,
    /// without filling in the middle
    fn draw_hollow_box_double(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        fg: RGBA,
        bg: RGBA,
    );

    /// Fills a rectangle-defined region with a given glyph
    fn fill_region(&mut self, target: Rect, glyph: FontCharType, fg: RGBA, bg: RGBA);

    /// Draws a horizontal progress bar.
    #[allow(clippy::too_many_arguments)]
    fn draw_bar_horizontal(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        n: i32,
        max: i32,
        fg: RGBA,
        bg: RGBA,
    );

    /// Draws a vertical progress bar.
    #[allow(clippy::too_many_arguments)]
    fn draw_bar_vertical(
        &mut self,
        x: i32,
        y: i32,
        height: i32,
        n: i32,
        max: i32,
        fg: RGBA,
        bg: RGBA,
    );

    /// Prints text, centered to the whole console width, at vertical location y.
    fn print_centered(&mut self, y: i32, text: &str);

    /// Prints text in color, centered to the whole console width, at vertical location y.
    fn print_color_centered(&mut self, y: i32, fg: RGBA, bg: RGBA, text: &str);

    /// Prints text, centered on an arbitrary point
    fn print_centered_at(&mut self, x: i32, y: i32, text: &str);

    /// Prints colored text, centered on an arbitrary point
    fn print_color_centered_at(&mut self, x: i32, y: i32, fg: RGBA, bg: RGBA, text: &str);

    /// Prints text right-aligned
    fn print_right(&mut self, x: i32, y: i32, text: &str);

    /// Prints colored text right-aligned
    fn print_color_right(&mut self, x: i32, y: i32, fg: RGBA, bg: RGBA, text: &str);

    /// Serializes the console layer to an XpFile
    fn to_xp_layer(&self) -> XpLayer;

    /// Specify a global offset (by character count, so 0.5 is half a character). Useful for
    /// drawing walls between tiles.
    fn set_offset(&mut self, x: f32, y: f32);

    /// Specify a scale and center of the console.
    /// A scale above 1.0 will make the text larger.
    /// The center of the scale is at character position (center_x, center_y).
    fn set_scale(&mut self, scale: f32, center_x: i32, center_y: i32);

    /// Get the scale & center of the console.
    /// Returns (scale, center_x, center_y).
    fn get_scale(&self) -> (f32, i32, i32);

    /// Produces the implementor as an Any that can be matched to determine type and access
    /// natively.
    fn as_any(&self) -> &dyn Any;

    /// Produces the implementor as an Any that can be matched to determine type and access
    /// natively.
    fn as_any_mut(&mut self) -> &mut dyn Any;

    /// Permits the creation of an arbitrary clipping rectangle. It's a really good idea
    /// to make sure that this rectangle is entirely valid.
    fn set_clipping(&mut self, clipping: Option<Rect>);

    /// Returns the current arbitrary clipping rectangle, None if there isn't one.
    fn get_clipping(&self) -> Option<Rect>;

    /// Returns true if an x/y coordinate is within the console bounds
    fn in_bounds(&self, x: i32, y: i32) -> bool {
        let bounds = self.get_char_size();
        if let Some(clip) = self.get_clipping() {
            clip.point_in_rect(Point::new(x, y))
                && x >= 0
                && x < bounds.0 as i32
                && y >= 0
                && y < bounds.1 as i32
        } else {
            x >= 0 && x < bounds.0 as i32 && y >= 0 && y < bounds.1 as i32
        }
    }

    /// Try to use a coordinate: return Some(the coordinate) if it is valid,
    /// None if it isn't.
    fn try_at(&self, x: i32, y: i32) -> Option<usize> {
        if self.in_bounds(x, y) {
            Some(self.at(x, y))
        } else {
            None
        }
    }

    /// Sets ALL tiles foreground alpha (only tiles that exist, in sparse consoles).
    fn set_all_fg_alpha(&mut self, alpha: f32);

    /// Sets ALL tiles background alpha (only tiles that exist, in sparse consoles).
    fn set_all_bg_alpha(&mut self, alpha: f32);

    /// Sets ALL tiles foreground alpha (only tiles that exist, in sparse consoles).
    fn set_all_alpha(&mut self, fg: f32, bg: f32);

    /// Sets the character translation mode
    fn set_translation_mode(&mut self, mode: CharacterTranslationMode);

    /// Sets the character size of the layer. Be really careful with this.
    fn set_char_size(&mut self, width: u32, height: u32);

    // Clears the dirty bit
    fn clear_dirty(&mut self);
}

pub fn log<S: ToString>(message: S) {
    crate::hal::log(&message.to_string());
}
