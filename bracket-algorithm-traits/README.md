# bracket-algorithm-traits

This crate provides traits for use in the `bracket-pathfinding` and `bracket-geometry` crates. It is part of the overall `bracket-lib` system.

Using a trait interface means that `bracket-lib` doesn't have to know or care about how you store your data, and can still provide useful geometry and path-finding functions. Defaults are provided, allowing you to get up and running quickly.

Everything is exported via `bracket_algorithm_traits::prelude`.

## Map Indexing

A truly minimal implementation (replace `TestMap` with your map structure):

```rust
struct TestMap{};
impl BaseMap for TestMap {}
impl Algorithm2D for TestMap{
    fn dimensions(&self) -> Point {
        Point::new(2, 2)
    }
}
```

This is sufficient to provide the following services:

* `in_bounds(Point)`: is a point within the dimensions of the map?
* `point2d_to_index(Point) -> usize`: provides an array index for a point, within the map dimensions. It assumes that your array has striding by column.
* `index_to_point2d(usize) -> Point` : provides an `x/y` coordinate for a given array index, assuming the same striding.

If you don't like the default implementations, feel free to override them.

There is an equivalent `Algorithm3D` for 3D grid-based maps (substitute `Point3D` for `Point`).

## Map Traversal

The `BaseMap` trait helps you define the map. If you want to use path-finding, you need to implement the `is_opaque` function:

```rust
impl BaseMap for MyMap {
    fn is_opaque(&self, _idx: usize) -> bool {
        false
    }
}
```

To support path-finding, you need to implement two more functions:

```rust
impl BaseMap for MyMap {
    fn get_available_exits(&self, idx: usize) -> Vec<(usize, f32)> {
        Vec::new()
        // Provide a list of exit indices (you can use point2d_to_index to generate them) for this
        // tile inside the array.
    }

    fn get_pathing_distance(&self, _idx1: usize, _idx2: usize) -> f32 {
        1.0
        // This should be a distance, using whatever heuristic you prefer.
    }
```