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
    let mut draw_batch = ctx.new_draw_batch();
    draw_batch.print_color_with_z(
        Point::new(10, 10),
        "This is at always on top",
        ColorPair::new(YELLOW, BLUE),
        1000,
    );
    for y in 0..50 {
        for x in 0..80 {
            draw_batch.set(
                Point::new(x, y),
                ColorPair::new(DARKGRAY, BLACK),
                to_cp437('#'),
            );
        }
    }
    ctx.submit_batch(0, draw_batch);
}
