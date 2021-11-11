// This example demonstrates using the "with_z" functionality
// inside batched rendering.
//////////////////////////////////////////////////////////////

use bracket_terminal::prelude::*;
bracket_terminal::add_wasm_support!();

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        let mut draw_batch = DrawBatch::new();
        draw_batch.print_color_with_z(Point::new(10, 10), "This is at always on top", ColorPair::new(YELLOW, BLUE), 1000);
        for y in 0..50 {
            for x in 0..80 {
                draw_batch.set(
                    Point::new(x, y),
                    ColorPair::new(DARKGRAY, BLACK),
                    to_cp437('#')
                );
            }
        }
        draw_batch.submit(0).expect("Oops");
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Hello Minimal Bracket World")
        .build()?;

    let gs: State = State {};
    main_loop(context, gs)
}
