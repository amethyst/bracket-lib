rltk::add_wasm_support!();
use rltk::prelude::*;
use specs::prelude::*;

////////////////////////////////
// Define a bunch of components
////////////////////////////////

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

////////////////////////////////
// Define a resource
////////////////////////////////

// We're breaking out game state that the systems need into a GameInfo structure.
// We'll insert it as a resource, so systems can gain access to it as needed.
// In a real example, we'd probably break it into multiple resources.
struct GameInfo {
    time: f32,
    saved: i32,
    squished: i32,
    rng: RandomNumberGenerator,
}

// Specs wants resources to implement default, so here we are.
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

////////////////////////////////
// Define the systems dispatcher
////////////////////////////////

// This is a messy structure because WASM and threads don't co-exist well together. So if we are using
// a native setup, we'll use a Specs dispatcher. Unfortunately, that doesn't even compile on WASM, so we
// are doing some conditional compilation.
#[cfg(not(target_arch = "wasm32"))]
struct SysRunner {
    dispatcher: Dispatcher<'static, 'static>,
}

#[cfg(target_arch = "wasm32")]
struct SysRunner {}

impl SysRunner {
    // This makes a SysRunner with a dispatcher, so it's native code only.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new() -> Self {
        let dispatcher = DispatcherBuilder::new()
            .with(PlayerMovementSystem, "player_move", &[])
            .with(BabyMovementSystem, "baby_fall", &[])
            .with(RenderableSystem, "render", &[])
            .with(UiSystem, "ui", &[])
            .build();

        SysRunner { dispatcher }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn new() -> Self {
        SysRunner {}
    }

    // Non-WASM version of the runner - call the dispatcher and update the world.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn run(&mut self, ecs: &mut World) {
        self.dispatcher.dispatch(ecs);
        ecs.maintain();
    }

    #[cfg(target_arch = "wasm32")]
    pub fn run(&mut self, ecs: &mut World) {
        let mut pms = PlayerMovementSystem {};
        let mut bms = BabyMovementSystem {};
        let mut render = RenderableSystem {};
        let mut ui = UiSystem {};
        pms.run_now(ecs);
        bms.run_now(ecs);
        render.run_now(ecs);
        ui.run_now(ecs);
        ecs.maintain();
    }
}

////////////////////////////////
// Game State for RLTK
////////////////////////////////

// Our game state holds the World (ECS) and our systems runner abstraction.
struct State {
    ecs: World,
    systems: SysRunner,
}

impl GameState for State {
    // Notice how we've got `tick` down to such a small function by using the dispatcher.
    fn tick(&mut self, ctx: &mut Rltk) {
        // Start with a screen clear
        let mut draw_batch = DrawBatch::new();
        draw_batch.cls();
        draw_batch.submit(0);

        // Insert some resources so systems have access to them
        self.ecs.insert(ctx.key);
        self.ecs.insert(ctx.frame_time_ms);

        // Running the systems...
        self.systems.run(&mut self.ecs);

        // Commit the rendering
        render_draw_buffer(ctx);
    }
}

////////////////////////////////
// Systems
////////////////////////////////

// The PlayerMovementSystem reads the keyboard and moves the player as necessary.
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

// The BabyMovementSystem reads the positions of babies and has them fall downwards. It accesses frame time
// to ensure that the babies don't ZOOM to their death. It also reads the Player's position.
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

// The RenderableSystem draws all entities with a Renderable and Point/Position
struct RenderableSystem;

impl<'a> System<'a> for RenderableSystem {
    type SystemData = (ReadStorage<'a, Point>, ReadStorage<'a, Renderable>);

    fn run(&mut self, (positions, renderables): Self::SystemData) {
        let mut draw_batch = DrawBatch::new();
        for (pos, render) in (&positions, &renderables).join() {
            draw_batch.set(*pos, ColorPair::new(render.fg, render.bg), render.glyph);
        }
        draw_batch.submit(1000);
    }
}

// The UI system renders the instructions and score.
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

        draw_batch.submit(2000);
    }
}

////////////////////////////////
// Main program
////////////////////////////////

fn main() {
    // We start by making a new game state containing an ECS and dispatcher.
    let mut gs = State {
        ecs: World::new(),
        systems: SysRunner::new(),
    };

    // Specs requires that we register our component types.
    gs.ecs.register::<Point>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<BouncingBaby>();

    // We need GameInfo to be a resource so all systems can access it if needed.
    gs.ecs.insert(GameInfo::default());

    // Insert the player
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

    // Insert 3 falling babies
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

    // Initialize RLTK
    let context = Rltk::init_simple8x8(
        80,
        50,
        "Example 15 - Bouncing Babies with SPECS",
        "resources",
    );

    // Run the game loop
    rltk::main_loop(context, gs);
}
