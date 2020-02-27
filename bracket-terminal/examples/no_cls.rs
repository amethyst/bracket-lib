bracket_terminal::add_wasm_support!();
use bracket_random::prelude::*;
use bracket_terminal::prelude::*;

struct State {
    rng: RandomNumberGenerator,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::C => ctx.cls(),
                _ => {}
            }
        }

        let x = self.rng.range(0, 80);
        let y = self.rng.range(0, 50);
        let chr = self.rng.range(0, 255) as u8;

        let fg = RGB::from_u8(
            self.rng.range::<u8>(0, 254),
            self.rng.range::<u8>(0, 254),
            self.rng.range::<u8>(0, 254),
        );
        let bg = RGB::from_u8(
            self.rng.range::<u8>(0, 254),
            self.rng.range::<u8>(0, 254),
            self.rng.range::<u8>(0, 254),
        );

        ctx.set(x, y, fg, bg, chr);
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Hello Minimal Bracket World")
        .build()?;

    let gs: State = State {
        rng: RandomNumberGenerator::new(),
    };

    main_loop(context, gs)
}
