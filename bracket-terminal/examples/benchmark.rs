// This example helps checking the performances of differents
// backends and also performance regression from one version
// to another.
// The goal is to test a pathological case (every console cell
// updated every frame) using the fastest game state as possible
// to be as close as possible as measuring only rendering time
//////////////////////////////////////////////////////////////

use bracket_random::prelude::*;
use bracket_terminal::prelude::*;
bracket_terminal::add_wasm_support!();

struct State {
    rng: RandomNumberGenerator,
}

impl State {
    pub fn new() -> Self {
        Self {
            rng: RandomNumberGenerator::new(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        for y in 0..45 {
            for x in 0..80 {
                let val = self.rng.rand::<u64>();
                let back = RGB::from_u8(
                    (val & 0xFF) as u8,
                    ((val >> 8) & 0x5F) as u8,
                    ((val >> 16) & 0x3F) as u8,
                );
                let fore = RGB::from_u8(
                    ((val >> 16) & 0xFF) as u8,
                    ((val >> 24) & 0xFF) as u8,
                    ((val >> 32) & 0xFF) as u8,
                );
                let ascii = ((val >> 40) & 0xFF) as u8;
                ctx.set(x, y, fore, back, ascii);
            }
        }
        ctx.draw_box(
            30,
            20,
            20,
            5,
            RGB::from_u8(255, 255, 255),
            RGB::from_u8(0, 0, 0),
        );
        ctx.print_centered(22, &format!("{} fps", ctx.fps as u32));
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple(80, 45)
        .unwrap()
        .with_title("bracket-lib benchmark")
        .with_vsync(false)
        .build()?;
    main_loop(context, State::new())
}
