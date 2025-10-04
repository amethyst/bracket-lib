use bevy::prelude::*;
use bracket_bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BTermBuilder::simple_80x50())
        .add_systems(Update, tick)
        .run();
}

fn tick(ctx: Res<BracketContext>) {
    ctx.set_active_console(0);
    ctx.cls();
    ctx.print(1, 1, "Hello Bracket-Bevy World ☻");
}
