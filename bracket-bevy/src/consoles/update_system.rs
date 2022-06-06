use crate::BracketContext;
use bevy::prelude::{Assets, Mesh, Res, ResMut};

pub fn update_consoles(ctx: Res<BracketContext>, mut meshes: ResMut<Assets<Mesh>>) {
    for terminal in ctx.terminals.lock().iter() {
        terminal.update_mesh(&ctx, &mut meshes);
    }
}
