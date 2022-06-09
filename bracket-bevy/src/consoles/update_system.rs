use crate::BracketContext;
use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::{Assets, Mesh, Res, ResMut},
};

pub(crate) fn update_consoles(ctx: Res<BracketContext>, mut meshes: ResMut<Assets<Mesh>>) {
    for terminal in ctx.terminals.lock().iter_mut() {
        terminal.update_mesh(&ctx, &mut meshes);
    }
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
