use bevy::prelude::{Assets, Color, Mesh};
mod simple_console;
pub(crate) use simple_console::*;
mod update_system;
use crate::BracketContext;
pub(crate) use update_system::*;
mod sparse_console;
pub(crate) use sparse_console::*;
pub(crate) mod common_draw;

pub(crate) trait ConsoleFrontEnd: Sync + Send {
    fn cls(&mut self);
    fn cls_bg(&mut self, color: Color);
    fn print(&mut self, x: usize, y: usize, text: &str);
    fn print_color(&mut self, x: usize, y: usize, text: &str, foreground: Color, background: Color);
    fn print_centered(&mut self, y: usize, text: &str);
    fn set(&mut self, x: usize, y: usize, fg: Color, bg: Color, glyph: u16);
    fn draw_box(&mut self, x: usize, y: usize, width: usize, height: usize, fg: Color, bg: Color);
    fn draw_hollow_box(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        fg: Color,
        bg: Color,
    );
    fn draw_box_double(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        fg: Color,
        bg: Color,
    );
    fn draw_hollow_box_double(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        fg: Color,
        bg: Color,
    );

    fn update_mesh(&mut self, ctx: &BracketContext, meshes: &mut Assets<Mesh>);
}
