use crate::{
    consoles::SparseConsole, fonts::FontStore, BTermBuilder, BracketContext, SimpleConsole,
    TerminalLayer,
};
use bevy::{
    core_pipeline::core_2d::Camera2d,
    prelude::{AssetServer, Assets, Commands, Component, Mesh, Msaa, Res, ResMut, UntypedHandle},
    sprite::ColorMaterial,
};

use super::image_fixer::ImagesToLoad;

#[derive(Component)]
pub struct BracketCamera;

pub(crate) fn load_terminals(
    context: Res<BTermBuilder>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if context.with_ortho_camera {
        commands
            .spawn(Camera2d::default())
            .insert(BracketCamera)
            .insert(Msaa::Off);
    }

    // Setup the new context
    let mut new_context = BracketContext::new(context.palette.clone());
    new_context.scaling_mode = context.scaling_mode;

    // Load the fonts
    let mut texture_handles = Vec::<UntypedHandle>::new();
    for font in context.fonts.iter() {
        let texture_handle = asset_server.load(&font.filename);
        let material_handle = materials.add(ColorMaterial::from(texture_handle.clone()));
        texture_handles.push(texture_handle.clone().untyped());
        new_context.fonts.push(FontStore::new(
            texture_handle,
            material_handle,
            font.chars_per_row,
            font.n_rows,
            font.font_height_pixels,
        ));
    }
    commands.insert_resource(ImagesToLoad(texture_handles));

    // Setup the consoles
    for (idx, terminal) in context.layers.iter().enumerate() {
        match terminal {
            TerminalLayer::Simple {
                font_index,
                width,
                height,
                features,
            } => {
                let mut console = SimpleConsole::new(*font_index, *width, *height);
                console.initialize(&new_context.fonts, &mut meshes, features);
                console.spawn(
                    &mut commands,
                    new_context.fonts[*font_index].material_handle.clone(),
                    idx,
                );
                new_context.terminals.lock().push(Box::new(console));
            }
            TerminalLayer::Sparse {
                font_index,
                width,
                height,
                features,
            } => {
                let mut console = SparseConsole::new(*font_index, *width, *height);
                console.initialize(&new_context.fonts, &mut meshes, features);
                console.spawn(
                    &mut commands,
                    new_context.fonts[*font_index].material_handle.clone(),
                    idx,
                );
                new_context.terminals.lock().push(Box::new(console));
            }
        }
    }

    // Clean up after the building process
    commands.remove_resource::<BTermBuilder>();
    commands.insert_resource(new_context);
}
