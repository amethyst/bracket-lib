use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bracket_bevy::prelude::*;

fn main() {
    let bterm = BTermBuilder::simple_80x50().with_random_number_generator(true);

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bterm)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_system(tick)
        .run();
}

fn tick(mut ctx: ResMut<BracketContext>, rng: Res<RandomNumbers>, diagnostics: Res<Diagnostics>) {
    for y in 0..50 {
        for x in 0..80 {
            let val = rng.rand::<u64>();
            let back = Color::from([
                (val & 0xFF) as f32 / 255.0,
                ((val >> 8) & 0x5F) as f32 / 255.0,
                ((val >> 16) & 0x3F) as f32 / 255.0,
            ]);
            let fore = Color::from([
                ((val >> 16) & 0xFF) as f32 / 255.0,
                ((val >> 24) & 0xFF) as f32 / 255.0,
                ((val >> 32) & 0xFF) as f32 / 255.0,
            ]);
            let ascii = ((val >> 40) & 0xFF) as u16;
            ctx.set(x, y, fore, back, ascii);
        }
    }
    ctx.draw_box(30, 20, 20, 5, Color::WHITE, Color::BLACK);
    let mut fps = 0.0;
    if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(fps_avg) = fps_diagnostic.average() {
            fps = fps_avg.round();
        }
    }
    ctx.print_centered(22, &format!("{} fps", fps as u32));
}
