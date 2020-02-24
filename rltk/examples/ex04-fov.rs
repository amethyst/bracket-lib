#![warn(clippy::pedantic)]
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

// Just like example 3, but we're adding an additional vector: visible
struct State {
    map: Vec<TileType>,
    player_position: usize,
    visible: Vec<bool>,
}

impl State {
    pub fn new() -> State {
        // Same as example 3, but we've added the visible tiles
        let mut state = State {
            map: vec![TileType::Floor; 80 * 50],
            player_position: (25 * 80) + 40, // Equivalent to point2d_to_index(40, 25) but we haven't initialized it yet
            visible: vec![false; 80 * 50],
        };

        for x in 0..80 {
            let wall1_pos = state.point2d_to_index(Point::new(x, 0));
            let wall2_pos = state.point2d_to_index(Point::new(x, 49));
            state.map[wall1_pos] = TileType::Wall;
            state.map[wall2_pos] = TileType::Wall;
        }
        for y in 0..50 {
            let wall1_pos = state.point2d_to_index(Point::new(0, y));
            let wall2_pos = state.point2d_to_index(Point::new(79, y));
            state.map[wall1_pos] = TileType::Wall;
            state.map[wall2_pos] = TileType::Wall;
        }

        let mut rng = RandomNumberGenerator::new();

        for _ in 0..400 {
            let x = rng.range(1, 79);
            let y = rng.range(1, 49);
            let idx = state.point2d_to_index(Point::new(x, y));
            if state.player_position != idx {
                state.map[idx] = TileType::Wall;
            }
        }

        state
    }

    pub fn move_player(&mut self, delta: Point) {
        let current_position = self.index_to_point2d(self.player_position);
        let new_position = current_position + delta;
        let new_idx = self.point2d_to_index(new_position);
        if self.map[new_idx] == TileType::Floor {
            self.player_position = new_idx;
        }
    }
}

// Implement the game loop
impl GameState for State {
    #[allow(non_snake_case)]
    fn tick(&mut self, ctx: &mut Rltk) {
        match ctx.key {
            None => {} // Nothing happened
            Some(key) => {
                // A key is pressed or held
                match key {
                    // Numpad
                    VirtualKeyCode::Numpad8 => self.move_player(Point::new(0, -1)),
                    VirtualKeyCode::Numpad4 => self.move_player(Point::new(-1, 0)),
                    VirtualKeyCode::Numpad6 => self.move_player(Point::new(1, 0)),
                    VirtualKeyCode::Numpad2 => self.move_player(Point::new(0, 1)),

                    // Numpad diagonals
                    VirtualKeyCode::Numpad7 => self.move_player(Point::new(-1, -1)),
                    VirtualKeyCode::Numpad9 => self.move_player(Point::new(1, -1)),
                    VirtualKeyCode::Numpad1 => self.move_player(Point::new(-1, 1)),
                    VirtualKeyCode::Numpad3 => self.move_player(Point::new(1, 1)),

                    // Cursors
                    VirtualKeyCode::Up => self.move_player(Point::new(0, -1)),
                    VirtualKeyCode::Down => self.move_player(Point::new(0, 1)),
                    VirtualKeyCode::Left => self.move_player(Point::new(-1, 0)),
                    VirtualKeyCode::Right => self.move_player(Point::new(1, 0)),

                    _ => {} // Ignore all the other possibilities
                }
            }
        }

        // Set all tiles to not visible
        for v in self.visible.iter_mut() {
            *v = false;
        }

        // Obtain the player's visible tile set, and apply it
        let player_position = self.index_to_point2d(self.player_position);
        let fov = rltk::field_of_view_set(player_position, 8, self);

        // Note that the steps above would generally not be run every frame!
        for idx in fov.iter() {
            let point = self.point2d_to_index(*idx);
            self.visible[point] = true;
        }

        // Clear the screen
        ctx.cls();

        // Iterate the map array, incrementing coordinates as we go.
        let mut y = 0;
        let mut x = 0;
        for (i, tile) in self.map.iter().enumerate() {
            // Render a tile depending upon the tile type; now we check visibility as well!
            let mut fg;
            let mut glyph = ".";

            match tile {
                TileType::Floor => {
                    fg = RGB::from_f32(0.5, 0.5, 0.0);
                }
                TileType::Wall => {
                    fg = RGB::from_f32(0.0, 1.0, 0.0);
                    glyph = "#";
                }
            }
            if !self.visible[i] {
                fg = fg.to_greyscale();
            }
            ctx.print_color(x, y, fg, RGB::from_f32(0., 0., 0.), glyph);

            // Move the coordinates
            x += 1;
            if x > 79 {
                x = 0;
                y += 1;
            }
        }

        // Render the player @ symbol
        let ppos = self.index_to_point2d(self.player_position);
        ctx.print_color(
            ppos.x,
            ppos.y,
            RGB::from_f32(1.0, 1.0, 0.0),
            RGB::from_f32(0., 0., 0.),
            "@",
        );
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
        Point::new(80, 50)
    }
}

fn main() -> RltkError {
    let context = RltkBuilder::simple80x50()
        .with_title("RLTK Example 4 - Field-Of-View")
        .build()?;
    let gs = State::new();
    rltk::main_loop(context, gs)
}
