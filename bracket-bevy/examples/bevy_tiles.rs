use bevy::prelude::*;
use bracket_bevy::prelude::*;
use bracket_pathfinding::prelude::*;

const WIDTH: i32 = 40;
const HEIGHT: i32 = 25;

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall,
    Floor,
}

#[derive(Resource)]
struct State {
    map: Vec<TileType>,
    player_position: usize,
    visible: Vec<bool>,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * WIDTH as usize) + x as usize
}

pub fn idx_xy(idx: usize) -> (i32, i32) {
    (idx as i32 % WIDTH as i32, idx as i32 / WIDTH as i32)
}

pub fn setup(mut commands: Commands, rng: Res<RandomNumbers>) {
    let mut state = State {
        map: vec![TileType::Floor; (WIDTH * HEIGHT) as usize],
        player_position: xy_idx(WIDTH as i32 / 2, HEIGHT as i32 / 2),
        visible: vec![false; (WIDTH * HEIGHT) as usize],
    };

    for x in 0..WIDTH as i32 {
        state.map[xy_idx(x, 0)] = TileType::Wall;
        state.map[xy_idx(x, HEIGHT as i32 - 1)] = TileType::Wall;
    }
    for y in 0..HEIGHT as i32 {
        state.map[xy_idx(0, y)] = TileType::Wall;
        state.map[xy_idx(WIDTH as i32 - 1, y)] = TileType::Wall;
    }

    for _ in 0..400 {
        let x = rng.range(1, WIDTH - 1);
        let y = rng.range(1, HEIGHT - 1);
        let idx = xy_idx(x as i32, y as i32);
        if state.player_position != idx {
            state.map[idx] = TileType::Wall;
        }
    }

    commands.insert_resource(state);
}

impl State {
    pub fn move_player(&mut self, delta_x: i32, delta_y: i32) {
        let current_position = idx_xy(self.player_position);
        let new_position = (current_position.0 + delta_x, current_position.1 + delta_y);
        let new_idx = xy_idx(new_position.0, new_position.1);
        if self.map[new_idx] == TileType::Floor {
            self.player_position = new_idx;
        }
    }
}

impl BaseMap for State {
    fn is_opaque(&self, idx: usize) -> bool {
        self.map[idx as usize] == TileType::Wall
    }
}

impl Algorithm2D for State {
    fn dimensions(&self) -> Point {
        Point::new(WIDTH, HEIGHT)
    }
}

fn main() {
    let bterm = BTermBuilder::empty()
        .with_random_number_generator(true)
        .with_font("example_tiles.png", 16, 16, (16.0, 16.0))
        .with_simple_console(0, WIDTH, HEIGHT)
        .with_sparse_console(0, WIDTH, HEIGHT)
        .with_background(false);

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bterm)
        .add_startup_system(setup)
        .add_system(tick)
        .run();
}

fn tick(ctx: Res<BracketContext>, mut state: ResMut<State>, keyboard: Res<Input<KeyCode>>) {
    let mut draw_batch = ctx.new_draw_batch();
    if keyboard.just_pressed(KeyCode::Left) {
        state.move_player(-1, 0)
    }
    if keyboard.just_pressed(KeyCode::Right) {
        state.move_player(1, 0)
    }
    if keyboard.just_pressed(KeyCode::Up) {
        state.move_player(0, -1)
    }
    if keyboard.just_pressed(KeyCode::Down) {
        state.move_player(0, 1)
    }

    if keyboard.just_pressed(KeyCode::Numpad4) {
        state.move_player(-1, 0)
    }
    if keyboard.just_pressed(KeyCode::Numpad6) {
        state.move_player(1, 0)
    }
    if keyboard.just_pressed(KeyCode::Numpad8) {
        state.move_player(0, -1)
    }
    if keyboard.just_pressed(KeyCode::Numpad2) {
        state.move_player(0, 1)
    }

    if keyboard.just_pressed(KeyCode::Numpad7) {
        state.move_player(-1, -1)
    }
    if keyboard.just_pressed(KeyCode::Numpad9) {
        state.move_player(1, -1)
    }
    if keyboard.just_pressed(KeyCode::Numpad1) {
        state.move_player(-1, 1)
    }
    if keyboard.just_pressed(KeyCode::Numpad3) {
        state.move_player(1, 1)
    }

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
    draw_batch.target(0);
    draw_batch.cls();

    // Iterate the map array, incrementing coordinates as we go.
    let mut y = 0;
    let mut x = 0;
    for (i, tile) in state.map.iter().enumerate() {
        // Render a tile depending upon the tile type; now we check visibility as well!
        let mut fg = RGB::from_f32(1.0, 1.0, 1.0);
        let glyph;

        match tile {
            TileType::Floor => {
                glyph = 0;
            }
            TileType::Wall => {
                glyph = 1;
            }
        }
        if !state.visible[i] {
            fg = fg * 0.3;
        } else {
            let distance = 1.0
                - (DistanceAlg::Pythagoras.distance2d(Point::new(x, y), player_position) as f32
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
    let ppos = idx_xy(state.player_position);
    draw_batch.target(1);
    draw_batch.cls();
    draw_batch.set(
        Point::from_tuple(ppos),
        ColorPair::new(RGB::from_f32(1.0, 1.0, 1.0), RGB::from_f32(0., 0., 0.)),
        2,
    );
    ctx.submit_batch(0, draw_batch);
}
