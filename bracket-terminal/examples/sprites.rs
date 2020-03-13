bracket_terminal::add_wasm_support!();

use bracket_random::prelude::*;
use bracket_terminal::prelude::*;

struct Dood {
    x: i32,
    y: i32,
}

struct State {
    frame: usize,
    timer: f32,
    doods: Vec<Dood>,
    rng: RandomNumberGenerator,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(1);
        ctx.cls();
        ctx.print(1, 1, "Watch them go!");
        ctx.printer(
            1,
            2,
            &format!("#[pink]FPS: #[]{}", ctx.fps),
            TextAlign::Left,
            None,
        );

        ctx.set_active_console(0);
        ctx.cls();

        for dood in self.doods.iter() {
            ctx.add_sprite(
                Rect::with_size(dood.x, dood.y, 32, 32),
                400 - dood.y,
                RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                self.frame % 4,
            )
        }

        self.timer += ctx.frame_time_ms;
        if self.timer > 66.0 {
            self.timer = 0.0;
            self.frame += 1;

            for dood in self.doods.iter_mut() {
                dood.x += self.rng.range(0, 3) - 1;
                dood.y += self.rng.range(0, 3) - 1;
            }
        }
    }
}

bracket_terminal::embedded_resource!(NYAN_CAT, "../resources/sprite_dood.png");

fn main() -> BError {
    bracket_terminal::link_resource!(NYAN_CAT, "resources/sprite_dood.png");

    let context = BTermBuilder::new()
        .with_sprite_console(640, 400, 0)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console_no_bg(80, 50, "terminal8x8.png")
        .with_title("Bracket Terminal - Sprite Console")
        .with_sprite_sheet(
            SpriteSheet::new("resources/sprite_dood.png")
                .add_sprite(Rect::with_size(0, 0, 85, 132))
                .add_sprite(Rect::with_size(85, 0, 85, 132))
                .add_sprite(Rect::with_size(170, 0, 85, 132))
                .add_sprite(Rect::with_size(255, 0, 85, 132)),
        )
        .with_vsync(false)
        .build()?;

    let mut rng = RandomNumberGenerator::new();
    let mut doods = Vec::new();
    for _ in 0..100 {
        doods.push(Dood {
            x: rng.range(0, 640),
            y: rng.range(0, 400),
        });
    }

    let gs = State {
        frame: 0,
        timer: 0.0,
        doods,
        rng,
    };

    main_loop(context, gs)
}
