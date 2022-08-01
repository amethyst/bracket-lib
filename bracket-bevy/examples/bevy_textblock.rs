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
    draw_batch.cls();
    let mut block = TextBlock::new(0, 0, 80, 25);

    let mut buf = TextBuilder::empty();
    buf.ln()
            .fg(RGB::named(YELLOW))
            .bg(RGB::named(BLUE))
            .centered("Hello World")
            .fg(RGB::named(WHITE))
            .bg(RGB::named(BLACK))
            .ln()
            .ln()
            .line_wrap("The quick brown fox jumped over the lazy dog, and just kept on running in an attempt to exceed the console width.")
            .ln()
            .ln()
            .line_wrap("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.")
            .ln().ln()
            .fg(RGB::named(CYAN))
            .append("FPS: ")
            .fg(RGB::named(MAGENTA))
            .append(&format!("{}", ctx.fps))
            .reset();

    block.print(&buf).expect("Text was too long");

    block.render_to_draw_batch(&mut draw_batch);
    ctx.submit_batch(0, draw_batch);
}
