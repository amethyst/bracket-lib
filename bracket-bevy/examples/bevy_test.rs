use bevy::prelude::*;
use bracket_bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(BTermBuilder::simple_80x50())
        .add_system(tick)
        .run();
}

fn tick(ctx: Res<BracketContext>) {
    ctx.set_active_console(0);
    ctx.cls();
    ctx.print(1, 1, "Hello Bracket-Bevy World ☻");
    ctx.print_color(1, 2, "Now in color!", GREEN, NAVY);

    ctx.print_color(
        1,
        4,
        format!("Frames per Second: {}", ctx.fps as u32),
        YELLOW,
        BLACK,
    );
}
