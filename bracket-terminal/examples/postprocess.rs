bracket_terminal::add_wasm_support!();
use bracket_terminal::prelude::*;
use bracket_random::prelude::*;

struct State {
    nyan: XpFile,
    burn: bool,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_color(
            0,
            0,
            RGB::named(YELLOW),
            RGB::named(BLACK),
            "Hello Nyan Cat!",
        );
        ctx.print_color(
            0,
            1,
            RGB::named(GREEN),
            RGB::named(BLACK),
            "Loaded from REX Paint (https://www.gridsagegames.com/rexpaint/)",
        );
        ctx.print_color(
            0,
            2,
            RGB::named(WHITE),
            RGB::named(BLACK),
            "Press B to toggle burn.",
        );
        ctx.print_color(
            0,
            3,
            RGB::named(WHITE),
            RGB::named(BLACK),
            "Press C to change burn color.",
        );
        ctx.render_xp_sprite(&self.nyan, 2, 4);

        match ctx.key {
            None => {} // Nothing happened
            Some(key) => {
                // A key is pressed or held
                if let VirtualKeyCode::B = key {
                    self.burn = !self.burn;
                    ctx.with_post_scanlines(self.burn);
                }
                if let VirtualKeyCode::C = key {
                    let mut rng = RandomNumberGenerator::new();
                    let n = rng.range(0, 6);
                    match n {
                    0 => ctx.screen_burn_color(RGB::named(RED)),
                    1 => ctx.screen_burn_color(RGB::named(YELLOW)),
                    2 => ctx.screen_burn_color(RGB::named(GREEN)),
                    3 => ctx.screen_burn_color(RGB::named(BLUE)),
                    4 => ctx.screen_burn_color(RGB::named(MAGENTA)),
                    _ => ctx.screen_burn_color(RGB::named(CYAN)),
                    }
                }
            }
        }
    }
}

bracket_terminal::embedded_resource!(NYAN_CAT, "../resources/nyan.xp");

fn main() -> BError {
    bracket_terminal::link_resource!(NYAN_CAT, "../resources/nyan.xp");
    let xp = XpFile::from_resource("../resources/nyan.xp").unwrap();

    let mut context = BTermBuilder::simple80x50()
        .with_title("Bracket Terminal Example - Post-Processing Effects")
        .build()?;

    context.with_post_scanlines(true);
    let gs: State = State {
        nyan: xp,
        burn: true,
    };
    main_loop(context, gs)
}
