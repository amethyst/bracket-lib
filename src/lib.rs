pub mod gl {
    pub use self::Gles2 as Gl;
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

pub struct Gl {
    pub gl: gl::Gl,
}

mod codepage437;
mod color;
mod console;
mod fastnoise;
mod fieldofview;
mod font;
mod framebuffer;
mod geometry;
mod gui_helpers;
mod pathfinding;
mod quadrender;
mod random;
pub mod rex;
mod rltk;
mod shader;
mod simple_console;
mod sparse_console;
pub mod textblock;

pub use self::codepage437::{string_to_cp437, to_cp437};
pub use self::color::*;
pub use self::console::*;
pub use self::fastnoise::*;
pub use self::fieldofview::field_of_view;
pub use self::font::Font;
pub use self::geometry::{line2d, project_angle, DistanceAlg, LineAlg, Point, Point3};
pub use self::pathfinding::astar::{a_star_search, NavigationPath};
pub use self::pathfinding::dijkstra::DijkstraMap;
pub use self::random::RandomNumberGenerator;
pub use self::rltk::{letter_to_option, main_loop, Rltk};
pub use self::shader::Shader;
pub use self::simple_console::SimpleConsole;
pub use self::sparse_console::SparseConsole;
pub use self::textblock::{TextBlock, TextBuilder};
pub use glutin::event::VirtualKeyCode;

/// Implement this trait on your state struct, so the engine knows what to call on each tick.
pub trait GameState: 'static {
    fn tick(&mut self, ctx: &mut Rltk);
}

/// Implement this trait to support path-finding functions.
pub trait BaseMap {
    /// True is you can see through the tile, false otherwise.
    fn is_opaque(&self, idx: i32) -> bool;

    /// Return a vector of tile indices to which one can path from the idx.
    /// These do NOT have to be contiguous - if you want to support teleport pads, that's awesome.
    fn get_available_exits(&self, idx: i32) -> Vec<(i32, f32)>;

    /// Return the distance you would like to use for path-finding. Generally, Pythagoras distance (implemented in geometry)
    /// is fine, but you might use Manhattan or any other heuristic that fits your problem.
    fn get_pathing_distance(&self, idx1: i32, idx2: i32) -> f32;
}

/// Implement these for handling conversion to/from 2D coordinates (they are separate, because you might
/// want Dwarf Fortress style 3D!)
pub trait Algorithm2D: BaseMap {
    /// Convert a Point (x/y) to an array index.
    fn point2d_to_index(&self, pt: Point) -> i32;

    /// Convert an array index to a point.
    fn index_to_point2d(&self, idx: i32) -> Point;
}

/// Implement these for handling conversion to/from 2D coordinates (they are separate, because you might
/// want Dwarf Fortress style 3D!)
pub trait Algorithm3D: BaseMap {
    /// Convert a Point (x/y) to an array index.
    fn point3d_to_index(&self, pt: Point3) -> i32;

    /// Convert an array index to a point.
    fn index_to_point3d(&self, idx: i32) -> Point3;
}
