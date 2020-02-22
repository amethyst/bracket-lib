# bracket-pathfinding

This crate is part of the overall `bracket-lib` system, and (in conjunction with `bracket-algorithm-traits`) provides pathfinding functionality. A-Star (A*) and Dijkstra are supported. It also provides field of view (FOV) functionality.

## Trait Implementation

As a minimum, you need to define `get_available_exits` and `get_pathing_distance` (from the `BaseMap` trait) for your map. `is_opaque` is needed for field-of-view. These come from the `bracket-algorithm-traits` crate, but are exposed as part of the `prelude` for convenience. These in turn can benefit from implementing `Algorithm2D` (from the same crate). These serve to provide an interface to your map format for the library: the library tries very hard to not be opinionated about how you should store your map data.

Most of the `Algorithm2D` can be derived by just providing dimensions if you are happy with the simple striding offered by the `bracket-lib`:

```rust
impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(MAP_WIDTH, MAP_HEIGHT)
    }
}
```

For field-of-view, you need to indicate whether a tile blocks visibility or not. For example:

```rust
impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize] == '#' // Change this to your map definition!
    }
}
```

Dijkstra and A-Star need to know what exits are valid from a tile, and the "cost" of moving to that tile (most of the time you can use `1.0`). For example:

```rust
impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize] == '#'
    }

    fn get_available_exits(&self, idx: usize) -> Vec<(usize, f32)> {
        let mut exits: Vec<(usize, f32)> = Vec::new();
        let location = self.index_to_point2d(idx);

        if let Some(idx) = self.valid_exit(location, Point::new(-1, 0)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 0)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, -1)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, 1)) {
            exits.push((idx, 1.0))
        }

        exits
    }
}
```

A-Star also need to know a distance to the objective for a given tile. If you don't provide a useful number for this, it will become a *very* inefficient search. You can vary the behavior of the search by using Pythagoras, Manhattan or other algorithms to specify the distance heuristic. For example:

```rust
impl BaseMap for Map {
    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::PythagorasSquared.distance2d(
            self.index_to_point2d(idx1),
            self.index_to_point2d(idx2)
        )
    }
}
```

## A-Star (A*) Pathing

Bracket-lib includes a high-performance A* system. It uses a binary-heap to optimize open/closed storage. By default, it cancels the search after 65,536 iterations.

Actually performing the A Star search is very simple:

```rust
let path = a_star_search(
        map.point2d_to_index(START_POINT),
        map.point2d_to_index(END_POINT),
        &map
    );
    if path.success {
        // Do something with it! path.steps has the whole path.
    }
```

The example `astar` demonstrates this.

## Dijkstra Mapping

Bracket-lib also includes Dijkstra maps, that can include as many search targets as you want. See [The Incredible Power of Dijkstra Maps](http://www.roguebasin.com/index.php?title=The_Incredible_Power_of_Dijkstra_Maps) for some ideas as to what you can do with this.

To generate a Dijkstra map, you need a vector of target tile indices. You can then make the map:

```rust
let mut search_targets : Vec<usize> = Vec::new();
search_targets.push(map.point2d_to_index(START_POINT));
search_targets.push(map.point2d_to_index(END_POINT));
let flow_map = DijkstraMap::new(MAP_WIDTH, MAP_HEIGHT, &search_targets, &map, 1024.0);
```

Once you have the map, you can access individual distances at `flow_map.map` - or you can use various helper functions such as `find_highest_exist` and `find_lowest_exit` to help with path-finding.

The example `dijkstra` demonstrates this.

## Field of View (2D only for now)

With `is_opaque` defined for your `BaseMap` trait, obtaining a set of all visible tiles is easy:

```rust
let fov = field_of_view_set(START_POINT, 6, &map);
```

You can see this in action with the example `fov`.

## Feature Flags

If you enable the `threaded` feature, some Dijkstra functions will use a multi-threaded algorithm.

## Examples

There are three examples (ignore `common.rs` - it's shared code):

* `astar` (`cargo run --example astar`), demonstrating A-Star pathing across a random map.
* `dijkstra` (`cargo run --example dijkstra`), demonstrating Dijkstra mapping to two targets.
* `fov` (`cargo run --example fov`), demonstrating field-of-view generation.

These use `crossterm` for rendering to your terminal.
