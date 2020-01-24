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
struct BouncingBacy {}

impl Component for BouncingBacy {
    type Storage = VecStorage<Self>;
}

// State gets a new World entry for Specs, an RNG, and a score counter

struct State {
    ecs: World,
    time: f32,
    rng: rltk::RandomNumberGenerator,
    saved: i32,
    squished: i32,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        let mut draw_batch = DrawBatch::new();
        draw_batch.cls();

        // Readable data stores
        let mut positions = self.ecs.write_storage::<Point>();
        let renderables = self.ecs.write_storage::<Renderable>();
        let mut players = self.ecs.write_storage::<Player>();
        let mut babies = self.ecs.write_storage::<BouncingBacy>();

        ctx.cls();

        // Player movement
        match ctx.key {
            None => {} // Nothing happened
            Some(key) => match key {
                VirtualKeyCode::Left => {
                    for (_player, pos) in (&mut players, &mut positions).join() {
                        pos.x -= 1;
                        if pos.x < 0 {
                            pos.x = 0;
                        }
                    }
                }
                VirtualKeyCode::Right => {
                    for (_player, pos) in (&mut players, &mut positions).join() {
                        pos.x += 1;
                        if pos.x > 79 {
                            pos.x = 79;
                        }
                    }
                }
                _ => {}
            },
        }

        self.time += ctx.frame_time_ms;
        if self.time > 200.0 {
            self.time = 0.0;

            // Find the player
            let mut player_x = 0;
            for (_player, player_pos) in (&mut players, &mut positions).join() {
                player_x = player_pos.x;
            }

            // Baby movement
            for (_baby, pos) in (&mut babies, &mut positions).join() {
                pos.y += 1;
                if pos.y > 48 {
                    pos.y = 0;
                    if player_x == pos.x {
                        // We saved them
                        self.saved += 1;
                    } else {
                        // Squish!
                        self.squished += 1;
                    }
                    pos.x = self.rng.roll_dice(1, 79);
                }
            }
        }

        // Draw renderables
        for (pos, render) in (&positions, &renderables).join() {
            draw_batch.set(*pos, ColorPair::new(render.fg, render.bg), render.glyph);
        }

        // Print the score
        draw_batch.print_centered(0, "Left & right arrows to move. Catch the falling babies!");
        draw_batch.print_centered(
            2,
            &format!("Saved {}, Squished {}", self.saved, self.squished),
        );

        draw_batch.submit();
        render_draw_buffer(ctx);
    }
}

fn main() {
    let mut gs = State {
        ecs: World::new(),
        time: 0.0,
        rng: rltk::RandomNumberGenerator::new(),
        saved: 0,
        squished: 0,
    };
    gs.ecs.register::<Point>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<BouncingBacy>();

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
            .with(Point::new((i * 22) + 12, gs.rng.roll_dice(1, 20)))
            .with(Renderable {
                glyph: rltk::to_cp437('☺'),
                fg: RGB::named(rltk::MAGENTA),
                bg: RGB::named(rltk::BLACK),
            })
            .with(BouncingBacy {})
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
