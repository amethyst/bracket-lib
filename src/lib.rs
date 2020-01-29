#![warn(clippy::pedantic)]
#[macro_use]
extern crate lazy_static;
mod codepage437;
mod color;
pub mod console;
mod fastnoise;
mod fieldofview;
mod geometry;
mod gui_helpers;
mod hal;
mod pathfinding;
mod random;
pub mod rex;
mod rltk;
mod simple_console;
mod sparse_console;
pub mod textblock;
pub use hal::*;
mod algorithm2d;
mod algorithm3d;
mod basemap;
#[cfg(feature = "parsing")]
mod parsing;

pub use self::codepage437::{string_to_cp437, to_char, to_cp437};
pub use self::color::*;
pub use self::console::*;
pub use self::fastnoise::*;
pub use self::fieldofview::{field_of_view, field_of_view_set};
pub use self::font::Font;
pub use self::geometry::{
    line2d, project_angle, Bresenham, BresenhamCircle, BresenhamCircleNoDiag, DistanceAlg, LineAlg,
    Point, Point3, Rect, VectorLine,
};
pub use self::pathfinding::astar::{a_star_search, NavigationPath};
pub use self::pathfinding::dijkstra::DijkstraMap;
pub use self::random::RandomNumberGenerator;
pub use self::rltk::{letter_to_option, main_loop, Rltk};
pub use self::simple_console::SimpleConsole;
pub use self::sparse_console::SparseConsole;
pub use self::textblock::{TextBlock, TextBuilder};
pub use algorithm2d::Algorithm2D;
pub use algorithm3d::Algorithm3D;
pub use basemap::BaseMap;
#[cfg(feature = "parsing")]
pub use parsing::{parse_dice_string, DiceParseError, DiceType};
mod command_buffer;
pub mod embedding;
pub use command_buffer::*;
mod initializer;
pub use initializer::*;

pub mod prelude {
    pub use crate::*;
}

#[macro_export]
macro_rules! add_wasm_support {
    () => {
        //#[cfg(target_arch = "wasm32")]
        //extern crate console_error_panic_hook;

        //#[cfg(target_arch = "wasm32")]
        //use std::panic;

        #[cfg(target_arch = "wasm32")]
        use wasm_bindgen::prelude::*;

        #[cfg(target_arch = "wasm32")]
        #[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
        pub fn wasm_main() {
            //panic::set_hook(Box::new(console_error_panic_hook::hook));
            main();
        }
    };
}

#[macro_export]
macro_rules! embedded_resource {
    ($resource_name : ident, $filename : expr) => {
        const $resource_name: &'static [u8] = include_bytes!($filename);
    };
}

#[macro_export]
macro_rules! link_resource {
    ($resource_name : ident, $filename : expr) => {
        rltk::embedding::EMBED
            .lock()
            .unwrap()
            .add_resource($filename.to_string(), $resource_name);
    };
}

#[cfg(all(feature = "opengl", not(target_arch = "wasm32")))]
pub use glutin::event::VirtualKeyCode;

#[cfg(all(
    not(feature = "opengl"),
    any(feature = "amethyst_engine_vulkan", feature = "amethyst_engine_metal")
))]
pub use amethyst::input::VirtualKeyCode;

#[cfg(target_arch = "wasm32")]
pub use hal::VirtualKeyCode;

/// Implement this trait on your state struct, so the engine knows what to call on each tick.
pub trait GameState: 'static {
    fn tick(&mut self, ctx: &mut Rltk);
}
