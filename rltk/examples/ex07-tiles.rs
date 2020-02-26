// This is based on example 3, but adds in highlighting visible tiles.
//
// Comments that duplicate previous examples have been removed for brevity.
//////////////////////////////////////////////////////////////

rltk::add_wasm_support!();
use rltk::prelude::*;

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall,
    Floor,
}

const WIDTH: i32 = 40;
const HEIGHT: i32 = 25;

// Just like example 3, but we're adding an additional vector: visible
struct State {
    map: Vec<TileType>,
    player_position: usize,
    visible: Vec<bool>,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * WIDTH as usize) + x as usize
}

pub fn idx_xy(idx: usize) -> (i32, i32) {
    (idx as i32 % WIDTH, idx as i32 / WIDTH)
}

impl State {
    pub fn new() -> State {
        // Same as example 3, but we've added the visible tiles
        let mut state = State {
            map: vec![TileType::Floor; (WIDTH * HEIGHT) as usize],
            player_position: xy_idx(WIDTH / 2, HEIGHT / 2),
            visible: vec![false; (WIDTH * HEIGHT) as usize],
        };

        for x in 0..WIDTH {
            state.map[xy_idx(x, 0)] = TileType::Wall;
            state.map[xy_idx(x, HEIGHT - 1)] = TileType::Wall;
        }
        for y in 0..HEIGHT {
            state.map[xy_idx(0, y)] = TileType::Wall;
            state.map[xy_idx(WIDTH - 1, y)] = TileType::Wall;
        }

        let mut rng = RandomNumberGenerator::new();

        for _ in 0..400 {
            let x = rng.range(1, WIDTH - 1);
            let y = rng.range(1, HEIGHT - 1);
            let idx = xy_idx(x, y);
            if state.player_position != idx {
                state.map[idx] = TileType::Wall;
            }
        }

        state
    }

    pub fn move_player(&mut self, delta_x: i32, delta_y: i32) {
        let current_position = idx_xy(self.player_position);
        let new_position = (current_position.0 + delta_x, current_position.1 + delta_y);
        let new_idx = xy_idx(new_position.0, new_position.1);
        if self.map[new_idx] == TileType::Floor {
            self.player_position = new_idx;
        }
    }
}

// Implement the game loop
impl GameState for State {
    #[allow(non_snake_case)]
    fn tick(&mut self, ctx: &mut Rltk) {
        let mut draw_batch = DrawBatch::new();
        match ctx.key {
            None => {} // Nothing happened
            Some(key) => {
                // A key is pressed or held
                match key {
                    // Numpad
                    VirtualKeyCode::Numpad8 => self.move_player(0, -1),
                    VirtualKeyCode::Numpad4 => self.move_player(-1, 0),
                    VirtualKeyCode::Numpad6 => self.move_player(1, 0),
                    VirtualKeyCode::Numpad2 => self.move_player(0, 1),

                    // Numpad diagonals
                    VirtualKeyCode::Numpad7 => self.move_player(-1, -1),
                    VirtualKeyCode::Numpad9 => self.move_player(1, -1),
                    VirtualKeyCode::Numpad1 => self.move_player(-1, 1),
                    VirtualKeyCode::Numpad3 => self.move_player(1, 1),

                    // Cursors
                    VirtualKeyCode::Up => self.move_player(0, -1),
                    VirtualKeyCode::Down => self.move_player(0, 1),
                    VirtualKeyCode::Left => self.move_player(-1, 0),
                    VirtualKeyCode::Right => self.move_player(1, 0),

                    _ => {} // Ignore all the other possibilities
                }
            }
        }

        // Set all tiles to not visible
        for v in &mut self.visible {
            *v = false;
        }

        // Obtain the player's visible tile set, and apply it
        let player_position = self.index_to_point2d(self.player_position);
        let fov = rltk::field_of_view_set(player_position, 8, self);

        // Note that the steps above would generally not be run every frame!
        for idx in &fov {
            self.visible[xy_idx(idx.x, idx.y)] = true;
        }

        // Clear the screen
        draw_batch.target(0);
        draw_batch.cls();

        // Iterate the map array, incrementing coordinates as we go.
        let mut y = 0;
        let mut x = 0;
        for (i, tile) in self.map.iter().enumerate() {
            // Render a tile depending upon the tile type; now we check visibility as well!
            let mut fg = RGB::from_f32(1.0, 1.0, 1.0);
            let glyph: u8;

            match tile {
                TileType::Floor => {
                    glyph = 0;
                }
                TileType::Wall => {
                    glyph = 1;
                }
            }
            if !self.visible[i] {
                fg = fg * 0.3;
            } else {
                let distance = 1.0
                    - (DistanceAlg::Pythagoras.distance2d(Point::new(x, y), player_position)
                        as f32
                        / 10.0);
                fg = RGB::from_f32(distance, distance, distance);
            }
            draw_batch.set(
                Point::new(x, y),
                ColorPair::new(fg, RGB::from_f32(0., 0., 0.)),
                glyph,
            );

            // Move the coordinates
            x += 1;
            if x > WIDTH - 1 {
                x = 0;
                y += 1;
            }
        }

        // Render the player @ symbol
        let ppos = idx_xy(self.player_position);
        draw_batch.target(1);
        draw_batch.cls();
        draw_batch.set(
            Point::from_tuple(ppos),
            ColorPair::new(RGB::from_f32(1.0, 1.0, 1.0), RGB::from_f32(0., 0., 0.)),
            2,
        );

        draw_batch.submit(0).expect("Batch error");

        render_draw_buffer(ctx).expect("Render error");
    }
}

// To work with RLTK's algorithm features, we need to implement some the Algorithm2D trait for our map.

// First, default implementations of some we aren't using yet (more on these later!)
impl BaseMap for State {
    // We'll use this one - if its a wall, we can't see through it
    fn is_opaque(&self, idx: usize) -> bool {
        self.map[idx as usize] == TileType::Wall
    }
}

impl Algorithm2D for State {
    fn dimensions(&self) -> Point {
        Point::new(WIDTH, HEIGHT)
    }
}

rltk::embedded_resource!(TILE_FONT, "../resources/example_tiles.png");

fn main() -> RltkError {
    rltk::link_resource!(TILE_FONT, "resources/example_tiles.png");

    // This initialization is a bit more complicated than the previous examples. It uses
    // the "raw" initialization to build a tile-based setup from scatch.
    // new() starts with basically no useful settings
    let context = RltkBuilder::new()
        // We specify the CONSOLE dimensions
        .with_dimensions(WIDTH, HEIGHT)
        // We specify the size of the tiles
        .with_tile_dimensions(16, 16)
        // We give it a window title
        .with_title("RLTK Example 07 - Tiles")
        // We register our embedded "example_tiles.png" as a font.
        .with_font("example_tiles.png", 16, 16)
        // We want a base simple console for the terrain background
        .with_simple_console(WIDTH, HEIGHT, "example_tiles.png")
        // We also want a sparse console atop it to handle moving the character
        .with_sparse_console_no_bg(WIDTH, HEIGHT, "example_tiles.png")
        // And we call the builder function
        .build()?;

    let gs = State::new();
    rltk::main_loop(context, gs)
}
