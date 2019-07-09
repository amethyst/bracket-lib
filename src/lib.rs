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
mod dijkstra;
mod astar;

pub use self::rltk::main_loop;
pub use self::rltk::Rltk;
pub use self::color::*;
pub use self::font::Font;
pub use self::console::*;
pub use self::shader::Shader;
pub use self::simple_console::SimpleConsole;
pub use self::sparse_console::SparseConsole;
pub use self::fieldofview::field_of_view;
pub use self::geometry::{ distance2d, distance2d_squared };
pub use self::dijkstra::DijkstraMap;
pub use self::astar::{a_star_search, NavigationPath};
pub use glutin::event::VirtualKeyCode;

pub trait GameState {
    fn tick(&mut self, ctx : &mut Rltk);
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    pub fn new(x:i32, y:i32) -> Point {
        return Point{x, y};
    }
}

pub trait BaseMap {
    fn is_opaque(&self, idx: i32) -> bool;
    fn get_available_exits(&self, idx:i32) -> Vec<(i32, f32)>;
    fn get_pathing_distance(&self, idx1:i32, idx2:i32) -> f32;
}

pub trait Algorithm2D : BaseMap {
    fn point2d_to_index(&self, pt : Point) -> i32;
    fn index_to_point2d(&self, idx:i32) -> Point;
}