use bevy::prelude::{Assets, Color, Mesh};

mod simple_console;
pub(crate) use simple_console::*;
mod update_system;
pub(crate) use update_system::*;

use crate::BracketContext;

pub(crate) trait ConsoleFrontEnd: Sync + Send {
    fn cls(&mut self);
    fn print(&mut self, x: usize, y: usize, text: &str);
    fn print_color(&mut self, x: usize, y: usize, text: &str, foreground: Color, background: Color);

    fn update_mesh(&mut self, ctx: &BracketContext, meshes: &mut Assets<Mesh>);
}
