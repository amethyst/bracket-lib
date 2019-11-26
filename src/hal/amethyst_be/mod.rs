// Platform to integrate into Amethyst
use crate::{GameState, Rltk};

use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
        palette::Srgba
    },
    utils::application_root_dir,
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    core::TransformBundle,
    renderer::{Camera, ImageFormat, SpriteSheet, Texture},
    tiles::{MortonEncoder2D, RenderTiles2D, Tile, TileMap},
    core::math::{Point3, Vector3},
    input::{InputBundle, StringBindings, Bindings, InputHandler, Button},
    winit::MouseButton
};
pub mod shader;
pub mod font;
mod init;
pub use init::*;
mod tiles;
use tiles::*;
mod mainloop;
use mainloop::*;
mod dummy;
pub use dummy::*;
