bracket_terminal::add_wasm_support!();

use bracket_terminal::prelude::*;

struct State {
    display_lines: Vec<String>,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        for (y, line) in self.display_lines.iter().enumerate() {
            ctx.print(0, y as i32, &line);
        }

        match ctx.key {
            None => {}
            Some(key) => {
                self.display_lines.push(format!(
                    "{} pressed, shift: {}, control: {}, alt: {}",
                    key as i32, ctx.shift, ctx.control, ctx.alt
                ));
                while self.display_lines.len() > 49 {
                    self.display_lines.remove(0);
                }
            }
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("RLTK Example 16 - Keyboard Input Visualizer")
        .build()?;

    let gs: State = State {
        display_lines: vec!["Press keys and modifiers to see code combinations.".to_string()],
    };

    main_loop(context, gs)
}
