mod common;
use common::*;
use bracket_pathfinding::prelude::*;
use bracket_color::prelude::*;

fn main() {
    let mut map = Map::new();

    // Perform the search
    let path = a_star_search(
        map.point2d_to_index(START_POINT),
        map.point2d_to_index(END_POINT),
        &map
    );
    if path.success {
        for loc in &path.steps {
            map.tiles[*loc] = '*';
        }
    }

    // Draw the result
    for y in 0..MAP_HEIGHT {
        let idx = map.point2d_to_index(Point::new(0, y));
        for x in 0..MAP_WIDTH {
            match map.tiles[idx+x] {
                '#' => print_color(RGB::named(YELLOW), "#"),
                '*' => print_color(RGB::named(RED), "*"),
                _ => print_color(RGB::named(CHOCOLATE), "."),
            }
        }
        print_color(RGB::named(WHITE), "\n");
    }
    flush_console();
}