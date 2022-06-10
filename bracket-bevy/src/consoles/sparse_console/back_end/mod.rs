mod sparse_no_background;
mod sparse_with_background;
use crate::consoles::ScreenScaler;

use super::SparseConsole;
use bevy::{
    prelude::{Assets, Commands, Handle, Mesh},
    sprite::ColorMaterial,
};
pub(crate) use sparse_no_background::*;
pub(crate) use sparse_with_background::*;

pub(crate) trait SparseConsoleBackend: Sync + Send {
    fn new_mesh(
        &self,
        front_end: &SparseConsole,
        meshes: &mut Assets<Mesh>,
        scaler: &ScreenScaler,
    ) -> Handle<Mesh>;
    fn spawn(&self, commands: &mut Commands, material: Handle<ColorMaterial>, idx: usize);
    fn get_pixel_size(&self) -> (f32, f32);
    fn resize(&mut self, available_size: &(f32, f32)) -> (usize, usize);
}
