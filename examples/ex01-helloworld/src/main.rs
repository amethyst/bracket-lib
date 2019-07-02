// This is the canonical "Hello World" example for RLTK.
// It is crammed into one file, and kept as short as possible
//////////////////////////////////////////////////////////////

// We're utilizing functionality from RLTK, so we need to tell it to use the crate.
extern crate rltk;

// We're using Rltk (the main context) and GameState (a trait defining what our callback
// looks like), so we need to use that, too.`
use rltk::{Rltk, GameState, Console, RGB};

// This is the structure that will store our game state, typically a state machine pointing to
// other structures. This demo is realy simple, so we'll just put the minimum to make it work
// in here.
struct State {
    y : i32,
    going_down : bool
}

// We have to implement the "trait" GameState for our state object. This gives it a callback
// point for the main loop.
impl GameState for State {

    // This is called every time the screen refreshes (a "tick") by RLTK's main loop. Since GUIs
    // require that you process events every turn - rather than just sequentially like a good old text
    // console, you have to run your game as something of a state machine. This will be fleshed out in
    // later tutorials. For now, it just shows you the frame rate and says "Hello World".
    fn tick(&mut self, ctx : &mut Rltk) {
        let col1 = RGB::named(rltk::CYAN);
        let col2 = RGB::named(rltk::RED);
        let percent : f32 = self.y as f32 / (600.0 / 8.0);
        let fg = col1.lerp(col2, percent);

        ctx.cls();
        ctx.print_color(1, self.y, fg, RGB::named(rltk::BLACK), "Hello RLTK World");

        // Lets make the hello bounce up and down
        if self.going_down {
            self.y += 1;
            if self.y > (600/8)-2 { self.going_down = false; }
        } else {
            self.y -= 1;
            if self.y < 2 { self.going_down = true; }
        }

        // We'll also show the frame rate, since we generally care about keeping that high.
        ctx.print_color(40, 1, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), &format!("FPS: {}", ctx.fps));
        ctx.print_color(40, 2, RGB::named(rltk::CYAN), RGB::named(rltk::BLACK),&format!("Frame Time: {} ms", ctx.frame_time_ms));
    }
}

// Every program needs a main() function!
fn main() {
    // Ask RLTK to create an 800x600 window with the title "Hello RLTK World".
    // We need to provide a path in which the minimal shaders required for console
    // rendering are installed. In this case, we're using a relative path to fit with
    // the structure of this repo; ordinarily, you'd copy them into "resources" or "assets"
    // and use that.
    let mut context = Rltk::init_raw(800, 600, "Hello RLTK World", "../../resources");

    // Rendering text requires a font file. In this case, we're using an 8x8 CP437 sprite sheet.
    // Passing the (8,8) defines the size of the characters on the sprite sheet. An x and y is required,
    // because some fonts aren't square.
    // Once again, we're using a big relative path to the font - so that all the examples can share it.
    let font = context.register_font(rltk::Font::load("../../resources/terminal8x8.jpg", (8,8)));

    // We need to create a console to write on, and register it with RLTK.
    context.register_console(rltk::SimpleConsole::init(800/8, 600/8), font);

    // Now we create an empty state object.
    let mut gs = State{ y : 1, going_down: true };

    // Call into RLTK to run the main loop. This handles rendering, and calls back into State's tick
    // function every cycle.
    context.main_loop(&mut gs);
}
