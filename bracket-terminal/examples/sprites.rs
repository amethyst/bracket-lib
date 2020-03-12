bracket_terminal::add_wasm_support!();

use bracket_terminal::prelude::*;

struct State {
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1,1, "Watch him go!");
    }
}

bracket_terminal::embedded_resource!(NYAN_CAT, "../resources/sprite_dood.png");

fn main() -> BError {
    bracket_terminal::link_resource!(NYAN_CAT, "../resources/sprite_dood.png");

    let mut context = BTermBuilder::simple80x50()
        .with_sprite_console(640, 400)
        .with_title("Bracket Terminal - Sprite Console")
        .with_sprite_sheet(
            SpriteSheet::new("sprite_dood.png")
            .add_sprite(Rect::with_size(0, 0, 83, 105))
            .add_sprite(Rect::with_size(83, 0, 83, 105))
            .add_sprite(Rect::with_size(166, 0, 83, 105))
            .add_sprite(Rect::with_size(249, 0, 83, 105))
        )
        .with_vsync(false)
        .build()?;

    context.with_post_scanlines(true);
    context.with_post_scanlines(true);

    let gs = State {  };

    main_loop(context, gs)
}
