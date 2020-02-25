#![warn(clippy::pedantic)]
// This is the canonical "Hello World" example for RLTK.
// It's like example 01, but we implement a sparse second terminal using a nicer VGA font
// for the FPS and frame time portions. This illustrates how you can combine multiple fonts
// on a single layered console.
//////////////////////////////////////////////////////////////

// We're utilizing functionality from RLTK, so we need to tell it to use the crate.
rltk::add_wasm_support!();
extern crate rltk;

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
        let mut draw_batch = DrawBatch::new();
        let col1 = RGB::named(rltk::CYAN);
        let col2 = RGB::named(rltk::YELLOW);
        let percent: f32 = self.y as f32 / 50.0;
        let fg = col1.lerp(col2, percent);

        // The first console created (8x8) is always console 0. This makes it the recipient
        // of draw calls sent to ctx. You can also do ctx.consoles[0] - but that's more typing.
        draw_batch.target(0);
        draw_batch.cls();
        draw_batch.print_color(
            Point::new(1, self.y),
            "Hello RLTK World",
            ColorPair::new(fg, RGB::named(rltk::BLACK)),
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
        // We want to show this in VGA 8x16 font, so we'll set to console 1 - the one we added.
        // Again, this could be ctx.consoles[1] - but the short-hand is easier.
        draw_batch.target(1);
        draw_batch.cls();
        draw_batch.draw_double_box(
            Rect::with_size(39, 0, 20, 3),
            ColorPair::new(RGB::named(rltk::WHITE), RGB::named(rltk::BLACK)),
        );
        draw_batch.print_color(
            Point::new(40, 1),
            &format!("FPS: {}", ctx.fps),
            ColorPair::new(RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK)),
        );
        draw_batch.print_color(
            Point::new(40, 2),
            &format!("Frame Time: {} ms", ctx.frame_time_ms),
            ColorPair::new(RGB::named(rltk::CYAN), RGB::named(rltk::BLACK)),
        );
        draw_batch.submit(0).expect("Batch error");

        render_draw_buffer(ctx).expect("Render error");
    }
}

// Every program needs a main() function!
fn main() -> RltkError {
    // We're using the RLTK "builder" system to define what we want. We start with a simple
    // 80x50 background layer.
    let context = RltkBuilder::simple80x50()
        // Then we register the 8x16 VGA font. This is embedded automatically, so you can just use it.
        .with_font("vga8x16.png", 8, 16)
        // Next we want a "sparse" (no background) console, of half the height since its an 8x16 font.
        .with_sparse_console(80, 25, "vga8x16.png")
        // And a window title
        .with_title("RLTK Example 2 - Sparse Consoles")
        // And call the build function to actually obtain the context.
        .build()?;

    // Now we create an empty state object.
    let gs = State {
        y: 1,
        going_down: true,
    };

    // Call into RLTK to run the main loop. This handles rendering, and calls back into State's tick
    // function every cycle.
    rltk::main_loop(context, gs)
}
