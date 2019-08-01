extern crate rltk;

use rltk::{rex::XpFile, Console, GameState, Rltk, VirtualKeyCode, RGB};
use std::fs::File;

struct State {
    nyan: XpFile,
    burn: bool,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        ctx.print_color(
            0,
            0,
            RGB::named(rltk::YELLOW),
            RGB::named(rltk::BLACK),
            "Hello Nyan Cat!",
        );
        ctx.print_color(
            0,
            1,
            RGB::named(rltk::GREEN),
            RGB::named(rltk::BLACK),
            "Loaded from REX Paint (https://www.gridsagegames.com/rexpaint/)",
        );
        ctx.print_color(
            0,
            2,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            "Press B to toggle burn.",
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
            }
        }
    }
}

fn main() {
    let mut f = File::open("resources/nyan.xp").unwrap();
    let xp = XpFile::read(&mut f).unwrap();

    let mut context =
        Rltk::init_simple8x8(80, 50, "Example 10 - Post Process Effects", "resources");
    context.with_post_scanlines(true);
    let gs: State = State {
        nyan: xp,
        burn: true,
    };
    rltk::main_loop(context, gs);
}
