rltk::add_wasm_support!();
use rltk::{to_cp437, Console, GameState, Rltk, RGB};

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.print_color_centered(
            3,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            "Offset Consoles",
        );
        ctx.print_color_centered(
            4,
            RGB::named(rltk::GRAY),
            RGB::named(rltk::BLACK),
            "For those times you want thin walls",
        );

        for y in 20..30 {
            for x in 30..50 {
                ctx.set(
                    x,
                    y,
                    RGB::named(rltk::GRAY),
                    RGB::named(rltk::BLACK),
                    to_cp437('░'),
                )
            }
        }

        ctx.set(
            40,
            25,
            RGB::named(rltk::YELLOW),
            RGB::named(rltk::BLACK),
            to_cp437('☺'),
        );

        ctx.set_active_console(1);
        ctx.cls();
        ctx.print_color(
            38,
            24,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            "┌──┐",
        );
        ctx.print_color(
            38,
            25,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            "│  │",
        );
        ctx.print_color(
            38,
            26,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            "│  │",
        );
        ctx.print_color(
            38,
            27,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            "└┐ │",
        );
        ctx.print_color(
            38,
            28,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            " └─┘",
        );
    }
}

fn main() {
    let mut context = Rltk::init_simple8x8(80, 50, "Example 9 - Offset Tiles", "resources");
    context.register_console_no_bg(rltk::SparseConsole::init(80, 50, &context.backend.gl), 0);

    // We're going to set the second layer's offset to -0.5 to render between tiles
    context.set_active_console(1);
    context.set_offset(0.5, 0.5);

    // Returning to the default console is a good plan
    context.set_active_console(0);
    let gs: State = State {};
    rltk::main_loop(context, gs);
}
