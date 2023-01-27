use bevy::prelude::*;
use bracket_bevy::prelude::*;

fn main() {
    let bterm = BTermBuilder::empty()
        .with_font("terminal8x8.png", 16, 16, (8.0, 8.0))
        .with_font("vga8x16.png", 16, 16, (8.0, 16.0))
        .with_simple_console(0, 80, 50)
        .with_background(true)
        .with_sparse_console(1, 80, 25)
        .with_background(true);

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bterm)
        .insert_resource(Bouncer(0))
        .add_system(tick)
        .run();
}

#[derive(Resource)]
struct Bouncer(i32);

fn tick(ctx: Res<BracketContext>, mut bouncer: ResMut<Bouncer>) {
    ctx.set_active_console(0);
    ctx.cls();
    ctx.print(1, 1, "Hello Bracket-Bevy World ☻");
    ctx.print_color(1, 2, "Now in color!", GREEN, NAVY);

    ctx.set_active_console(1);
    ctx.cls();
    ctx.print_color(
        1,
        bouncer.0,
        format!(
            "Frames per Second: {}, {} ms per frame",
            ctx.fps, ctx.frame_time_ms
        ),
        RED,
        WHITE,
    );
    bouncer.0 += 1;
    bouncer.0 %= 25;
}
