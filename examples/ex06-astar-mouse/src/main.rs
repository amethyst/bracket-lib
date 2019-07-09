// This is based on example 3, but adds in highlighting visible tiles.
//
// Comments that duplicate previous examples have been removed for brevity.
//////////////////////////////////////////////////////////////

extern crate rltk;
use rltk::{Rltk, GameState, Console, RGB, BaseMap, Algorithm2D, Point};

extern crate rand;
use crate::rand::Rng;

#[derive(PartialEq, Copy, Clone)]
enum TileType { Wall, Floor }

#[derive(PartialEq, Copy, Clone)]
enum Mode { Waiting, Moving }

struct State {
    map : Vec<TileType>,
    player_position : usize,
    visible : Vec<bool>,
    mode : Mode,
    path : rltk::NavigationPath
}

pub fn xy_idx(x : i32, y : i32) -> usize {
    (y as usize * 80) + x as usize
}

pub fn idx_xy(idx : usize) -> (i32, i32) {
    (idx as i32 % 80, idx as i32 / 80)
}

impl State {
    pub fn new() -> State {
        let mut state = State{
            map : vec![TileType::Floor; 80*50],
            player_position: xy_idx(40, 25),
            visible: vec![false ; 80*50],
            mode : Mode::Waiting,
            path: rltk::NavigationPath::new()
        };

        for x in 0 .. 80 {
            state.map[xy_idx(x, 0)] = TileType::Wall;
            state.map[xy_idx(x, 49)] = TileType::Wall;
        }
        for y in 0 .. 50 {
            state.map[xy_idx(0, y)] = TileType::Wall;
            state.map[xy_idx(79, y)] = TileType::Wall;
        }

        let mut rng = rand::thread_rng();

        for _i in 0..1400 {
            let x = rng.gen_range(1, 79);
            let y = rng.gen_range(1, 49);
            let idx = xy_idx(x, y);
            if state.player_position != idx {
                state.map[idx] = TileType::Wall;
            }
        }

        state
    }

    pub fn is_exit_valid(&self, x:i32, y:i32) -> bool {
        if x < 1 || x > 79 || y < 1 || y > 49 { return false; }
        let idx = (y * 80) + x;
        return self.map[idx as usize] == TileType::Floor;
    }
}

// Implement the game loop
impl GameState for State {
    #[allow(non_snake_case)]
    fn tick(&mut self, ctx : &mut Rltk) {    
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
        ctx.cls();

        // Iterate the map array, incrementing coordinates as we go.
        let mut y = 0;
        let mut x = 0;
        let mut i : usize = 0;
        for tile in self.map.iter() {
            // Render a tile depending upon the tile type; now we check visibility as well!
            let mut fg;
            let mut glyph = ".";

            match tile {
                TileType::Floor => { fg = RGB::from_f32(0.5, 0.5, 0.0); }
                TileType::Wall => { fg = RGB::from_f32(0.0, 1.0, 0.0); glyph = "#"; }
            }
            if !self.visible[i] { fg = fg.to_greyscale(); }
            ctx.print_color(x, y, fg, RGB::from_f32(0., 0., 0.), glyph);

            // Move the coordinates
            x += 1;
            if x > 79 {
                x = 0;
                y += 1;
            }
            i += 1;
        }

        // Either render the proposed path or run along it
        if self.mode == Mode::Waiting {
            // Render a mouse cursor
            let mouse_pos = ctx.mouse_pos();      
            let mouse_idx = self.point2d_to_index(Point::new(mouse_pos.0, mouse_pos.1));
            ctx.print_color(mouse_pos.0, mouse_pos.1, RGB::from_f32(0.0, 1.0, 1.0), RGB::from_f32(0.0, 1.0, 1.0), "X");
            if self.map[mouse_idx as usize] != TileType::Wall {
                let path = rltk::a_star_search(self.player_position as i32, mouse_idx as i32, self);
                if path.success {
                    for loc in path.steps.iter().skip(1) {
                        let x = loc % 80;
                        let y = loc / 80;
                        ctx.print_color(x, y, RGB::from_f32(1., 0., 0.), RGB::from_f32(0., 0., 0.), "*");
                    }

                    if ctx.left_click {
                        self.mode = Mode::Moving;
                        self.path = path.clone();
                    }
                }
            }
        } else {
            self.player_position = self.path.steps[0] as usize;
            self.path.steps.remove(0);
            if self.path.steps.len() == 0 { self.mode = Mode::Waiting; }
        }

        // Render the player @ symbol
        let ppos = idx_xy(self.player_position);
        ctx.print_color(ppos.0, ppos.1, RGB::from_f32(1.0, 1.0, 0.0), RGB::from_f32(0., 0., 0.), "@");
    }

}

impl BaseMap for State {
    fn is_opaque(&self, idx: i32) -> bool { self.map[idx as usize] == TileType::Wall }
    
    fn get_available_exits(&self, idx:i32) -> Vec<(i32, f32)> {
        let mut exits : Vec<(i32, f32)> = Vec::new();
        let x = idx % 80;
        let y = idx / 80;

        // Cardinal directions
        if self.is_exit_valid(x-1, y) { exits.push((idx-1, 1.0)) };
        if self.is_exit_valid(x+1, y) { exits.push((idx+1, 1.0)) };
        if self.is_exit_valid(x, y-1) { exits.push((idx-80, 1.0)) };
        if self.is_exit_valid(x, y+1) { exits.push((idx+80, 1.0)) };

        // Diagonals
        if self.is_exit_valid(x-1, y-1) { exits.push(((idx-80)-1, 1.4)); }
        if self.is_exit_valid(x+1, y-1) { exits.push(((idx-80)+1, 1.4)); }
        if self.is_exit_valid(x-1, y+1) { exits.push(((idx+80)-1, 1.4)); }
        if self.is_exit_valid(x+1, y+1) { exits.push(((idx+80)+1, 1.4)); }

        return exits;
    }
    
    fn get_pathing_distance(&self, idx1:i32, idx2:i32) -> f32 { 
        let p1 = Point::new(idx1 % 80, idx1 / 80);
        let p2 = Point::new(idx2 % 80, idx2 / 80);
        return rltk::distance2d(p1, p2);
     }
}

impl Algorithm2D for State {
    fn point2d_to_index(&self, pt : Point) -> i32 { xy_idx(pt.x, pt.y) as i32 }
    fn index_to_point2d(&self, idx:i32) -> Point { Point::new(idx % 80, idx / 80) }
}

fn main() {
    let context = Rltk::init_simple8x8(80, 50, "RLTK Example 05 - A Star and a Mouse", "../../resources");
    let gs = State::new();
    rltk::main_loop(context, Box::new(gs));
}
