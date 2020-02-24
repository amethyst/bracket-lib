// This the first roguelike-ish example - a walking @. We build a very simple map,
// and you can use the cursor keys to move around a world.
//
// Comments that duplicate previous examples have been removed for brevity.
//////////////////////////////////////////////////////////////

rltk::add_wasm_support!();
use rltk::prelude::*;

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall,
    Floor,
    Ramp,
    RampDown,
    OpenSpace,
}

#[derive(PartialEq, Copy, Clone)]
enum Mode {
    Waiting,
    Moving,
}

struct State {
    map: Vec<TileType>,
    player_position: usize,
    enable_dive: bool,
    mode: Mode,
    path: rltk::NavigationPath,
}

const WIDTH: i32 = 80;
const HEIGHT: i32 = 50;
const DEPTH: i32 = 128;
const LAYER_SIZE: usize = (WIDTH * HEIGHT) as usize;
const NUM_TILES: usize = LAYER_SIZE * DEPTH as usize;

pub fn xyz_idx(x: i32, y: i32, z: i32) -> usize {
    (LAYER_SIZE * z as usize) + (y as usize * WIDTH as usize) + x as usize
}

pub fn idx_xyz(idx: usize) -> (i32, i32, i32) {
    let z = (idx / LAYER_SIZE) as i32;
    let y = ((idx as i32 - (z * LAYER_SIZE as i32) as i32) / WIDTH as i32) as i32;
    let x = ((idx as i32 - (z * LAYER_SIZE as i32) as i32) % WIDTH as i32) as i32;

    (x, y, z)
}

impl State {
    pub fn new() -> State {
        let mut state = State {
            map: vec![TileType::OpenSpace; NUM_TILES],
            player_position: xyz_idx(40, 19, 127),
            enable_dive: true,
            mode: Mode::Waiting,
            path: rltk::NavigationPath::new(),
        };

        // Now we noise-generate a world.
        // There's nothing special about these numbers; they were picked by
        // playing around until I liked the map!
        let mut noise = FastNoise::seeded(2);
        noise.set_noise_type(NoiseType::SimplexFractal);
        noise.set_fractal_type(FractalType::FBM);
        noise.set_fractal_octaves(2);
        noise.set_fractal_gain(0.2);
        noise.set_fractal_lacunarity(1.0);
        noise.set_frequency(2.0);
        for y in 0..50 {
            for x in 0..80 {
                let n = noise.get_noise((x as f32) / 200.0, (y as f32) / 100.0);
                let altitude = (n + 1.0) * 16.0;

                for z in 0..altitude as i32 {
                    let idx = xyz_idx(x, y, z);
                    state.map[idx] = TileType::Wall;
                    state.map[xyz_idx(x, y, z + 1)] = TileType::Floor;
                }
            }
        }

        // We look for floor tiles that can become ramps
        for y in 1..HEIGHT - 1 {
            for x in 1..WIDTH - 1 {
                for z in 0..DEPTH - 1 {
                    let idx = xyz_idx(x, y, z);
                    if state.map[idx] == TileType::Floor {
                        // Look to see if we need to ramp it up
                        if state.map[xyz_idx(x - 1, y, z + 1)] == TileType::Floor
                            || state.map[xyz_idx(x + 1, y, z + 1)] == TileType::Floor
                            || state.map[xyz_idx(x, y - 1, z + 1)] == TileType::Floor
                            || state.map[xyz_idx(x, y + 1, z + 1)] == TileType::Floor
                        {
                            state.map[idx] = TileType::Ramp;
                            state.map[idx + LAYER_SIZE] = TileType::RampDown;
                        }
                    }
                }
            }
        }

        // Fall from the sky until we hit a floor or ramp
        while state.map[state.player_position] != TileType::Floor
            && state.map[state.player_position] != TileType::Ramp
        {
            state.player_position -= LAYER_SIZE;
        }

        // We'll return the state with the short-hand
        state
    }

    pub fn is_exit_valid(&self, x: i32, y: i32, z: i32) -> bool {
        if x < 1 || x > WIDTH - 1 || y < 1 || y > HEIGHT - 1 || z < 1 || z > LAYER_SIZE as i32 - 1 {
            return false;
        }
        let idx = xyz_idx(x, y, z);
        self.map[idx as usize] == TileType::Floor
            || self.map[idx as usize] == TileType::Ramp
            || self.map[idx as usize] == TileType::RampDown
    }
}

// Implement the game loop
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        // Clear the screen
        ctx.cls();

        let ppos = idx_xyz(self.player_position);

        // Iterate the map array, on the current level, rendering tiles. If a tile is open
        // space, "dive" downwards and show layers below darkened.
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let mut idx = xyz_idx(x, y, ppos.2);

                let mut glyph: u8 = rltk::to_cp437('░');
                let mut fg = RGB::from_f32(0.0, 0.5, 0.5);

                match self.map[idx] {
                    TileType::Floor => {
                        glyph = rltk::to_cp437(';');
                        fg = RGB::from_f32(0.0, 1.0, 0.0);
                    }
                    TileType::Wall => {
                        glyph = rltk::to_cp437('█');
                        fg = RGB::from_f32(0.5, 0.5, 0.5);
                    }
                    TileType::Ramp => {
                        glyph = rltk::to_cp437('▲');
                        fg = RGB::from_f32(1., 1., 1.);
                    }
                    TileType::RampDown => {
                        glyph = rltk::to_cp437('▼');
                        fg = RGB::from_f32(1., 1., 1.);
                    }
                    _ => {
                        if self.enable_dive {
                            let mut dive = 1;
                            let mut darken = 0.2;
                            while dive < 10 {
                                idx -= LAYER_SIZE;

                                if idx > 0 && self.map[idx] != TileType::OpenSpace {
                                    match self.map[idx] {
                                        TileType::Floor => {
                                            dive = 100;
                                            glyph = rltk::to_cp437(';');
                                            fg = RGB::from_f32(0.0, 1., 0.0);
                                        }
                                        TileType::Wall => {
                                            dive = 100;
                                            glyph = rltk::to_cp437('█');
                                            fg = RGB::from_f32(0.5, 0.5, 0.5);
                                        }
                                        TileType::Ramp => {
                                            dive = 100;
                                            glyph = rltk::to_cp437('▲');
                                            fg = RGB::from_f32(1., 1., 1.);
                                        }
                                        TileType::RampDown => {
                                            glyph = rltk::to_cp437('▼');
                                            fg = RGB::from_f32(1., 1., 1.);
                                        }
                                        _ => {}
                                    }
                                }

                                dive += 1;
                                darken += 0.1;
                            }
                            if dive > 99 {
                                fg = fg - darken;
                            };
                        }
                    }
                }
                ctx.set(x, y, fg, RGB::from_f32(0., 0., 0.), glyph);
            }
        }

        // Either render a mouse path or traverse it
        if self.mode == Mode::Waiting {
            // Render a mouse cursor
            let mouse_pos = ctx.mouse_pos();
            let mx = mouse_pos.0;
            let my = mouse_pos.1;
            let mut mz = 1;
            for altitude in 1..DEPTH as i32 - 1 {
                let idx = xyz_idx(mx, my, altitude);
                if self.map[idx] == TileType::Floor {
                    mz = altitude;
                }
            }
            let mouse_idx = xyz_idx(mx, my, mz);
            let player_idx = xyz_idx(ppos.0, ppos.1, ppos.2);
            if self.map[mouse_idx as usize] != TileType::Wall
                && self.map[mouse_idx as usize] != TileType::OpenSpace
            {
                let path = rltk::a_star_search(player_idx, mouse_idx, self);
                if path.success {
                    for loc in path.steps.iter().skip(1) {
                        let (x, y, _z) = idx_xyz(*loc as usize);
                        ctx.print_color(
                            x,
                            y,
                            RGB::from_f32(1., 0., 0.),
                            RGB::from_f32(0., 0., 0.),
                            "*",
                        );
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
            if self.path.steps.is_empty() {
                self.mode = Mode::Waiting;
            }
        }

        // Render the player @ symbol
        ctx.print_color(
            ppos.0,
            ppos.1,
            RGB::from_f32(1.0, 1.0, 0.0),
            RGB::from_f32(0., 0., 0.),
            "☺",
        );
    }
}

impl BaseMap for State {
    fn is_opaque(&self, idx: usize) -> bool {
        self.map[idx as usize] == TileType::Wall
    }

    fn get_available_exits(&self, idx: usize) -> Vec<(usize, f32)> {
        let mut exits: Vec<(usize, f32)> = Vec::new();
        let (x, y, z) = idx_xyz(idx as usize);

        // Cardinal directions
        if self.is_exit_valid(x - 1, y, z) {
            exits.push((idx - 1, 1.0))
        };
        if self.is_exit_valid(x + 1, y, z) {
            exits.push((idx + 1, 1.0))
        };
        if self.is_exit_valid(x, y - 1, z) {
            exits.push((idx - WIDTH as usize, 1.0))
        };
        if self.is_exit_valid(x, y + 1, z) {
            exits.push((idx + WIDTH as usize, 1.0))
        };

        // Diagonals
        if self.is_exit_valid(x - 1, y - 1, z) {
            exits.push(((idx - WIDTH as usize) - 1, 1.4));
        }
        if self.is_exit_valid(x + 1, y - 1, z) {
            exits.push(((idx - WIDTH as usize) + 1, 1.4));
        }
        if self.is_exit_valid(x - 1, y + 1, z) {
            exits.push(((idx + WIDTH as usize) - 1, 1.4));
        }
        if self.is_exit_valid(x + 1, y + 1, z) {
            exits.push(((idx + WIDTH as usize) + 1, 1.4));
        }

        // Up and down for ramps
        if self.map[idx as usize] == TileType::Ramp {
            exits.push((idx + LAYER_SIZE, 1.4));
        }
        if self.map[idx as usize] == TileType::RampDown {
            exits.push((idx - LAYER_SIZE, 1.4));
        }

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let pt1 = idx_xyz(idx1);
        let p1 = Point3::new(pt1.0, pt1.1, pt1.2);
        let pt2 = idx_xyz(idx2);
        let p2 = Point3::new(pt2.0, pt2.1, pt2.2);
        DistanceAlg::Pythagoras.distance3d(p1, p2)
    }
}

impl Algorithm3D for State {
    fn point3d_to_index(&self, pt: Point3) -> usize {
        xyz_idx(pt.x, pt.y, pt.z)
    }
    fn index_to_point3d(&self, idx: usize) -> Point3 {
        let i = idx_xyz(idx);
        Point3::new(i.0, i.1, i.2)
    }
}

fn main() -> RltkError {
    let context = RltkBuilder::simple80x50()
        .with_title("RLTK Example 14 - Dwarf Fortress Map Style")
        .build()?;
    let gs = State::new();
    rltk::main_loop(context, gs)
}
