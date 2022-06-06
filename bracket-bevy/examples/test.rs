use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bracket_bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(BTermBuilder::simple_80x50())
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_system(tick)
        .run();
}

fn tick(mut ctx: ResMut<BracketContext>, diagnostics: Res<Diagnostics>) {
    ctx.set_layer(0);
    ctx.cls();
    ctx.print(1, 1, "Hello Bracket-Bevy World ☻");
    ctx.print_color(1, 2, "Now in color!", Color::GREEN, Color::NAVY);

    let mut fps = 0.0;
    if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(fps_avg) = fps_diagnostic.average() {
            fps = fps_avg.round();
        }
    }
    ctx.print_color(
        1,
        4,
        format!("Frames per Second: {fps}"),
        Color::YELLOW,
        Color::BLACK,
    );
}
