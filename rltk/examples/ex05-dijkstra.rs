#![warn(clippy::pedantic)]
// Generates a map with FoV like example 4, and replaces human input with path-finding
// to reveal the whole map. Uses RLTK's Dijkstra flow maps functionality.
//
// You may want to run this in release mode.
//
// Comments that duplicate previous examples have been removed for brevity.
//////////////////////////////////////////////////////////////

use rltk::prelude::*;
rltk::add_wasm_support!();

use std::f32::MAX;

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall,
    Floor,
}

// We're breaking the map structure out into its own, so we can hold a copy of both it
// and various flow-mapping stuff without annoying the borrow checker.
struct Map {
    tiles: Vec<TileType>,
    visible: Vec<bool>,
    revealed: Vec<bool>,
}

// We've added a new field, revealed. If a tile isn't revealed, we have never seen it.
struct State {
    map: Map,
    player_position: usize,
    search_targets: Vec<usize>,
    flow_map: DijkstraMap,
}

impl State {
    pub fn new() -> State {
        let mut state = State {
            map: Map {
                tiles: vec![TileType::Floor; 80 * 50],
                visible: vec![false; 80 * 50],
                revealed: vec![false; 80 * 50],
            },
            player_position: (40 * 80) + 25, // Equivalent to point2d_to_index
            search_targets: Vec::with_capacity(80 * 50),
            // Here we create an empty placeholder for the flow map; this way we don't allocate it repeatedly
            flow_map: DijkstraMap::new_empty(80, 50, 2048.0),
        };

        for x in 0..80 {
            let wall1_pos = state.map.point2d_to_index(Point::new(x, 0));
            let wall2_pos = state.map.point2d_to_index(Point::new(x, 49));
            state.map.tiles[wall1_pos] = TileType::Wall;
            state.map.tiles[wall2_pos] = TileType::Wall;
        }
        for y in 0..50 {
            let wall1_pos = state.map.point2d_to_index(Point::new(0, y));
            let wall2_pos = state.map.point2d_to_index(Point::new(79, y));
            state.map.tiles[wall1_pos] = TileType::Wall;
            state.map.tiles[wall2_pos] = TileType::Wall;
        }

        let mut rng = RandomNumberGenerator::new();

        // Lets add more walls to make it harder
        for _ in 0..1600 {
            let x = rng.range(1, 79);
            let y = rng.range(1, 49);
            let idx = state.map.point2d_to_index(Point::new(x, y));
            if state.player_position != idx {
                state.map.tiles[idx] = TileType::Wall;
            }
        }

        // Populate the search targets
        // Since the map doesn't change, we'll do this once. It's a list of indices of tiles that
        // are not walls, and aren't revealed
        for i in 0..80 * 50 {
            if !state.map.revealed[i] && state.map.tiles[i] == TileType::Floor {
                state.search_targets.push(i);
            }
        }

        state
    }
}

// Implement the game loop
impl GameState for State {
    #[allow(non_snake_case)]
    fn tick(&mut self, ctx: &mut Rltk) {
        // Set all tiles to not visible
        for v in &mut self.map.visible {
            *v = false;
        }

        // Obtain the player's visible tile set, and apply it
        let player_position = self.map.index_to_point2d(self.player_position);
        let fov = rltk::field_of_view_set(player_position, 8, &self.map);

        // Note that the steps above would generally not be run every frame!
        for idx in &fov {
            let mapidx = self.map.point2d_to_index(*idx);
            self.map.visible[mapidx] = true;
            if !self.map.revealed[mapidx] {
                self.map.revealed[mapidx] = true;
                // We remove all elements of the search targets list that are now revealed.
                self.search_targets.retain(|a| a != &mapidx);
            }
        }

        // Use RLTK's DijkstraMap to build a flow map for finding unrevealed areas.
        let mut anything_left = true;
        DijkstraMap::clear(&mut self.flow_map);
        DijkstraMap::build(&mut self.flow_map, &self.search_targets, &self.map);
        if self.flow_map.map[self.player_position] >= MAX {
            anything_left = false;
        }
        if anything_left {
            // Now we use the flow map to move
            // If its MAX, then there's nowhere to go.
            let destination =
                DijkstraMap::find_lowest_exit(&self.flow_map, self.player_position, &self.map);
            match destination {
                None => {}
                Some(idx) => {
                    self.player_position = idx as usize;
                }
            }
        }

        // Clear the screen
        ctx.cls();

        // Iterate the map array, incrementing coordinates as we go.
        let mut y = 0;
        let mut x = 0;
        for (i, tile) in self.map.tiles.iter().enumerate() {
            // New test: only render if its revealed
            let bg;
            let distance = self.flow_map.map[i];
            if distance == MAX {
                bg = RGB::from_f32(0.0, 0.0, 1.0);
            } else {
                bg = RGB::from_f32(0.0, self.flow_map.map[i] / 32.0, 0.0);
            }

            if self.map.revealed[i] {
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
                if !self.map.visible[i] {
                    fg = fg.to_greyscale();
                }
                ctx.print_color(x, y, fg, bg, glyph);
            } else {
                ctx.print_color(x, y, RGB::from_f32(0.5, 0.5, 0.0), bg, " ");
            }

            // Move the coordinates
            x += 1;
            if x > 79 {
                x = 0;
                y += 1;
            }
        }

        // Render the player @ symbol
        let ppos = self.map.index_to_point2d(self.player_position);
        ctx.print_color(
            ppos.x,
            ppos.y,
            RGB::from_f32(1.0, 1.0, 0.0),
            RGB::from_f32(0., 0., 0.),
            "@",
        );

        if !anything_left {
            ctx.print_color(
                30,
                25,
                RGB::from_f32(1.0, 1.0, 1.0),
                RGB::from_f32(1., 0., 0.),
                "Search Complete",
            );
        } else {
            ctx.print_color(
                0,
                0,
                RGB::from_f32(1.0, 1.0, 0.0),
                RGB::from_f32(0., 0., 0.),
                &format!("{} Targets Remain", self.search_targets.len()),
            );
            ctx.print_color(
                0,
                1,
                RGB::from_f32(1.0, 1.0, 0.0),
                RGB::from_f32(0., 0., 0.),
                &format!("{} FPS", ctx.fps),
            );
        }
    }
}

// Implementing a function called is_exit_valid to help our available exists
// call.
impl Map {
    #[inline(always)]
    pub fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        if x < 1 || x > 79 || y < 1 || y > 49 {
            return false;
        }
        let idx = ((y * 80) + x) as usize;
        self.tiles[idx] == TileType::Floor
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize] == TileType::Wall
    }

    fn get_available_exits(&self, idx: usize) -> Vec<(usize, f32)> {
        let mut exits: Vec<(usize, f32)> = Vec::with_capacity(8);
        let x = (idx % 80) as i32;
        let y = (idx / 80) as i32;

        // Cardinal directions
        if self.is_exit_valid(x - 1, y) {
            exits.push((idx - 1, 1.0))
        };
        if self.is_exit_valid(x + 1, y) {
            exits.push((idx + 1, 1.0))
        };
        if self.is_exit_valid(x, y - 1) {
            exits.push((idx - 80, 1.0))
        };
        if self.is_exit_valid(x, y + 1) {
            exits.push((idx + 80, 1.0))
        };

        // Diagonals
        if self.is_exit_valid(x - 1, y - 1) {
            exits.push(((idx - 80) - 1, 1.4));
        }
        if self.is_exit_valid(x + 1, y - 1) {
            exits.push(((idx - 80) + 1, 1.4));
        }
        if self.is_exit_valid(x - 1, y + 1) {
            exits.push(((idx + 80) - 1, 1.4));
        }
        if self.is_exit_valid(x + 1, y + 1) {
            exits.push(((idx + 80) + 1, 1.4));
        }

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let p1 = Point::new(idx1 % 80, idx1 / 80);
        let p2 = Point::new(idx2 % 80, idx2 / 80);
        DistanceAlg::Pythagoras.distance2d(p1, p2)
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(80, 50)
    }
}

fn main() -> RltkError {
    let context = RltkBuilder::simple80x50()
        .with_title("RLTK Example 5 - Dijkstra")
        .build()?;
    let gs = State::new();
    rltk::main_loop(context, gs)
}
