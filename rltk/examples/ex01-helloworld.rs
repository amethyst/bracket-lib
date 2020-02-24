#![warn(clippy::pedantic)]
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
    y: i32,
    going_down: bool,
}

// We have to implement the "trait" GameState for our state object. This gives it a callback
// point for the main loop.
impl GameState for State {
    // This is called every time the screen refreshes (a "tick") by RLTK's main loop. Since GUIs
    // require that you process events every turn - rather than just sequentially like a good old text
    // console, you have to run your game as something of a state machine. This will be fleshed out in
    // later tutorials. For now, it just shows you the frame rate and says "Hello World".
    fn tick(&mut self, ctx: &mut Rltk) {
        let col1 = RGB::named(rltk::CYAN);
        let col2 = RGB::named(rltk::YELLOW);
        let percent: f32 = self.y as f32 / 50.0;
        let fg = col1.lerp(col2, percent);

        ctx.cls();
        // Notice that unicode conversion is active, so we can cut/paste characters from
        // a CP437 reference such as http://dwarffortresswiki.org/index.php/Character_table
        ctx.print_color(
            1,
            self.y,
            fg,
            RGB::named(rltk::BLACK),
            "♫ ♪ Hello RLTK World ☺",
        );

        // Lets make the hello bounce up and down
        if self.going_down {
            self.y += 1;
            if self.y > 48 {
                self.going_down = false;
            }
        } else {
            self.y -= 1;
            if self.y < 2 {
                self.going_down = true;
            }
        }

        // We'll also show the frame rate, since we generally care about keeping that high.
        ctx.draw_box(
            39,
            0,
            20,
            3,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
        );
        ctx.print_color(
            40,
            1,
            RGB::named(rltk::YELLOW),
            RGB::named(rltk::BLACK),
            &format!("FPS: {}", ctx.fps),
        );
        ctx.print_color(
            40,
            2,
            RGB::named(rltk::CYAN),
            RGB::named(rltk::BLACK),
            &format!("Frame Time: {} ms", ctx.frame_time_ms),
        );
    }
}

// Every program needs a main() function!
fn main() -> RltkError {
    // RLTK's ConsoleBuilder interface offers a number of helpers to get you up and running quickly.
    // Here, we are using the `simple80x50()` helper, which builds an 80-wide by 50-tall console,
    // with the baked-in 8x8 terminal font.
    let context = RltkBuilder::simple80x50()
        .with_title("Hello RLTK World")
        .with_fps_cap(30.0)
        .build()?;

    // Now we create an empty state object.
    let gs: State = State {
        y: 1,
        going_down: true,
    };

    // Call into RLTK to run the main loop. This handles rendering, and calls back into State's tick
    // function every cycle. The box is needed to work around lifetime handling.
    rltk::main_loop(context, gs)
}
