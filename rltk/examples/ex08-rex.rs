rltk::add_wasm_support!();
use rltk::prelude::*;

struct State {
    nyan: XpFile,
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
        ctx.render_xp_sprite(&self.nyan, 2, 4);
    }
}

// This is a helper macro to embed a file in your binary.
rltk::embedded_resource!(NYAN_CAT, "../resources/nyan.xp");

fn main() -> RltkError {
    // This helper macro links the above embedding, allowing it to be accessed as a resource from various parts of the program.
    rltk::link_resource!(NYAN_CAT, "../resources/nyan.xp");
    let xp = XpFile::from_resource("../resources/nyan.xp").unwrap();

    let context = RltkBuilder::simple80x50()
        .with_title("RLTK Example 8 - REX Paint, Hello Nyan Cat")
        .build()?;
    let gs: State = State { nyan: xp };
    rltk::main_loop(context, gs)
}
