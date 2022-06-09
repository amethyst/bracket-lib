use crate::{BracketContext, TerminalScalingMode};
use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    ecs::event::Events,
    prelude::*,
    sprite::Mesh2dHandle,
    window::WindowResized,
};

use super::BracketMesh;

pub(crate) fn update_consoles(
    mut ctx: ResMut<BracketContext>,
    mut meshes: ResMut<Assets<Mesh>>,
    find_mesh: Query<(&BracketMesh, &Mesh2dHandle)>,
) {
    let mut new_meshes: Vec<(Mesh2dHandle, Mesh2dHandle, bool)> = Vec::new();
    {
        let mut terms = ctx.terminals.lock();
        for (id, handle) in find_mesh.iter() {
            let terminal_id = id.0;
            let new_mesh = terms[terminal_id].new_mesh(&ctx, &mut meshes);
            if let Some(new_mesh) = new_mesh {
                let old_mesh = handle.clone();
                new_meshes.push((old_mesh, new_mesh.into(), false));
            }
        }
    }

    new_meshes
        .drain(0..)
        .for_each(|m| ctx.mesh_replacement.push(m));
}

pub(crate) fn replace_meshes(
    mut ctx: ResMut<BracketContext>,
    mut ev_asset: EventReader<AssetEvent<Mesh>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut update_mesh: Query<&mut Mesh2dHandle, With<BracketMesh>>,
) {
    for ev in ev_asset.iter() {
        if let AssetEvent::Created { handle } = ev {
            for (old, new, done) in ctx.mesh_replacement.iter_mut() {
                if handle.id == new.0.id {
                    update_mesh.for_each_mut(|mut m| {
                        if old.0.id == m.0.id {
                            *m = new.clone();
                        }
                    });
                    *done = true;
                }
            }
        }
    }

    for (old, _, _) in ctx.mesh_replacement.iter().filter(|(_, _, done)| *done) {
        meshes.remove(old.0.clone());
    }
    ctx.mesh_replacement.retain(|(_, _, done)| !done);
}

pub(crate) fn update_timing(mut ctx: ResMut<BracketContext>, diagnostics: Res<Diagnostics>) {
    if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(fps_avg) = fps_diagnostic.measurement() {
            ctx.fps = fps_avg.value.round();
        }
    }

    if let Some(frame_time) = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME) {
        if let Some(frame_time_avg) = frame_time.measurement() {
            ctx.frame_time_ms = (frame_time_avg.value * 1000.0).round();
        }
    }
}

pub(crate) fn window_resize(
    context: Res<BracketContext>,
    resize_event: Res<Events<WindowResized>>,
    mut query: Query<&mut Transform, With<BracketMesh>>,
) {
    let mut reader = resize_event.get_reader();
    for e in reader.iter(&resize_event) {
        match context.scaling_mode {
            TerminalScalingMode::Stretch => {
                let terminal_size_native = context.get_pixel_size();
                let x = e.width / terminal_size_native.0;
                let y = e.height / terminal_size_native.1;
                query.for_each_mut(|mut trans| {
                    trans.scale.x = x;
                    trans.scale.y = y;
                });
            }
        }
    }
}
