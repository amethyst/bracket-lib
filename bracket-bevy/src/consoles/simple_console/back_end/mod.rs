mod simple_no_background;
mod simple_with_background;
use super::{SimpleConsole, TerminalGlyph};
use bevy::{
    prelude::{Assets, Commands, Handle, Mesh},
    sprite::ColorMaterial,
};
pub(crate) use simple_no_background::*;
pub(crate) use simple_with_background::*;

pub(crate) trait SimpleConsoleBackend: Sync + Send {
    fn update_mesh(&self, front_end: &SimpleConsole, meshes: &mut Assets<Mesh>);
    fn spawn(&self, commands: &mut Commands, material: Handle<ColorMaterial>, idx: usize);
    fn clear_dirty(&mut self);
    fn update_dirty(&mut self, terminals: &[TerminalGlyph]);
}
