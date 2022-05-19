use bevy::{prelude::*, diagnostic::{FrameTimeDiagnosticsPlugin, Diagnostics}};
use bracket_bevy::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(BTermBuilder::simple_80x50())
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_system(tick)
        .run();
}

fn tick(
    mut ctx: ResMut<BracketContext>,
    diagnostics: Res<Diagnostics>,
) {
    ctx.set_layer(0);
    ctx.cls();
    ctx.print(1, 1, "Hello Bracket-Bevy World ☻");

    let mut fps = 0.0;
    if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(fps_avg) = fps_diagnostic.average() {
            fps = fps_avg.round();
        }
    }
    ctx.print(1, 3, format!("Frames per Second: {fps}"));
}