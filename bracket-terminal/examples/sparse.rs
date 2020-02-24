bracket_terminal::add_wasm_support!();

use bracket_terminal::prelude::*;

struct State {
    y: i32,
    going_down: bool,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        let mut draw_batch = DrawBatch::new();
        let col1 = RGB::named(CYAN);
        let col2 = RGB::named(YELLOW);
        let percent: f32 = self.y as f32 / 50.0;
        let fg = col1.lerp(col2, percent);

        // The first console created (8x8) is always console 0. This makes it the recipient
        // of draw calls sent to ctx. You can also do ctx.consoles[0] - but that's more typing.
        draw_batch.target(0);
        draw_batch.cls();
        draw_batch.print_color(
            Point::new(1, self.y),
            "Hello Bracket World",
            ColorPair::new(fg, RGB::named(BLACK)),
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
            ColorPair::new(RGB::named(WHITE), RGB::named(BLACK)),
        );
        draw_batch.print_color(
            Point::new(40, 1),
            &format!("FPS: {}", ctx.fps),
            ColorPair::new(RGB::named(YELLOW), RGB::named(BLACK)),
        );
        draw_batch.print_color(
            Point::new(40, 2),
            &format!("Frame Time: {} ms", ctx.frame_time_ms),
            ColorPair::new(RGB::named(CYAN), RGB::named(BLACK)),
        );
        draw_batch.submit(0).expect("Batch error");

        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_font("vga8x16.png", 8u32, 16u32)
        .with_sparse_console(80u32, 25u32, "vga8x16.png")
        .with_title("Bracket Terminal - Sparse Consoles")
        .build()?;

    let gs = State {
        y: 1,
        going_down: true,
    };

    main_loop(context, gs)
}
