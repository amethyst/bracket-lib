use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bracket_bevy::prelude::*;

fn main() {
    let bterm = BTermBuilder::empty()
        .with_font("terminal8x8.png", 16, 16, (8.0, 8.0))
        .with_font("vga8x16.png", 16, 16, (8.0, 16.0))
        .with_simple_console(0, 80, 50)
        .with_background(true)
        .with_sparse_console(1, 80, 25)
        .with_background(false);

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bterm)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .insert_resource(Bouncer(0))
        .add_system(tick)
        .run();
}

struct Bouncer(usize);

fn tick(
    mut ctx: ResMut<BracketContext>,
    diagnostics: Res<Diagnostics>,
    mut bouncer: ResMut<Bouncer>,
) {
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
    ctx.set_layer(1);
    ctx.cls();
    ctx.print_color(
        1,
        bouncer.0,
        format!("Frames per Second: {fps}"),
        Color::YELLOW,
        Color::BLACK,
    );
    bouncer.0 += 1;
    bouncer.0 %= 25;
}
