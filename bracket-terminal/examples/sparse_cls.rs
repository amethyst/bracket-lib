use bracket_terminal::prelude::{main_loop, BTerm, BTermBuilder, GameState, VirtualKeyCode, RGBA};

const USE_WORKAROUND: bool = false;
fn main() {
    let context = BTermBuilder::simple80x50()
        .with_font("vga8x16.png", 8, 16)
        .with_tile_dimensions(16, 16)
        .with_sparse_console(80, 25, "vga8x16.png")
        .build()
        .unwrap();

    main_loop(
        context,
        Demo {
            show_outer_console: false,
        },
    )
    .unwrap();
}

struct Demo {
    show_outer_console: bool,
}

impl GameState for Demo {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.print_centered(0, "Main console - overlapping text");

        if self.show_outer_console {
            ctx.set_active_console(1);
            ctx.cls();
            ctx.print_centered(0, "Sparse console");
        }

        match ctx.key {
            Some(VirtualKeyCode::Space) => {
                if self.show_outer_console {
                    ctx.set_active_console(1);
                    ctx.cls();

                    if USE_WORKAROUND {
                        let transparent = RGBA::from_f32(0., 0., 0., 0.);
                        ctx.set(40, 0, transparent, transparent, ' ' as u16);
                    }
                }
                self.show_outer_console = !self.show_outer_console
            }
            _ => (),
        }
    }
}