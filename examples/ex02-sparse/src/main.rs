// This is the canonical "Hello World" example for RLTK.
// It's like example 01, but we implement a sparse second terminal using a nicer VGA font
// for the FPS and frame time portions. This illustrates how you can combine multiple fonts
// on a single layered console.
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
        let col2 = RGB::named(rltk::YELLOW);
        let percent : f32 = self.y as f32 / 50.0;
        let fg = col1.lerp(col2, percent);

        // The first console created (8x8) is always console 0. This makes it the recipient
        // of draw calls sent to ctx. You can also do ctx.consoles[0] - but that's more typing.
        ctx.set_active_console(0);
        ctx.cls();
        ctx.print_color(1, self.y, fg, RGB::named(rltk::BLACK), "Hello RLTK World");

        // Lets make the hello bounce up and down
        if self.going_down {
            self.y += 1;
            if self.y > 48 { self.going_down = false; }
        } else {
            self.y -= 1;
            if self.y < 2 { self.going_down = true; }
        }

        // We'll also show the frame rate, since we generally care about keeping that high.
        // We want to show this in VGA 8x16 font, so we'll set to console 1 - the one we added.
        // Again, this could be ctx.consoles[1] - but the short-hand is easier.
        ctx.set_active_console(1);
        ctx.cls();
        ctx.print_color(40, 1, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), &format!("FPS: {}", ctx.fps));
        ctx.print_color(40, 2, RGB::named(rltk::CYAN), RGB::named(rltk::BLACK),&format!("Frame Time: {} ms", ctx.frame_time_ms));
    }
}

// Every program needs a main() function!
fn main() {
    // RLTK provides a simple initializer for a simple 8x8 font window of a given number of
    // characters. Since that's all we need here, we'll use it!
    // We're specifying that we want an 80x50 console, with a title, and a relative path
    // to where it can find the font files and shader files.
    // These would normally be "resources" rather than "../../resources" - but to make it
    // work in the repo without duplicating, they are a relative path.
    let mut context = Rltk::init_simple8x8(80, 50, "Hello RLTK World", "../../resources");

    // We want to add a second layer, using an 8x16 VGA font. It looks nicer, and shows how
    // RLTK can have layers.
    //
    // We start by loading the font.
    let font = context.register_font(rltk::Font::load("../../resources/vga8x16.jpg", (8,16)));

    // Then we initialize it; notice 80x25 (half the height, since 8x16 is twice as tall).
    // This actually returns the console number, but it's always going to be 1.
    context.register_console(rltk::SparseConsole::init(80, 25, &context.gl), font);

    // Now we create an empty state object.
    let gs = State{ y : 1, going_down: true };

    // Call into RLTK to run the main loop. This handles rendering, and calls back into State's tick
    // function every cycle.
    rltk::main_loop(context, Box::new(gs));
}
