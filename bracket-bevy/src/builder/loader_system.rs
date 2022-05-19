use bevy::{prelude::{Res, Commands, AssetServer, ResMut, Assets, Mesh, OrthographicCameraBundle, Transform, default}, sprite::{ColorMaterial, MaterialMesh2dBundle}};
use crate::{BTermBuilder, BracketContext, fonts::FontStore, TerminalLayer, SimpleConsole, SimpleConsoleMarker};

pub(crate) fn load_terminals(
    context: Res<BTermBuilder>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if context.with_ortho_camera {
        commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    }

    // Setup the new context
    let mut new_context = BracketContext::new();

    // Load the fonts
    for font in context.fonts.iter() {
        let texture_handle = asset_server.load(&font.filename);
        let material_handle = materials.add(ColorMaterial::from(texture_handle.clone()));
        new_context.fonts.push(FontStore::new(texture_handle, material_handle, font.chars_per_row, font.n_rows));
    }

    // Setup the consoles
    for (idx, terminal) in context.layers.iter().enumerate() {
        match terminal {
            TerminalLayer::Simple { font_index, width, height } => {
                let mut console = SimpleConsole::new(*font_index, *width, *height);
                let mesh = console.build_mesh(new_context.fonts[console.font_index].chars_per_row, new_context.fonts[console.font_index].n_rows);
                let mesh_handle = meshes.add(mesh);
                console.mesh_handle=Some(mesh_handle.clone());

                // Test code
                commands.spawn_bundle(MaterialMesh2dBundle {
                    mesh: mesh_handle.into(),
                    transform: Transform::default(),
                    material: new_context.fonts[*font_index].material_handle.clone(),
                    ..default()
                })
                .insert(SimpleConsoleMarker(idx));

                new_context.terminals.push(console);
            }
        }
    }

    // Clean up after the building process
    commands.remove_resource::<BTermBuilder>();
    commands.insert_resource(new_context);
}