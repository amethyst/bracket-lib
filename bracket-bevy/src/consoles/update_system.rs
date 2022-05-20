use bevy::prelude::{ResMut, Assets, Mesh};
use crate::BracketContext;



pub fn update_consoles(
    ctx: ResMut<BracketContext>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for terminal in ctx.terminals.iter() {
        if let Some(mesh_handle) = &terminal.mesh_handle {
            if let Some(mesh) = meshes.get_mut(mesh_handle.clone()) {
                mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, terminal.build_uvs(
                    ctx.fonts[terminal.font_index].chars_per_row,
                    ctx.fonts[terminal.font_index].n_rows,
                ));
                mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, terminal.build_colors());
            }
        }
    }
}
