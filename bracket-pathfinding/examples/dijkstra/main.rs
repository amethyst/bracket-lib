mod common;
use common::*;
use bracket_pathfinding::prelude::*;
use bracket_color::prelude::*;

fn main() {
    let map = Map::new();

    // Perform the search
    let mut search_targets : Vec<usize> = Vec::new();
    search_targets.push(map.point2d_to_index(START_POINT));
    search_targets.push(map.point2d_to_index(END_POINT));
    let flow_map = DijkstraMap::new(MAP_WIDTH, MAP_HEIGHT, &search_targets, &map, 1024.0);

    // Draw the result
    for y in 0..MAP_HEIGHT {
        let base_idx = map.point2d_to_index(Point::new(0, y));
        for x in 0..MAP_WIDTH {
            let idx = base_idx + x;

            let tile = map.tiles[idx];
            let color = match tile {
                '#' => RGB::named(YELLOW),
                _ => {
                    if flow_map.map[idx] < std::f32::MAX {
                        RGB::from_u8(0, 255-(flow_map.map[idx]) as u8, 0)
                    } else {
                        RGB::named(CHOCOLATE)
                    }
                }
            };
            print_color(color, &tile.to_string());
        }
        print_color(RGB::named(WHITE), "\n");
    }
    flush_console();
}