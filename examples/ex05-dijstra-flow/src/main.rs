// Generates a map with FoV like example 4, and replaces human input with path-finding
// to reveal the whole map. Uses RLTK's Dijkstra flow maps functionality.
//
// Comments that duplicate previous examples have been removed for brevity.
//////////////////////////////////////////////////////////////

extern crate rltk;
use rltk::{Rltk, GameState, Console, RGB, BaseMap, Algorithm2D, Point, DijkstraMap};

extern crate rand;
use crate::rand::Rng;

use std::f32::MAX;

#[derive(PartialEq, Copy, Clone)]
enum TileType { Wall, Floor }

// We've added a new field, revealed. If a tile isn't revealed, we have never seen it.
struct State {
    map : Vec<TileType>,
    player_position : usize,
    visible : Vec<bool>,
    revealed : Vec<bool>
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
            map : Vec::new(),
            player_position: xy_idx(40, 25),
            visible: Vec::new(),
            revealed: Vec::new()
        };

        for _i in 0 .. 80*50 {
            state.map.push(TileType::Floor);
            state.visible.push(false);
            state.revealed.push(false);
        }

        for x in 0 .. 80 {
            state.map[xy_idx(x, 0)] = TileType::Wall;
            state.map[xy_idx(x, 49)] = TileType::Wall;
        }
        for y in 0 .. 50 {
            state.map[xy_idx(0, y)] = TileType::Wall;
            state.map[xy_idx(79, y)] = TileType::Wall;
        }

        let mut rng = rand::thread_rng();

        for _i in 0..1600 {
            let x = rng.gen_range(1, 79);
            let y = rng.gen_range(1, 49);
            let idx = xy_idx(x, y);
            if state.player_position != idx {
                state.map[idx] = TileType::Wall;
            }
        }

        state
    }

    #[inline(always)]
    pub fn is_exit_valid(&self, x:i32, y:i32) -> bool {
        if x < 1 || x > 80-1 || y < 1 || y > 50-1 { return false; }
        let idx = ((y * 80) + x) as usize;
        return self.map[idx] == TileType::Floor;
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
            let mapidx = xy_idx(idx.x, idx.y);
            self.visible[mapidx] = true;
            self.revealed[mapidx] = true;
        }

        // Use RLTK's DijkstraMap to build a flow map for finding unrevealed areas.
        let mut search_targets : Vec<i32> = Vec::with_capacity(80*50);
        let mut added = 0;
        for i in 0 .. 80*50 {
            if self.revealed[i]==false && self.map[i] == TileType::Floor {
                search_targets.push(i as i32);
                added += 1;
            }
        }
        let mut anything_left = added > 0;
        let flow_map = DijkstraMap::new(80, 50, &search_targets, self, 2048.0);        
        if !(flow_map.map[self.player_position] < MAX) { anything_left = false; }
        if anything_left {
            // Now we use the flow map to move
            // If its MAX, then there's nowhere to go.
            let destination = flow_map.find_lowest_exit(self.player_position as i32, self);
            match destination {
                None => {}
                Some(idx) => { self.player_position = idx as usize; }
            }            
        }

        // Clear the screen
        ctx.cls();

        // Iterate the map array, incrementing coordinates as we go.
        let mut y = 0;
        let mut x = 0;
        let mut i : usize = 0;
        for tile in self.map.iter() {
            // New test: only render if its revealed
            if self.revealed[i] {
                // Render a tile depending upon the tile type; now we check visibility as well!
                let mut fg;
                let mut glyph = ".";
                let bg;

                let distance = flow_map.map[i];
                if distance == MAX {
                    bg = RGB::from_f32(0.0, 0.0, 0.0);
                } else {
                    bg = RGB::from_f32(0.0, 0.0, 1.0 - (flow_map.map[i] / 256.0));
                }

                match tile {
                    TileType::Floor => { fg = RGB::from_f32(0.5, 0.5, 0.0); }
                    TileType::Wall => { fg = RGB::from_f32(0.0, 1.0, 0.0); glyph = "#"; }
                }
                if !self.visible[i] { fg = fg.to_greyscale(); }
                ctx.print_color(x, y, fg, bg, glyph);
            }

            // Move the coordinates
            x += 1;
            if x > 79 {
                x = 0;
                y += 1;
            }
            i += 1;
        }

        // Render the player @ symbol
        let ppos = idx_xy(self.player_position);
        ctx.print_color(ppos.0, ppos.1, RGB::from_f32(1.0, 1.0, 0.0), RGB::from_f32(0., 0., 0.), "@");

        if !anything_left {
            ctx.print_color(30, 25, RGB::from_f32(1.0, 1.0, 0.0), RGB::from_f32(0., 0., 0.), "Search Complete");
        }
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
    let mut context = Rltk::init_simple8x8(80, 50, "RLTK Example 05 - Dijstra Flow Maps", "../../resources");
    let mut gs = State::new();
    context.main_loop(&mut gs);
}
