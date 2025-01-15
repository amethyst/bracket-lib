use crate::{BracketCamera, BracketContext, TerminalScalingMode};
use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    render::camera::RenderTarget,
    window::{PrimaryWindow, WindowRef, WindowResized},
};

use super::{BracketMesh, ScreenScaler};

pub(crate) fn update_consoles(
    mut ctx: ResMut<BracketContext>,
    mut meshes: ResMut<Assets<Mesh>>,
    find_mesh: Query<(&BracketMesh, &Mesh2d)>,
    scaler: Res<ScreenScaler>,
) {
    let mut new_meshes: Vec<(Mesh2d, Mesh2d, bool)> = Vec::new();
    {
        let mut terms = ctx.terminals.lock();
        for (id, handle) in find_mesh.iter() {
            let terminal_id = id.0;
            let new_mesh = terms[terminal_id].new_mesh(&ctx, &mut meshes, &scaler);
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
    mut update_mesh: Query<&mut Mesh2d, With<BracketMesh>>,
) {
    for ev in ev_asset.read() {
        if let AssetEvent::Added { id } = ev {
            for (old, new, done) in ctx.mesh_replacement.iter_mut() {
                if *id == new.0.id() {
                    for mut m in update_mesh.iter_mut() {
                        if old.0.id() == m.0.id() {
                            *m = new.clone();
                        }
                    }
                    *done = true;
                }
            }
        }
    }

    for (old, _, _) in ctx.mesh_replacement.iter().filter(|(_, _, done)| *done) {
        meshes.remove(old.0.id().clone());
    }
    ctx.mesh_replacement.retain(|(_, _, done)| !done);
}

pub(crate) fn update_timing(mut ctx: ResMut<BracketContext>, diagnostics: Res<DiagnosticsStore>) {
    if let Some(fps_diagnostic) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(fps_avg) = fps_diagnostic.measurement() {
            ctx.fps = fps_avg.value.round();
        }
    }

    if let Some(frame_time) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME) {
        if let Some(frame_time_avg) = frame_time.measurement() {
            ctx.frame_time_ms = (frame_time_avg.value * 1000.0).round();
        }
    }
}

pub(crate) fn window_resize(
    mut context: ResMut<BracketContext>,
    mut reader: EventReader<WindowResized>,
    mut scaler: ResMut<ScreenScaler>,
) {
    for e in reader.read() {
        scaler.set_screen_size(e.width, e.height);
        if let TerminalScalingMode::ResizeTerminals = context.scaling_mode {
            context.resize_terminals(&scaler);
        }
        scaler.recalculate(context.get_pixel_size(), context.largest_font());
    }
}

pub(crate) fn apply_all_batches(mut context: ResMut<BracketContext>) {
    context.render_all_batches();
}

pub(crate) fn update_mouse_position(
    // wnds: Res<Windows>,
    wnd_primary: Query<&Window, With<PrimaryWindow>>,
    wnd: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform), With<BracketCamera>>,
    mut context: ResMut<BracketContext>,
    scaler: Res<ScreenScaler>,
) {
    // Modified from: https://bevy-cheatbook.github.io/cookbook/cursor2world.html
    for (camera, camera_transform) in &q_camera {
        // get the window the camera is rendering to
        let window = match camera.target {
            // the camera is rendering to the primary window
            RenderTarget::Window(WindowRef::Primary) => wnd_primary.single(),
            // the camera is rendering to some other window
            RenderTarget::Window(WindowRef::Entity(e_window)) => wnd.get(e_window).unwrap(),
            // the camera is rendering to something else (like a texture), not a window
            _ => {
                // skip this camera
                continue;
            }
        };

        // check if the cursor is inside the window and get its position
        // then, ask bevy to convert into world coordinates, and truncate to discard Z
        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
            .map(|ray| ray.origin.truncate())
        {
            // worldcursor.0 = world_position;
            context.set_mouse_pixel_position((world_position.x, world_position.y), &scaler);
        }
    }
}
