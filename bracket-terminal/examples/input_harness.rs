use bracket_terminal::prelude::*;

bracket_terminal::add_wasm_support!();

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        let mut input = INPUT.lock().unwrap();
        let mouse_pixels = input.mouse_pixel_pos();
        ctx.print(1, 1, &format!("Mouse pixel position: {}, {}", mouse_pixels.0, mouse_pixels.1));
        let mouse_tile = input.mouse_tile(0);
        ctx.print(1, 2, &format!("Mouse tile position: {}, {}", mouse_tile.x, mouse_tile.y));
        ctx.print(1, 3, &format!("FPS: {}", ctx.fps));

        input.for_each_message(|event| {
            bracket_terminal::console::log(&format!("{:#?}", event));
            if event == BEvent::CloseRequested {
                ctx.quitting = true;
            }
         });
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Input Harness - Check Your STDOUT")
        .with_vsync(false)
        .with_advanced_input(true)
        .build()?;

    let gs: State = State {};

    main_loop(context, gs)
}
