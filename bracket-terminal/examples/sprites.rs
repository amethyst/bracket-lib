bracket_terminal::add_wasm_support!();

use bracket_terminal::prelude::*;

struct State {
    frame: usize,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.print(1, 1, "Watch him go!");
        ctx.printer(
            1,
            2,
            &format!("#[pink]FPS: #[]{}", ctx.fps),
            TextAlign::Left,
            None,
        );

        ctx.set_active_console(1);
        ctx.cls();
        ctx.add_sprite(
            Rect::with_size(
                320 + (f32::sin(self.frame as f32 / 100.0) * 100.0) as i32,
                self.frame as i32 % 400,
                32,
                32,
            ),
            0,
            RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
            self.frame % 4,
            0.0,
        );

        self.frame += 1;
    }
}

bracket_terminal::embedded_resource!(NYAN_CAT, "../resources/sprite_dood.png");

fn main() -> BError {
    bracket_terminal::link_resource!(NYAN_CAT, "../resources/sprite_dood.png");

    let context = BTermBuilder::simple80x50()
        .with_sprite_console(640, 400, 0)
        .with_title("Bracket Terminal - Sprite Console")
        .with_sprite_sheet(
            SpriteSheet::new("../resources/sprite_dood.png")
                .add_sprite(Rect::with_size(0, 0, 85, 132))
                .add_sprite(Rect::with_size(85, 0, 85, 132))
                .add_sprite(Rect::with_size(170, 0, 85, 132))
                .add_sprite(Rect::with_size(255, 0, 85, 132)),
        )
        .with_vsync(false)
        .build()?;

    let gs = State { frame: 0 };

    main_loop(context, gs)
}
