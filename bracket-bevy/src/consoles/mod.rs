use bevy::prelude::{Assets, Mesh};
mod simple_console;
pub(crate) use simple_console::*;
mod update_system;
use crate::BracketContext;
pub(crate) use update_system::*;
mod sparse_console;
pub(crate) use sparse_console::*;
pub(crate) mod common_draw;
mod point;
mod rect;
use bracket_color::prelude::RGBA;
pub use point::Point;
pub use rect::Rect;
mod text_spans;
pub(crate) use text_spans::*;

pub(crate) trait ConsoleFrontEnd: Sync + Send {
    fn get_char_size(&self) -> (usize, usize);
    fn at(&self, x: usize, y: usize) -> usize;
    fn get_clipping(&self) -> Option<Rect>;
    fn set_clipping(&mut self, clipping: Option<Rect>);
    fn cls(&mut self);
    fn cls_bg(&mut self, color: RGBA);
    fn print(&mut self, x: usize, y: usize, text: &str);
    fn print_color(&mut self, x: usize, y: usize, text: &str, foreground: RGBA, background: RGBA);
    fn print_centered(&mut self, y: usize, text: &str);
    fn set(&mut self, x: usize, y: usize, fg: RGBA, bg: RGBA, glyph: u16);
    fn draw_box(&mut self, x: usize, y: usize, width: usize, height: usize, fg: RGBA, bg: RGBA);
    fn draw_hollow_box(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        fg: RGBA,
        bg: RGBA,
    );

    fn draw_box_double(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        fg: RGBA,
        bg: RGBA,
    );

    fn draw_hollow_box_double(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        fg: RGBA,
        bg: RGBA,
    );

    fn fill_region(&mut self, target: Rect, glyph: u16, fg: RGBA, bg: RGBA);

    fn printer(
        &mut self,
        context: &BracketContext,
        x: usize,
        y: usize,
        output: &str,
        align: TextAlign,
        background: Option<RGBA>,
    );

    fn in_bounds(&self, x: usize, y: usize) -> bool {
        let bounds = self.get_char_size();
        if let Some(clip) = self.get_clipping() {
            clip.point_in_rect(Point::new(x, y)) && x < bounds.0 as usize && y < bounds.1 as usize
        } else {
            x < bounds.0 as usize && y < bounds.1 as usize
        }
    }

    fn try_at(&self, x: usize, y: usize) -> Option<usize> {
        if self.in_bounds(x, y) {
            Some(self.at(x, y))
        } else {
            None
        }
    }

    fn update_mesh(&mut self, ctx: &BracketContext, meshes: &mut Assets<Mesh>);
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}
