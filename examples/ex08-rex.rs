extern crate rltk;

use rltk::{Rltk, GameState, Console, rex::XpFile, RGB};
use std::fs::File;

struct State {
    nyan : XpFile
}

impl GameState for State {

    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();
        ctx.print_color(0, 0, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), "Hello Nyan Cat!");
        ctx.print_color(0, 1, RGB::named(rltk::GREEN), RGB::named(rltk::BLACK), "Loaded from REX Paint (https://www.gridsagegames.com/rexpaint/)");
        ctx.render_xp_sprite(&self.nyan, 2, 3);
    }
}

fn main() {
    let mut f = File::open("resources/nyan.xp").unwrap();
    let xp = XpFile::read(&mut f).unwrap();

    let context = Rltk::init_simple8x8(80, 50, "Example 8 - Hello Nyan Cat", "resources");
    let gs : State = State{ nyan: xp };
    rltk::main_loop(context, Box::new(gs));
}
