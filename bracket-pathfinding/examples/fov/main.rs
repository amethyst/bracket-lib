mod common;
use common::*;
use bracket_pathfinding::prelude::*;
use bracket_color::prelude::*;

fn main() {
    let map = Map::new();

    // Field of view
    let fov = field_of_view_set(START_POINT, 6, &map);

    // Draw the result
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let pos = Point::new(x,y);
            let idx = map.point2d_to_index(pos);
            let tile;
            let mut color;

            if pos == START_POINT {
                color = RGB::named(GREEN);
                tile = '@';
            } else {
                match map.tiles[idx] {
                    '#' => { tile = '#'; color = RGB::named(YELLOW); }
                    _ => { tile = '.'; color = RGB::named(CHOCOLATE); },
                }
            }

            if !fov.contains(&pos) {
                color = color.to_greyscale();
            }

            print_color(color, &tile.to_string());
        }
        print_color(RGB::named(WHITE), "\n");
    }
    flush_console();
}