use super::mainloop::SimpleConsoleResource;

use amethyst::{
    prelude::*,
    renderer::{
        palette::Srgba
    },
    tiles::{Tile},
    core::math::{Point3},
};

#[derive(Default, Clone)]
pub struct SimpleConsoleTile;

impl Tile for SimpleConsoleTile {  
    fn sprite(&self, pt : Point3<u32>, world: &World) -> Option<usize> {
        let tiles = world.fetch::<SimpleConsoleResource>();
        let y = (tiles.size.1-1) - pt.y;
        let idx = (y * tiles.size.0) + pt.x;
        Some(tiles.tiles[idx as usize].glyph as usize)
    }

    fn tint(&self, pt: Point3<u32>, world: &World) -> Srgba {
        let tiles = world.fetch::<SimpleConsoleResource>();
        let y = (tiles.size.1-1) - pt.y;
        let idx = (y * tiles.size.0) + pt.x;
        let fg = tiles.tiles[idx as usize].fg;
        Srgba::new(fg.r, fg.g, fg.b, 1.0)
    }
}
