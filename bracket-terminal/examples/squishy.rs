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
    width: u32,
}

impl State {
    pub fn new() -> Self {
        Self {
            rng: RandomNumberGenerator::new(),
            width: 80,
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        let console_size = ctx.get_char_size();
        for y in 0..console_size.1 as i32 {
            for x in 0..console_size.0 as i32 {
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
        ctx.print_color_centered(
            22,
            RGBA::named(WHITE),
            RGBA::named(BLACK),
            &format!("{} fps", ctx.fps as u32),
        );
        ctx.print_color_centered(
            23,
            RGBA::named(WHITE),
            RGBA::named(BLACK),
            &format!("{} width", self.width),
        );

        self.width += 1;
        self.width %= 200;
        ctx.set_char_size(self.width, 45);
        if let Some(key) = ctx.key {
            if key == VirtualKeyCode::W {
                ctx.set_char_size_and_resize_window(self.width, 45);
            }
        }
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
