bracket_terminal::add_wasm_support!();

use bracket_terminal::prelude::*;

struct State {
    x: f32,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        let mut draw_batch = DrawBatch::new();

        draw_batch.target(1);
        draw_batch.cls();


        let simple_x = self.x as i32;
        let fancy_x = self.x + 20.0;

        draw_batch.print(Point::new(0, 0), format!("Simple Console"));
        draw_batch.print(Point::new(0, 1), format!("X={}", simple_x));
        draw_batch.print(Point::new(20, 0), format!("Fancy Console"));
        draw_batch.print(Point::new(20, 1), format!("X={:2}", fancy_x));

        draw_batch.print(Point::new(simple_x, 3), "@");
        draw_batch.set_fancy(
            PointF::new(fancy_x, 4.0),
            1,
            Degrees::new(0.0),
            PointF::new(1.0, 1.0),
            ColorPair::new(WHITE,BLACK),
            to_cp437('@')
        );

        draw_batch.submit(0).expect("Batch error");
        render_draw_buffer(ctx).expect("Render error");

        self.x += 0.05;
        if self.x > 10.0 {
            self.x = 0.0;
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_fancy_console(80, 50, "terminal8x8.png")
        .with_title("Bracket Terminal - Fancy Consoles")
        .with_fps_cap(30.0)
        .build()?;

    let gs = State {
        x: 0.0,
    };

    main_loop(context, gs)
}
