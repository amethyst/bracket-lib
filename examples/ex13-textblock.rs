extern crate rltk;

use rltk::{Console, GameState, Rltk, TextBlock, TextBuilder, RGB};

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        let mut block = TextBlock::new(0, 0, 80, 25);

        let mut buf = TextBuilder::empty();
        buf.ln()
            .fg(RGB::named(rltk::YELLOW))
            .centered("Hello World")
            .fg(RGB::named(rltk::WHITE))
            .ln()
            .ln()
            .line_wrap("The quick brown fox jumped over the lazy dog, and just kept on running in an attempt to exceed the console width.")
            .ln()
            .ln()
            .line_wrap("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.")
            .ln().ln()
            .fg(RGB::named(rltk::CYAN))
            .append("FPS: ")
            .fg(RGB::named(rltk::MAGENTA))
            .append(&format!("{}", ctx.fps))
            .reset();

        block.print(&buf);

        block.render(&mut ctx.consoles[0].console);
    }
}

fn main() {
    let gs: State = State {};

    let context = Rltk::init_simple8x16(80, 25, "Example 13 - Text Blocks", "resources");
    rltk::main_loop(context, gs);
}
