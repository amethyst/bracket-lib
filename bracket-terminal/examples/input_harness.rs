use bracket_terminal::prelude::*;

bracket_terminal::add_wasm_support!();

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        let mouse_pixels = ctx.input.mouse_pixel_pos();
        ctx.print(1, 1, &format!("Mouse pixel position: {}, {}", mouse_pixels.0, mouse_pixels.1));
        let mouse_tile = ctx.input.mouse_tile(0);
        ctx.print(1, 2, &format!("Mouse tile position: {}, {}", mouse_tile.x, mouse_tile.y));

        loop {
            let event = ctx.input.pop();
            if let Some(event) = event {
                println!("{:#?}", event);
                if event == BEvent::CloseRequested {
                    ctx.quitting = true;
                }
            } else {
                break;
            }
        }
    }
}

fn main() -> BError {
    let mut context = BTermBuilder::simple80x50()
        .with_title("Input Harness - Check Your STDOUT")
        .build()?;
    context.input.activate_event_queue();

    let gs: State = State {};

    main_loop(context, gs)
}
