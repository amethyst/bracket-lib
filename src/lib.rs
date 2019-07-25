pub mod gl {
    pub use self::Gles2 as Gl;
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

pub struct Gl {
    pub gl: gl::Gl,
}

mod color;
mod font;
mod shader;
mod rltk;
mod console;
mod simple_console;
mod sparse_console;
mod fieldofview;
mod geometry;
mod pathfinding;
pub mod rex;
mod codepage437;
mod framebuffer;
mod quadrender;
mod gui_helpers;
mod random;
mod fastnoise;
pub mod textblock;

pub use self::rltk::main_loop;
pub use self::rltk::Rltk;
pub use self::rltk::letter_to_option;
pub use self::color::*;
pub use self::font::Font;
pub use self::console::*;
pub use self::shader::Shader;
pub use self::simple_console::SimpleConsole;
pub use self::sparse_console::SparseConsole;
pub use self::fieldofview::field_of_view;
pub use self::geometry::{ distance2d, distance3d, DistanceAlg, line2d, project_angle };
pub use self::pathfinding::dijkstra::DijkstraMap;
pub use self::pathfinding::astar::{a_star_search, NavigationPath};
pub use glutin::event::VirtualKeyCode;
pub use self::codepage437::{string_to_cp437, to_cp437};
pub use self::random::RandomNumberGenerator;
pub use self::fastnoise::*;
pub use self::textblock::{TextBlock, TextBuilder};

#[cfg(feature = "serialization")]
extern crate serde;

/// Implement this trait on your state struct, so the engine knows what to call on each tick.
pub trait GameState {
    fn tick(&mut self, ctx : &mut Rltk);
}

#[cfg(feature = "serialization")]
#[derive(Eq, PartialEq, Copy, Clone, serde::Serialize, serde::Deserialize, Debug)]
/// Helper function definint a 2D point in space.
pub struct Point {
    pub x: i32,
    pub y: i32
}

#[cfg(not(feature = "serialization"))]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
/// Helper function definint a 2D point in space.
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    /// Create a new point from an x/y coordinate.
    pub fn new(x:i32, y:i32) -> Point {
        return Point{x, y};
    }
}

#[cfg(feature = "serialization")]
#[derive(Eq, PartialEq, Copy, Clone, serde::Serialize, serde::Deserialize, Debug)]
/// Helper function definint a 2D point in space.
pub struct Point3 {
    pub x: i32,
    pub y: i32,
    pub z: i32
}

#[cfg(not(feature = "serialization"))]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
/// Helper function definint a 2D point in space.
pub struct Point3 {
    pub x: i32,
    pub y: i32,
    pub z: i32
}

impl Point3 {
    /// Create a new point from an x/y coordinate.
    pub fn new(x:i32, y:i32, z:i32) -> Point3 {
        return Point3{x, y, z};
    }
}

/// Implement this trait to support path-finding functions.
pub trait BaseMap {
    /// True is you can see through the tile, false otherwise.
    fn is_opaque(&self, idx: i32) -> bool;

    /// Return a vector of tile indices to which one can path from the idx.
    /// These do NOT have to be contiguous - if you want to support teleport pads, that's awesome.
    fn get_available_exits(&self, idx:i32) -> Vec<(i32, f32)>;

    /// Return the distance you would like to use for path-finding. Generally, Pythagoras distance (implemented in geometry)
    /// is fine, but you might use Manhattan or any other heuristic that fits your problem.
    fn get_pathing_distance(&self, idx1:i32, idx2:i32) -> f32;
}

/// Implement these for handling conversion to/from 2D coordinates (they are separate, because you might
/// want Dwarf Fortress style 3D!)
pub trait Algorithm2D : BaseMap {
    /// Convert a Point (x/y) to an array index.
    fn point2d_to_index(&self, pt : Point) -> i32;

    /// Convert an array index to a point.
    fn index_to_point2d(&self, idx:i32) -> Point;
}

/// Implement these for handling conversion to/from 2D coordinates (they are separate, because you might
/// want Dwarf Fortress style 3D!)
pub trait Algorithm3D : BaseMap {
    /// Convert a Point (x/y) to an array index.
    fn point3d_to_index(&self, pt : Point3) -> i32;

    /// Convert an array index to a point.
    fn index_to_point3d(&self, idx:i32) -> Point3;
}