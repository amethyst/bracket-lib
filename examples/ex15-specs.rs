#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
rltk::add_wasm_support!();
use rltk::prelude::*;
use specs::prelude::*;

// Define a bunch of components

/// Renderable is a glyph definition
struct Renderable {
    glyph: u8,
    fg: RGB,
    bg: RGB,
}

impl Component for Renderable {
    type Storage = VecStorage<Self>;
}

/// Marker for this is the player
struct Player {}

impl Component for Player {
    type Storage = VecStorage<Self>;
}

/// Marker for this is a bouncing baby
struct BouncingBaby {}

impl Component for BouncingBaby {
    type Storage = VecStorage<Self>;
}

struct GameInfo {
    time: f32,
    saved: i32,
    squished: i32,
    rng: RandomNumberGenerator,
}

impl Default for GameInfo {
    fn default() -> Self {
        GameInfo {
            time: 0.0,
            saved: 0,
            squished: 0,
            rng: rltk::RandomNumberGenerator::new(),
        }
    }
}
struct SysRunner {
    dispatcher: Dispatcher<'static, 'static>,
}

impl SysRunner {
    pub fn new() -> Self {
        let mut dispatcher = DispatcherBuilder::new()
            .with(PlayerMovementSystem, "player_move", &[])
            .with(BabyMovementSystem, "baby_fall", &[])
            .with(RenderableSystem, "render", &[])
            .with(UiSystem, "ui", &[])
            .build();

        SysRunner { dispatcher }
    }

    pub fn run(&mut self, ecs: &mut World) {
        self.dispatcher.dispatch(ecs);
        ecs.maintain();
    }
}

struct State {
    ecs: World,
    systems: SysRunner,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        // Start with a screen clear
        let mut draw_batch = DrawBatch::new();
        draw_batch.cls();
        draw_batch.submit();

        // Insert some resources so systems have access to them
        self.ecs.insert(ctx.key);
        self.ecs.insert(ctx.frame_time_ms);

        // Running the systems...
        self.systems.run(&mut self.ecs);

        // Commit the rendering
        render_draw_buffer(ctx);
    }
}

struct PlayerMovementSystem;

impl<'a> System<'a> for PlayerMovementSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        WriteStorage<'a, Point>,
        Read<'a, Option<VirtualKeyCode>>,
    );

    fn run(&mut self, (players, mut positions, maybe_key): Self::SystemData) {
        match *maybe_key {
            None => {} // Nothing happened
            Some(key) => match key {
                VirtualKeyCode::Left => {
                    for (_player, pos) in (&players, &mut positions).join() {
                        pos.x -= 1;
                        if pos.x < 0 {
                            pos.x = 0;
                        }
                    }
                }
                VirtualKeyCode::Right => {
                    for (_player, pos) in (&players, &mut positions).join() {
                        pos.x += 1;
                        if pos.x > 79 {
                            pos.x = 79;
                        }
                    }
                }
                _ => {}
            },
        }
    }
}

struct BabyMovementSystem;

impl<'a> System<'a> for BabyMovementSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        WriteStorage<'a, Point>,
        ReadStorage<'a, BouncingBaby>,
        Write<'a, GameInfo>,
        Read<'a, f32>,
    );

    fn run(&mut self, (players, mut positions, babies, mut info, frame_time): Self::SystemData) {
        info.time += *frame_time;
        if info.time > 200.0 {
            info.time = 0.0;
            // Find the player
            let mut player_x = 0;
            for (_player, player_pos) in (&players, &mut positions).join() {
                player_x = player_pos.x;
            }

            // Baby movement
            for (_baby, pos) in (&babies, &mut positions).join() {
                pos.y += 1;
                if pos.y > 48 {
                    pos.y = 0;
                    if player_x == pos.x {
                        // We saved them
                        info.saved += 1;
                    } else {
                        // Squish!
                        info.squished += 1;
                    }
                    pos.x = info.rng.roll_dice(1, 79);
                }
            }
        }
    }
}

struct RenderableSystem;

impl<'a> System<'a> for RenderableSystem {
    type SystemData = (ReadStorage<'a, Point>, ReadStorage<'a, Renderable>);

    fn run(&mut self, (positions, renderables): Self::SystemData) {
        let mut draw_batch = DrawBatch::new();
        for (pos, render) in (&positions, &renderables).join() {
            draw_batch.set(*pos, ColorPair::new(render.fg, render.bg), render.glyph);
        }
        draw_batch.submit();
    }
}

struct UiSystem;

impl<'a> System<'a> for UiSystem {
    type SystemData = Read<'a, GameInfo>;

    fn run(&mut self, info: Self::SystemData) {
        let mut draw_batch = DrawBatch::new();
        draw_batch.print_centered(0, "Left & right arrows to move. Catch the falling babies!");
        draw_batch.print_centered(
            2,
            &format!("Saved {}, Squished {}", info.saved, info.squished),
        );

        draw_batch.submit();
    }
}

fn main() {
    let mut gs = State {
        ecs: World::new(),
        systems: SysRunner::new(),
    };
    gs.ecs.register::<Point>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<BouncingBaby>();
    gs.ecs.insert(GameInfo {
        time: 0.0,
        saved: 0,
        squished: 0,
        rng: rltk::RandomNumberGenerator::new(),
    });

    gs.ecs
        .create_entity()
        .with(Point { x: 40, y: 49 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .build();

    for i in 0..3 {
        gs.ecs
            .create_entity()
            .with(Point::new((i * 22) + 12, 1 + (i * 10)))
            .with(Renderable {
                glyph: rltk::to_cp437('☺'),
                fg: RGB::named(rltk::MAGENTA),
                bg: RGB::named(rltk::BLACK),
            })
            .with(BouncingBaby {})
            .build();
    }

    let context = Rltk::init_simple8x8(
        80,
        50,
        "Example 15 - Bouncing Babies with SPECS",
        "resources",
    );
    rltk::main_loop(context, gs);
}
