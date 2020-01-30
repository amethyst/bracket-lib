use crate::prelude::*;
use std::convert::TryInto;
pub struct MultiTileSprite {
    content: Vec<Tile>,
    dimensions: Point,
}

impl MultiTileSprite {
    pub fn from_string<S: ToString, T>(content: S, width: T, height: T) -> Self
    where
        T: TryInto<i32>,
    {
        let w: i32 = width.try_into().ok().unwrap();
        let h: i32 = height.try_into().ok().unwrap();
        let content_s = content.to_string();

        let bytes = super::string_to_cp437(content_s);
        let tiles = bytes
            .into_iter()
            .map(|glyph| Tile {
                glyph,
                fg: RGB::from_f32(1.0, 1.0, 1.0),
                bg: RGB::from_f32(0.0, 0.0, 0.0),
            })
            .collect();

        Self {
            content: tiles,
            dimensions: Point::new(w, h),
        }
    }

    pub fn render(&self, context: &mut Rltk, position: Point) {
        let mut x = 0;
        let mut y = 0;
        for tile in self.content.iter() {
            x += 1;
            context.set(x + position.x, y + position.y, tile.fg, tile.bg, tile.glyph);
            if x >= self.dimensions.x {
                x = 0;
                y += 1;
            }
        }
    }

    pub fn add_to_batch(&self, batch: &mut DrawBatch, position: Point) {
        let mut pos = Point::zero();
        for tile in self.content.iter() {
            pos.x += 1;
            batch.set(pos + position, ColorPair::new(tile.fg, tile.bg), tile.glyph);
            if pos.x >= self.dimensions.x {
                pos.x = 0;
                pos.y += 1;
            }
        }
    }
}
