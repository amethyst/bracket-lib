bracket_terminal::add_wasm_support!();
use bracket_terminal::prelude::*;

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
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

        block.print(&buf);

        block.render(&mut ctx.consoles[0].console);
    }
}

fn main() -> BError {
    let gs: State = State {};

    let context = BTermBuilder::simple80x50()
        .with_title("Bracket Terminal Example - Text Blocks")
        .build()?;
    main_loop(context, gs)
}
