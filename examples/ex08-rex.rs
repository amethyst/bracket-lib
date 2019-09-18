rltk::add_wasm_support!();
use rltk::{rex::XpFile, Console, GameState, Rltk, VirtualKeyCode, RGB};
use std::fs::File;

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
        ctx.print_color(
            0,
            2,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            "Press S to save this console to ./screenshot.xp",
        );
        ctx.render_xp_sprite(&self.nyan, 2, 4);

        match ctx.key {
            None => {} // Nothing happened
            Some(key) => {
                // A key is pressed or held
                if let VirtualKeyCode::S = key {
                    // Demonstrates saving the console stack on an xp file
                    let xpfile = ctx.to_xp_file(80, 50);
                    let mut f = File::create("./screenshot.xp").expect("Unable to create file");
                    xpfile.write(&mut f).expect("Unable to save file");
                }
            }
        }
    }
}

fn main() {
    let mut f = File::open("resources/nyan.xp").unwrap();
    let xp = XpFile::read(&mut f).unwrap();

    let context = Rltk::init_simple8x8(80, 50, "Example 8 - Hello Nyan Cat", "resources");
    let gs: State = State { nyan: xp };
    rltk::main_loop(context, gs);
}
