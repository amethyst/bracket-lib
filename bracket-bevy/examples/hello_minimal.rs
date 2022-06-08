use bevy::prelude::*;
use bracket_bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(BTermBuilder::simple_80x50())
        .add_system(tick)
        .run();
}

fn tick(mut ctx: ResMut<BracketContext>) {
    ctx.set_layer(0);
    ctx.cls();
    ctx.print(1, 1, "Hello Bracket-Bevy World ☻");
}
