// This is based on example 3, but adds in highlighting visible tiles.
//
// Comments that duplicate previous examples have been removed for brevity.
//////////////////////////////////////////////////////////////

extern crate rltk;
use rltk::{Rltk, GameState, Console, RGB, VirtualKeyCode, BaseMap, Algorithm2D, Point, DistanceAlg};

extern crate rand;
use crate::rand::Rng;

#[derive(PartialEq, Copy, Clone)]
enum TileType { Wall, Floor }

const WIDTH : i32 = 40;
const HEIGHT : i32 = 25;

// Just like example 3, but we're adding an additional vector: visible
struct State {
    map : Vec<TileType>,
    player_position : usize,
    visible : Vec<bool>
}

pub fn xy_idx(x : i32, y : i32) -> usize {
    (y as usize * WIDTH as usize) + x as usize
}

pub fn idx_xy(idx : usize) -> (i32, i32) {
    (idx as i32 % WIDTH, idx as i32 / WIDTH)
}

impl State {
    pub fn new() -> State {
        // Same as example 3, but we've added the visible tiles
        let mut state = State{
            map : Vec::new(),
            player_position: xy_idx(WIDTH/2, HEIGHT/2),
            visible: Vec::new()
        };

        // We also want to add visible data
        for _i in 0 .. WIDTH*HEIGHT {
            state.map.push(TileType::Floor);
            state.visible.push(false);
        }

        for x in 0 .. WIDTH {
            state.map[xy_idx(x, 0)] = TileType::Wall;
            state.map[xy_idx(x, HEIGHT-1)] = TileType::Wall;
        }
        for y in 0 .. HEIGHT {
            state.map[xy_idx(0, y)] = TileType::Wall;
            state.map[xy_idx(WIDTH-1, y)] = TileType::Wall;
        }

        let mut rng = rand::thread_rng();

        for _i in 0..400 {
            let x = rng.gen_range(1, WIDTH-1);
            let y = rng.gen_range(1, HEIGHT-1);
            let idx = xy_idx(x, y);
            if state.player_position != idx {
                state.map[idx] = TileType::Wall;
            }
        }

        state
    }

    pub fn move_player(&mut self, delta_x : i32, delta_y : i32) {
        let current_position = idx_xy(self.player_position);
        let new_position = ( current_position.0 + delta_x, current_position.1 + delta_y );
        let new_idx = xy_idx(new_position.0, new_position.1);
        if self.map[new_idx] == TileType::Floor {
            self.player_position = new_idx;
        }
    }
}

// Implement the game loop
impl GameState for State {
    #[allow(non_snake_case)]
    fn tick(&mut self, ctx : &mut Rltk) {
        match ctx.key {
            None => {} // Nothing happened
            Some(key) => { // A key is pressed or held
                match key {
                    // Numpad
                    VirtualKeyCode::Numpad8 => { self.move_player(0, -1); }
                    VirtualKeyCode::Numpad4 => { self.move_player(-1, 0); }
                    VirtualKeyCode::Numpad6 => { self.move_player(1, 0);  }
                    VirtualKeyCode::Numpad2 => { self.move_player(0, 1); }

                    // Numpad diagonals
                    VirtualKeyCode::Numpad7 => { self.move_player(-1, -1); }
                    VirtualKeyCode::Numpad9 => { self.move_player(1, -1); }
                    VirtualKeyCode::Numpad1 => { self.move_player(-1, 1); }
                    VirtualKeyCode::Numpad3 => { self.move_player(1, 1); }

                    // Cursors
                    VirtualKeyCode::Up => { self.move_player(0, -1); }
                    VirtualKeyCode::Down => { self.move_player(0, 1); }
                    VirtualKeyCode::Left => { self.move_player(-1, 0); }
                    VirtualKeyCode::Right => { self.move_player(1, 0); }

                    _ => {} // Ignore all the other possibilities
                }
            }
        }

        // Set all tiles to not visible
        for v in self.visible.iter_mut() { *v = false; }

        // Obtain the player's visible tile set, and apply it
        let player_position = self.index_to_point2d(self.player_position as i32);
        let fov = rltk::field_of_view(player_position, 8, self);

        // Note that the steps above would generally not be run every frame!
        for idx in fov.iter() {
            self.visible[xy_idx(idx.x, idx.y)] = true;
        }

        // Clear the screen
        ctx.set_active_console(0);
        ctx.cls();

        // Iterate the map array, incrementing coordinates as we go.
        let mut y = 0;
        let mut x = 0;
        let mut i : usize = 0;
        for tile in self.map.iter() {
            // Render a tile depending upon the tile type; now we check visibility as well!
            let mut fg = RGB::from_f32(1.0, 1.0, 1.0);
            let glyph : u8;

            match tile {
                TileType::Floor => { glyph = 0; }
                TileType::Wall => { glyph = 1; }
            }
            if !self.visible[i] { fg =  fg * 0.3; } else {
                let distance = 1.0 - (rltk::distance2d(DistanceAlg::Pythagoras, Point::new(x,y), player_position) as f32 / 10.0);
                fg = RGB::from_f32(distance, distance, distance);
            }
            ctx.set(x, y, fg, RGB::from_f32(0., 0., 0.), glyph);

            // Move the coordinates
            x += 1;
            if x > WIDTH-1 {
                x = 0;
                y += 1;
            }
            i += 1;
        }

        // Render the player @ symbol
        let ppos = idx_xy(self.player_position);
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set(ppos.0, ppos.1, RGB::from_f32(1.0, 1.0, 1.0), RGB::from_f32(0., 0., 0.), 2);
    }
}

// To work with RLTK's algorithm features, we need to implement some the Algorithm2D trait for our map.

// First, default implementations of some we aren't using yet (more on these later!)
impl BaseMap for State {
    // We'll use this one - if its a wall, we can't see through it
    fn is_opaque(&self, idx: i32) -> bool { self.map[idx as usize] == TileType::Wall }
    fn get_available_exits(&self, _idx:i32) -> Vec<(i32, f32)> { Vec::new() }
    fn get_pathing_distance(&self, _idx1:i32, _idx2:i32) -> f32 { 0.0 }
}

impl Algorithm2D for State {
    // Point translations that we need for field-of-view. Fortunately, we've already written them!
    fn point2d_to_index(&self, pt : Point) -> i32 { xy_idx(pt.x, pt.y) as i32 }
    fn index_to_point2d(&self, idx:i32) -> Point { Point::new(idx % WIDTH, idx / WIDTH) }
}

fn main() {
    let mut context = Rltk::init_raw(WIDTH as u32 * 16, HEIGHT as u32 * 16, "RLTK Example 07 - Tiles", "resources");
    let font = context.register_font(rltk::Font::load("resources/example_tiles.jpg", (16,16)));
    context.register_console(rltk::SimpleConsole::init(WIDTH as u32, HEIGHT as u32, &context.gl), font);
    context.register_console_no_bg(rltk::SparseConsole::init(WIDTH as u32, HEIGHT as u32, &context.gl), font);
    let gs = State::new();
    rltk::main_loop(context, Box::new(gs));
}
