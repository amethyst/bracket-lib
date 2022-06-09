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
    fn new_mesh(&self, front_end: &SparseConsole, meshes: &mut Assets<Mesh>) -> Handle<Mesh>;
    fn spawn(&self, commands: &mut Commands, material: Handle<ColorMaterial>, idx: usize);
    fn get_pixel_size(&self) -> (f32, f32);
}
