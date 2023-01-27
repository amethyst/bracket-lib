use bevy::prelude::*;
use bracket_bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(BTermBuilder::simple_80x50().with_random_number_generator(true))
        .add_startup_system(build_state)
        .add_system(tick)
        .run();
}

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall,
    Floor,
}

#[derive(Resource)]
struct State {
    map: Vec<TileType>,
    visited: Vec<bool>,
    player_position: usize,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

pub fn idx_xy(idx: usize) -> (i32, i32) {
    (idx as i32 % 80, idx as i32 / 80)
}

pub fn build_state(rng: Res<RandomNumbers>, mut commands: Commands) {
    let state = State::new(&rng);
    commands.insert_resource(state);
}

impl State {
    pub fn new(rng: &RandomNumbers) -> State {
        let mut state = State {
            map: vec![TileType::Floor; 80 * 50],
            player_position: xy_idx(40, 25),
            visited: vec![false; 80 * 50],
        };

        // Make the boundaries walls
        for x in 0..80 {
            state.map[xy_idx(x, 0)] = TileType::Wall;
            state.map[xy_idx(x, 49)] = TileType::Wall;
        }
        for y in 0..50 {
            state.map[xy_idx(0, y)] = TileType::Wall;
            state.map[xy_idx(79, y)] = TileType::Wall;
        }

        for _ in 0..400 {
            // rand provides a gen_range function to get numbers in a range.
            let x = rng.roll_dice(1, 80) - 1;
            let y = rng.roll_dice(1, 50) - 1;
            let idx = xy_idx(x, y);
            // We don't want to add a wall on top of the player
            if state.player_position != idx {
                state.map[idx] = TileType::Wall;
            }
        }

        // We'll return the state with the short-hand
        state
    }

    // Handle player movement. Delta X and Y are the relative move
    // requested by the player. We calculate the new coordinates,
    // and if it is a floor - move the player there.
    pub fn move_player(&mut self, delta_x: i32, delta_y: i32) {
        let current_position = idx_xy(self.player_position);
        let new_position = (current_position.0 + delta_x, current_position.1 + delta_y);
        let new_idx = xy_idx(new_position.0, new_position.1);
        if self.map[new_idx] == TileType::Floor {
            self.player_position = new_idx;
            self.visited[new_idx] = true;
        }
    }
}

fn tick(ctx: Res<BracketContext>, mut state: ResMut<State>, keyboard: Res<Input<KeyCode>>) {
    // Clear the screen
    ctx.cls();

    // Handle keyboard
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

    // Iterate the map array, incrementing coordinates as we go.
    let mut y = 0;
    let mut x = 0;
    for (idx, tile) in state.map.iter().enumerate() {
        // Render a tile depending upon the tile type
        match tile {
            TileType::Floor => {
                ctx.print_color(
                    x,
                    y,
                    ".",
                    RGB::from_f32(0.5, 0.5, 0.5),
                    RGB::from_f32(0., 0., 0.),
                );
            }
            TileType::Wall => {
                ctx.print_color(
                    x,
                    y,
                    "#",
                    RGB::from_f32(0.0, 1.0, 0.0),
                    RGB::from_f32(0., 0., 0.),
                );
            }
        }
        if state.visited[idx] {
            ctx.set_bg((idx % 80) as i32, (idx / 80) as i32, RGB::named(NAVY));
        }

        // Move the coordinates
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }

    // Render the player @ symbol
    let ppos = idx_xy(state.player_position);
    ctx.print_color(
        ppos.0,
        ppos.1,
        "@",
        RGB::from_f32(1.0, 1.0, 0.0),
        RGB::from_f32(0., 0., 0.),
    );
}
