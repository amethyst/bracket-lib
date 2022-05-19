use bevy::{
    prelude::*, sprite::{MaterialMesh2dBundle, Material2d},
};
use bracket_bevy::SimpleConsole;

/// This example shows how to manually render 2d items using "mid level render apis" with a custom pipeline for 2d meshes
/// It doesn't use the [`Material2d`] abstraction, but changes the vertex buffer to include vertex color
/// Check out the "mesh2d" example for simpler / higher level 2d meshes
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let console = SimpleConsole::new(80, 50);
    let mesh = console.build_mesh();
    let mesh_handle = meshes.add(mesh);
    let texture_handle = asset_server.load("terminal8x8.png");

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: mesh_handle.into(),
        transform: Transform::default(),
        material: materials.add(ColorMaterial::from(texture_handle)),
        ..default()
    });
}