mod simple_with_background;
use bevy::{
    prelude::{Assets, Commands, Handle, Mesh},
    sprite::ColorMaterial,
};
pub(crate) use simple_with_background::*;

use super::SimpleConsole;

pub(crate) trait SimpleConsoleBackend: Sync + Send {
    fn update_mesh(&self, front_end: &SimpleConsole, meshes: &mut Assets<Mesh>);
    fn spawn(&self, commands: &mut Commands, material: Handle<ColorMaterial>, idx: usize);
}
