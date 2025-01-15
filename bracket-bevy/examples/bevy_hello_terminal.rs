use bevy::prelude::*;
use bracket_bevy::prelude::*;

fn main() {
    let bterm = BTermBuilder::simple_80x50()
        .with_named_color("blue", BLUE)
        .with_named_color(
            "pink",
            bracket_color::rgba::RGBA::from_f32(255.0, 182.0, 193.0, 1.0),
        );

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bterm)
        .insert_resource(State {
            y: 0,
            going_down: true,
        })
        .add_systems(Update, tick)
        .run();
}

#[derive(Resource)]
struct State {
    y: i32,
    going_down: bool,
}

fn tick(ctx: Res<BracketContext>, mut state: ResMut<State>) {
    let col1 = RGB::named(CYAN);
    let col2 = RGB::named(YELLOW);
    let percent: f32 = state.y as f32 / 50.0;
    let fg = col1.lerp(col2, percent);

    ctx.cls();
    ctx.printer(
        40,
        49,
        "#[blue]Hello #[pink]Bracket#[] world.",
        TextAlign::Center,
        Some(RGBA::from_u8(200, 200, 200, 255)),
    );

    ctx.print_color(
        1,
        state.y,
        "♫ ♪ Hello Bracket World ☺",
        fg,
        RGB::named(BLACK),
    );

    // Lets make the hello bounce up and down
    if state.going_down {
        state.y += 1;
        if state.y > 48 {
            state.going_down = false;
        }
    } else {
        state.y -= 1;
        if state.y < 2 {
            state.going_down = true;
        }
    }

    // We'll also show the frame rate, since we generally care about keeping that high.
    ctx.draw_box(39, 0, 20, 3, RGB::named(WHITE), RGB::named(BLACK));
    ctx.printer(
        58,
        1,
        &format!("#[pink]FPS: #[]{}", ctx.fps as u32),
        TextAlign::Right,
        None,
    );
    ctx.printer(
        58,
        2,
        &format!("#[pink]Frame Time: #[]{} ms", ctx.frame_time_ms),
        TextAlign::Right,
        None,
    );
}
