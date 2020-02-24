// This is the canonical "Hello World" example for Bracket Terminal.
// It is crammed into one file, and kept as short as possible
//////////////////////////////////////////////////////////////

// We're using BTerm (the main context) and GameState (a trait defining what our callback
// looks like), so we need to use that, too.`
use bracket_terminal::prelude::*;

// We're utilizing functionality from BTerm, so we need to tell it to use the crate.
bracket_terminal::add_wasm_support!();

// This is the structure that will store our game state, typically a state machine pointing to
// other structures. This demo is realy simple, so we'll just put the minimum to make it work
// in here.
struct State {}

// We have to implement the "trait" GameState for our state object. This gives it a callback
// point for the main loop.
impl GameState for State {
    // This is called every time the screen refreshes (a "tick") by BTerm's main loop. Since GUIs
    // require that you process events every turn - rather than just sequentially like a good old text
    // console, you have to run your game as something of a state machine. This will be fleshed out in
    // later tutorials. For now, it just shows you the frame rate and says "Hello World".
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.print(1, 1, "Hello Bracket World");
    }
}

// Every program needs a main() function!
fn main() -> BError {
    // BTerm's Builder interface offers a number of helpers to get you up and running quickly.
    // Here, we are using the `simple80x50()` helper, which builds an 80-wide by 50-tall console,
    // with the baked-in 8x8 terminal font.
    let context = BTermBuilder::simple80x50()
        .with_title("Hello Minimal Bracket World")
        .build()?;

    // Now we create an empty state object.
    let gs: State = State {};

    // Call into BTerm to run the main loop. This handles rendering, and calls back into State's tick
    // function every cycle. The box is needed to work around lifetime handling.
    main_loop(context, gs)
}
