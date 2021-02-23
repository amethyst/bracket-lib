// This example demonstrates error returning.
//////////////////////////////////////////////////////////////

use bracket_terminal::prelude::*;

bracket_terminal::add_wasm_support!();

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.print(1, 1, "Hello Bracket World");
    }
}

fn build_context() -> BResult<BTerm> {
    BTermBuilder::simple80x50()
        .with_title("Hello Minimal Bracket World")
        .build()
}

fn main() -> BError {
    let context = build_context()?;
    let gs: State = State {};

    main_loop(context, gs)
}
