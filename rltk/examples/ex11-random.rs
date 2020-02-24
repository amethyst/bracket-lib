rltk::add_wasm_support!();
use rltk::prelude::*;

struct State {
    rng: RandomNumberGenerator,
    n_rolls: u32,
    rolls: Vec<u32>,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        self.n_rolls += 1;

        // Handle rollover
        if self.n_rolls == std::u32::MAX {
            self.n_rolls = 1;
            for d in &mut self.rolls {
                *d = 0;
            }
        }

        let roll = self.rng.roll_dice(3, 6) as usize;
        self.rolls[roll] += 1;

        let max = self.rolls.iter().max().unwrap();

        ctx.cls();
        ctx.print(0, 1, "3d6 Distribution");
        for i in 3..19 {
            ctx.print(5, i, &format!("{:02}  : {}", i, self.rolls[i as usize]));
            ctx.draw_bar_horizontal(
                20,
                i,
                50,
                self.rolls[i as usize] as i32,
                *max as i32,
                RGB::named(rltk::GREEN),
                RGB::named(rltk::BLACK),
            );
        }

        ctx.print(5, 22, &format!("Total rolls: {}", self.n_rolls));
    }
}

fn main() -> RltkError {
    let context = RltkBuilder::vga(80, 23)
        .with_title("Example 11 - Random Numbers")
        .build()?;
    let gs: State = State {
        rng: RandomNumberGenerator::new(),
        n_rolls: 0,
        rolls: vec![0; 19],
    };
    rltk::main_loop(context, gs)
}
