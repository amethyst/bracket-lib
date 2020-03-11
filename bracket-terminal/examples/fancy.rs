bracket_terminal::add_wasm_support!();

use bracket_terminal::prelude::*;

struct State {
    x : f32,
    spin: f32
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        let mut draw_batch = DrawBatch::new();

        // Show frame rate
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

        let y = 20.0 + (f32::sin(self.x / 2.0) * 5.0);
        let scale = f32::cos(self.x / 3.0) + 1.1;
        draw_batch.set_fancy(
            PointF{x: self.x, y}, 
            0, 
            self.spin, 
            PointF{x:scale, y:scale}, 
            ColorPair::new(RGB::named(YELLOW), RGB::named(BLACK)), 
            to_cp437('@')
        );

        // Submission
        draw_batch.submit(0).expect("Batch error");
        render_draw_buffer(ctx).expect("Render error");

        // Moving
        self.x += 0.1;
        if self.x > 80.0 {
            self.x = 0.0;
        }
        self.spin += 0.2;
    }
}

fn main() -> BError {
    let mut context = BTermBuilder::simple80x50()
        .with_fancy_console(80, 50, "terminal8x8.png")
        .with_title("Bracket Terminal - Sparse Consoles")
        .build()?;

    context.with_post_scanlines(true);
    context.with_post_scanlines(true);

    let gs = State {
        x: 0.0,
        spin: 0.0
    };

    main_loop(context, gs)
}
