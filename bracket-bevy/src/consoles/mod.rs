use bevy::prelude::{Assets, Handle, Mesh};
mod simple_console;
use bracket_geometry::prelude::{Point, Rect};
pub(crate) use simple_console::*;
mod update_system;
use crate::{BracketContext, FontCharType};
pub(crate) use update_system::*;
mod sparse_console;
pub(crate) use sparse_console::*;
pub(crate) mod common_draw;
use bracket_color::prelude::RGBA;
mod text_spans;
pub(crate) use text_spans::*;
mod scaler;
pub(crate) use scaler::*;
mod virtual_console;
pub use virtual_console::*;
mod draw_batch;
pub use draw_batch::*;

pub(crate) trait ConsoleFrontEnd: Sync + Send {
    fn get_char_size(&self) -> (usize, usize);
    fn get_pixel_size(&self) -> (f32, f32);
    fn at(&self, x: i32, y: i32) -> usize;
    fn get_clipping(&self) -> Option<Rect>;
    fn set_clipping(&mut self, clipping: Option<Rect>);
    fn cls(&mut self);
    fn cls_bg(&mut self, color: RGBA);
    fn print(&mut self, x: i32, y: i32, text: &str);
    fn print_color(&mut self, x: i32, y: i32, text: &str, foreground: RGBA, background: RGBA);
    fn print_centered(&mut self, y: i32, text: &str);
    fn print_color_centered(&mut self, y: i32, fg: RGBA, bg: RGBA, text: &str);
    fn print_centered_at(&mut self, x: i32, y: i32, text: &str);
    fn print_color_centered_at(&mut self, x: i32, y: i32, fg: RGBA, bg: RGBA, text: &str);
    fn print_right(&mut self, x: i32, y: i32, text: &str);
    fn print_color_right(&mut self, x: i32, y: i32, fg: RGBA, bg: RGBA, text: &str);
    fn set(&mut self, x: i32, y: i32, fg: RGBA, bg: RGBA, glyph: FontCharType);
    fn set_bg(&mut self, x: i32, y: i32, bg: RGBA);
    fn draw_box(&mut self, x: i32, y: i32, width: i32, height: i32, fg: RGBA, bg: RGBA);
    fn draw_hollow_box(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        fg: RGBA,
        bg: RGBA,
    );

    fn draw_box_double(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        fg: RGBA,
        bg: RGBA,
    );

    fn draw_hollow_box_double(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        fg: RGBA,
        bg: RGBA,
    );

    fn fill_region(&mut self, target: Rect, glyph: FontCharType, fg: RGBA, bg: RGBA);

    fn printer(
        &mut self,
        context: &BracketContext,
        x: i32,
        y: i32,
        output: &str,
        align: TextAlign,
        background: Option<RGBA>,
    );

    fn in_bounds(&self, x: i32, y: i32) -> bool {
        let bounds = self.get_char_size();
        let bounds = (bounds.0 as i32, bounds.1 as i32);
        if let Some(clip) = self.get_clipping() {
            clip.point_in_rect(Point::new(x, y)) && x < bounds.0 && y < bounds.1
        } else {
            x < bounds.0 && y < bounds.1 && x > 0 && y > 0
        }
    }

    fn try_at(&self, x: i32, y: i32) -> Option<usize> {
        if self.in_bounds(x, y) {
            Some(self.at(x, y))
        } else {
            None
        }
    }

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

    /// Sets ALL tiles foreground alpha (only tiles that exist, in sparse consoles).
    fn set_all_fg_alpha(&mut self, alpha: f32);

    /// Sets ALL tiles background alpha (only tiles that exist, in sparse consoles).
    fn set_all_bg_alpha(&mut self, alpha: f32);

    /// Sets ALL tiles foreground alpha (only tiles that exist, in sparse consoles).
    fn set_all_alpha(&mut self, fg: f32, bg: f32);

    fn new_mesh(
        &mut self,
        ctx: &BracketContext,
        meshes: &mut Assets<Mesh>,
        scaler: &ScreenScaler,
    ) -> Option<Handle<Mesh>>;

    fn resize(&mut self, available_size: &(f32, f32));

    fn get_mouse_position_for_current_layer(&self) -> Point;
    fn set_mouse_position(&mut self, position: (f32, f32), scaler: &ScreenScaler);
    fn get_font_index(&self) -> usize;
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}
