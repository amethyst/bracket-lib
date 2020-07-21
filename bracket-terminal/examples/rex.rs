bracket_terminal::add_wasm_support!();
use bracket_terminal::prelude::*;

struct State {
    nyan: XpFile,
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
        ctx.render_xp_sprite(&self.nyan, 2, 4);
    }
}

// This is a helper macro to embed a file in your binary.
bracket_terminal::embedded_resource!(NYAN_CAT, "../resources/nyan.xp");

fn main() -> BError {
    // This helper macro links the above embedding, allowing it to be accessed as a resource from various parts of the program.
    bracket_terminal::link_resource!(NYAN_CAT, "../resources/nyan.xp");
    let xp = XpFile::from_resource("../resources/nyan.xp")?;

    let context = BTermBuilder::simple80x50()
        .with_title("Bracket Terminal Example - REX Paint, Hello Nyan Cat")
        .build()?;
    let gs: State = State { nyan: xp };
    main_loop(context, gs)
}
