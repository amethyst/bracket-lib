mod common;
use bracket_color::prelude::*;
use bracket_pathfinding::prelude::*;
use common::*;
use bracket_random::prelude::RandomNumberGenerator;

fn main() {
    let map = Map::new();

    // Perform the search
    let mut search_targets: Vec<(usize, f32)> = Vec::new();
    let mut rng = RandomNumberGenerator::seeded(3);
    for _ in 0..10 {
        loop {
            let n = rng.range(0, map.tiles.len());
            if map.tiles[n] != '#' {
                search_targets.push((n, rng.roll_dice(1, 3) as f32));
                break;
            }
        }
    }

    let flow_map = DijkstraMap::new_weighted(MAP_WIDTH, MAP_HEIGHT, &search_targets, &map, 1024.0);

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
                        RGB::from_u8(
                            0,
                            255 - {
                                let n = flow_map.map[idx] * 12.0;
                                if n > 255.0 {
                                    255.0
                                } else {
                                    n
                                }
                            } as u8,
                            0,
                        )
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
