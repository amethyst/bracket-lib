mod sparse_no_background;
mod sparse_with_background;
use super::SparseConsole;
use bevy::{
    prelude::{Assets, Commands, Handle, Mesh},
    sprite::ColorMaterial,
};
pub(crate) use sparse_no_background::*;
pub(crate) use sparse_with_background::*;

pub(crate) trait SparseConsoleBackend: Sync + Send {
    fn update_mesh(&self, front_end: &SparseConsole, meshes: &mut Assets<Mesh>);
    fn spawn(&self, commands: &mut Commands, material: Handle<ColorMaterial>, idx: usize);
}
