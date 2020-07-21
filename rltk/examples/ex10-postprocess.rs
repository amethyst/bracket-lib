rltk::add_wasm_support!();
use rltk::prelude::*;

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

rltk::embedded_resource!(NYAN_CAT, "../resources/nyan.xp");

fn main() -> RltkError {
    rltk::link_resource!(NYAN_CAT, "../resources/nyan.xp");
    let xp = XpFile::from_resource("../resources/nyan.xp").unwrap();

    let mut context = RltkBuilder::simple80x50()
        .with_title("RLTK Example 10 - Post-Processing Effects")
        .build()?;

    context.with_post_scanlines(true);
    let gs: State = State {
        nyan: xp,
        burn: true,
    };
    rltk::main_loop(context, gs)
}
