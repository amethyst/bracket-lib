use bracket_terminal::prelude::*;

bracket_terminal::add_wasm_support!();

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_translation_mode(0, CharacterTranslationMode::Unicode);
        ctx.print(1, 1, "Hello Bracket World");

        ctx.print(20, 8, "こんにちは!");
        ctx.print(20, 10, "真棒!");
        ctx.print(20, 12, "классно");
        ctx.print(20, 14, "Φοβερός!");
        ctx.print(20, 16, "Ça, c'est énorme!");
    }
}

bracket_terminal::embedded_resource!(TILE_FONT3, "../resources/unicode_16x16.png");

fn main() -> BError {
    bracket_terminal::link_resource!(TILE_FONT3, "resources/unicode_16x16.png");

    let context = BTermBuilder::new()
        .with_dimensions(80, 50)
        .with_tile_dimensions(16, 16)
        .with_title("Hello Minimal Bracket World")
        .with_font("unicode_16x16.png", 16, 16)
        .with_simple_console(80, 50, "unicode_16x16.png")
        .build()?;

    let gs: State = State {};

    main_loop(context, gs)
}
