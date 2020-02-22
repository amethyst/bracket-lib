# bracket-geometry

This crate provides geometry support for the overall `bracket-lib` system, and is useful stand-alone. It provides some geometric primitives (`Point`, `Point3D`, `Rect`), support functions and distance calculations. It also includes Bresenham's line algorithm, a vector line algorithm, and Bresenham's Circle algorithm.

It uses UltraViolet for fast processing, and includes conversion functions to/from native UltraViolet types.

## Using bracket-geometry

You can include it in your project by adding the following to your `Cargo.toml` file:

```toml
[dependencies]
bracket-geometry = "0.7.0"
```

## Point2D

A basic 2D (integer) point. You can create points with:

* `new`: Create a point with any integer-compatible type (it will be `i32` internally, but uses `TryFrom`).
* `zero`: a `(0,0)` point.
* `from_tuple`: takes a `(i32, i32)` tuple.

It also provides conversion to other types:

* `to_index` for simple array striding (use the `bracket-algorithm-traits` crate for better implementations).
* `to_tuple` converts to an `(i32, i32)` tuple.
* `to_unsigned_tuple` converts to a `(usize, usize)` tuple.
* `to_vec2` converts to an UltraViolet `Vec2`.
* `to_vec2i` converts to an UltraViolet `Vec2i`.

`From` is also implemented for these.

## Point3D

A basic 3D (integer) point. You can create points with:

* `new`: Create a point from any integer-compatible type (x/y/z).
* `from_tuple`: Creates a point from an `(i32,i32,i32)` tuple.

It also provides conversion to UltraViolet's `Vec3` and `Vec3i` types.

## Rectangle (`Rect`)

Represents a rectangle in 2D space. You can create rectangles with:

* `with_size`: provide an X/Y and a width/height.
* `with_exact`: provide all four coordinates.
* `zero`: a zeroed rectangle.

It provides quite a few helper functions:

* `intersect` - does this rectangle intersect with another rectangle?
* `center` - the middle point of the rectangle.
* `point_in_rect` - is a point inside the rectangle (including edges)?
* `for_each` - call a passed lambda/callback for each point in the rectangle.
* `point_set` - returns a `HashSet` of all points in the rectangle.
* `width` and `height` return the current dimensions of the rectangle.

## Line Plotting

Line plotting is provided using Bresenham and vector algorithms. You can return points in the line as either a vector of `Point` objects, or an iterator.

For example, returning a line as a vector:

```rust
use bracket_geometry::prelude::*;
let bresenham_line = line2d(LineAlg::Bresenham, Point::new(1,1), Point::new(5,5));
println!("{:?}", bresenham_line);
```

Or iterating along the points:

```rust
use bracket_geometry::prelude::*;
for point in Bresenham::new(Point::new(1,1), Point::new(5,5)) {
    println!("{:?}", point);
}
```

You can substitute `LineAlg::Bresenham` with `LineAlg::Vector` to use a simple vector-based projection instead (this is faster on some systems).

## Circle Plotting

Bresenham's circle algorithm is also included. For example:

```rust
use bracket_geometry::prelude::*;
for point in BresenhamCircle::new(Point::new(10,10), 5) {
    println!("{:?}", point);
}
```

## Distance Heuristics

There's a full set of distance algorithms available:

```rust
use bracket_geometry::prelude::*;
println!("{:?}", DistanceAlg::Pythagoras.distance2d(Point::new(0,0), Point::new(5,5)));
println!("{:?}", DistanceAlg::PythagorasSquared.distance2d(Point::new(0,0), Point::new(5,5)));
println!("{:?}", DistanceAlg::Manhattan.distance2d(Point::new(0,0), Point::new(5,5)));
println!("{:?}", DistanceAlg::Chebyshev.distance2d(Point::new(0,0), Point::new(5,5)));
```

## Feature Flags

If you enable `serde`, it provides serialization/de-serialization via the `Serde` library for the `Point`, `Point3D` and `Rect` types.

## Examples

The following examples are included; they use `crossterm` to print to your terminal. Run examples with `cargo run --example <name>`:

* `bresenham_circle` - draws a circle.
* `bresenham_line` - draws a line.
* `vector_line` - draws a line, using an algorithm that doesn't smooth corners.
* `distance` - calculates distance between two points with all supported algorithms and outputs it.
