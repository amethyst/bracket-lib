// This is the canonical "Hello World" example for RLTK.
// It is crammed into one file, and kept as short as possible
//////////////////////////////////////////////////////////////

// We're utilizing functionality from RLTK, so we need to tell it to use the crate.
rltk::add_wasm_support!();

// We're using Rltk (the main context) and GameState (a trait defining what our callback
// looks like), so we need to use that, too.`
use rltk::prelude::*;

// This is the structure that will store our game state, typically a state machine pointing to
// other structures. This demo is realy simple, so we'll just put the minimum to make it work
// in here.
struct State {
    display_lines: Vec<String>,
}

// We have to implement the "trait" GameState for our state object. This gives it a callback
// point for the main loop.
impl GameState for State {
    // This is called every time the screen refreshes (a "tick") by RLTK's main loop. Since GUIs
    // require that you process events every turn - rather than just sequentially like a good old text
    // console, you have to run your game as something of a state machine. This will be fleshed out in
    // later tutorials. For now, it just shows you the frame rate and says "Hello World".
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        for (y, line) in self.display_lines.iter().enumerate() {
            ctx.print(0, y as i32, &line);
        }

        match ctx.key {
            None => {}
            Some(key) => {
                self.display_lines.push(format!(
                    "{} pressed, shift: {}, control: {}, alt: {}",
                    key as i32, ctx.shift, ctx.control, ctx.alt
                ));
                while self.display_lines.len() > 49 {
                    self.display_lines.remove(0);
                }
            }
        }
    }
}

// Every program needs a main() function!
fn main() -> RltkError {
    // RLTK provides a simple initializer for a simple 8x8 font window of a given number of
    // characters. Since that's all we need here, we'll use it!
    // We're specifying that we want an 80x50 console, with a title, and a relative path
    // to where it can find the font files and shader files.
    // These would normally be "resources" rather than "../../resources" - but to make it
    // work in the repo without duplicating, they are a relative path.
    let context = RltkBuilder::simple80x50()
        .with_title("RLTK Example 16 - Keyboard Input Visualizer")
        .build()?;

    // Now we create an empty state object.
    let gs: State = State {
        display_lines: vec!["Press keys and modifiers to see code combinations.".to_string()],
    };

    // Call into RLTK to run the main loop. This handles rendering, and calls back into State's tick
    // function every cycle. The box is needed to work around lifetime handling.
    rltk::main_loop(context, gs)
}
