use bevy::prelude::*;
use bracket_bevy::prelude::*;
use bracket_pathfinding::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(BTermBuilder::simple_80x50().with_random_number_generator(true))
        .add_startup_system(setup)
        .add_system(tick)
        .run();
}

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall,
    Floor,
}

#[derive(PartialEq, Copy, Clone)]
enum Mode {
    Waiting,
    Moving,
}

#[derive(Resource)]
struct State {
    map: Vec<TileType>,
    player_position: usize,
    visible: Vec<bool>,
    mode: Mode,
    path: NavigationPath,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

pub fn idx_xy(idx: usize) -> (i32, i32) {
    (idx as i32 % 80, idx as i32 / 80)
}

fn setup(mut commands: Commands, rng: Res<RandomNumbers>) {
    let mut state = State {
        map: vec![TileType::Floor; 80 * 50],
        player_position: xy_idx(40, 25),
        visible: vec![false; 80 * 50],
        mode: Mode::Waiting,
        path: NavigationPath::new(),
    };

    for x in 0..80 {
        state.map[xy_idx(x, 0)] = TileType::Wall;
        state.map[xy_idx(x, 49)] = TileType::Wall;
    }
    for y in 0..50 {
        state.map[xy_idx(0, y)] = TileType::Wall;
        state.map[xy_idx(79, y)] = TileType::Wall;
    }

    for _ in 0..1400 {
        let x = rng.range(1, 79);
        let y = rng.range(1, 49);
        let idx = xy_idx(x, y);
        if state.player_position != idx {
            state.map[idx] = TileType::Wall;
        }
    }

    commands.insert_resource(state);
}

impl State {
    pub fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        if x < 1 || x > 79 || y < 1 || y > 49 {
            return false;
        }
        let idx = (y * 80) + x;
        self.map[idx as usize] == TileType::Floor
    }
}

impl BaseMap for State {
    fn is_opaque(&self, idx: usize) -> bool {
        self.map[idx] == TileType::Wall
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
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

impl Algorithm2D for State {
    fn dimensions(&self) -> Point {
        Point::new(80, 50)
    }
}

fn tick(ctx: Res<BracketContext>, mut state: ResMut<State>, mouse: Res<Input<MouseButton>>) {
    // Let's use batched drawing
    let mut draw_batch = ctx.new_draw_batch();

    // Set all tiles to not visible
    for v in &mut state.visible {
        *v = false;
    }

    // Obtain the player's visible tile set, and apply it
    let player_position = state.index_to_point2d(state.player_position);
    let fov = field_of_view_set(player_position, 8, &*state);

    // Note that the steps above would generally not be run every frame!
    for idx in &fov {
        state.visible[xy_idx(idx.x, idx.y)] = true;
    }

    // Clear the screen
    draw_batch.cls();

    // Iterate the map array, incrementing coordinates as we go.
    let mut y = 0;
    let mut x = 0;
    for (i, tile) in state.map.iter().enumerate() {
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
        if !state.visible[i] {
            fg = fg.to_greyscale();
        }
        draw_batch.print_color(
            Point::new(x, y),
            glyph,
            ColorPair::new(fg, RGB::from_f32(0., 0., 0.)),
        );

        // Move the coordinates
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }

    // Either render the proposed path or run along it
    if state.mode == Mode::Waiting {
        // Render a mouse cursor
        let mouse_pos = ctx.get_mouse_position_for_current_layer();
        let mouse_idx = state.point2d_to_index(mouse_pos);
        draw_batch.print_color(
            mouse_pos,
            "X",
            ColorPair::new(RGB::from_f32(0.0, 1.0, 1.0), RGB::from_f32(0.0, 1.0, 1.0)),
        );
        if state.map[mouse_idx as usize] != TileType::Wall {
            let path = a_star_search(state.player_position, mouse_idx, &*state);
            if path.success {
                for loc in path.steps.iter().skip(1) {
                    let x = (loc % 80) as i32;
                    let y = (loc / 80) as i32;
                    draw_batch.print_color(
                        Point::new(x, y),
                        "*",
                        ColorPair::new(RGB::from_f32(1., 0., 0.), RGB::from_f32(0., 0., 0.)),
                    );
                }

                if mouse.just_pressed(MouseButton::Left) {
                    state.mode = Mode::Moving;
                    state.path = path;
                }
            }
        }
    } else {
        state.player_position = state.path.steps[0] as usize;
        state.path.steps.remove(0);
        if state.path.steps.is_empty() {
            state.mode = Mode::Waiting;
        }
    }

    // Render the player @ symbol
    let ppos = idx_xy(state.player_position);
    draw_batch.print_color(
        Point::from_tuple(ppos),
        "@",
        ColorPair::new(RGB::from_f32(1.0, 1.0, 0.0), RGB::from_f32(0., 0., 0.)),
    );

    // Submit the rendering
    ctx.submit_batch(0, draw_batch);
    // Note: automatic batch submission is enabled, so you don't need to
    // apply the batches
}
