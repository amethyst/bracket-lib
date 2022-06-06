use bevy::prelude::{ResMut, Res, Assets, Mesh};
use crate::BracketContext;

pub fn update_consoles(
    ctx: Res<BracketContext>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for terminal in ctx.terminals.lock().iter() {
        terminal.update_mesh(&ctx, &mut meshes);
    }
}
