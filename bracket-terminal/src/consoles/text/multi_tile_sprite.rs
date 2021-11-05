use crate::prelude::{string_to_cp437, BTerm, DrawBatch, FontCharType, Tile, XpFile};
use bracket_color::prelude::{ColorPair, RGBA};
use bracket_geometry::prelude::Point;

/// Represents a sprite consisting of multiple glyphs/colors, occupying multiple console locations.
pub struct MultiTileSprite {
    content: Vec<Tile>,
    dimensions: Point,
}

impl MultiTileSprite {
    /// Generates a sprite from an input string, divided into width x height sizes.
    pub fn from_string<S: ToString, T>(content: S, width: T, height: T) -> Self
    where
        T: Into<i32>,
    {
        let w: i32 = width.into();
        let h: i32 = height.into();
        let content_s = content.to_string();

        let bytes = string_to_cp437(content_s);
        let tiles = bytes
            .into_iter()
            .map(|glyph| Tile {
                glyph,
                fg: RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                bg: RGBA::from_f32(0.0, 0.0, 0.0, 1.0),
            })
            .collect();

        Self {
            content: tiles,
            dimensions: Point::new(w, h),
        }
    }

    /// Generates a sprite from an input string, divided into width x height sizes. Also provides foreground and background colors.
    pub fn from_string_colored<S: ToString, T>(
        content: S,
        width: T,
        height: T,
        fg: &[RGBA],
        bg: &[RGBA],
    ) -> Self
    where
        T: Into<i32>,
    {
        let w: i32 = width.into();
        let h: i32 = height.into();
        let content_s = content.to_string();

        let bytes = string_to_cp437(content_s);
        let tiles = bytes
            .into_iter()
            .enumerate()
            .map(|(i, glyph)| Tile {
                glyph,
                fg: fg[i],
                bg: bg[i],
            })
            .collect();

        Self {
            content: tiles,
            dimensions: Point::new(w, h),
        }
    }

    /// Import a sprite from an XP Rex Paint file.
    pub fn from_xp(rex: &XpFile) -> Self {
        let dimensions = Point::new(rex.layers[0].width, rex.layers[0].height);
        let mut tiles: Vec<Tile> = vec![
            Tile {
                glyph: 0,
                fg: RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                bg: RGBA::from_f32(0.0, 0.0, 0.0, 1.0)
            };
            (dimensions.x * dimensions.y) as usize
        ];

        for layer in &rex.layers {
            for y in 0..layer.height {
                for x in 0..layer.width {
                    let cell = layer.get(x, y).unwrap();
                    if !cell.bg.is_transparent() {
                        let idx = (y * (dimensions.x as usize)) + (x as usize);
                        tiles[idx].glyph = cell.ch as FontCharType;
                        tiles[idx].fg = cell.fg.into();
                        tiles[idx].bg = cell.bg.into();
                    }
                }
            }
        }

        Self {
            content: tiles,
            dimensions,
        }
    }

    /// Directly renders a sprite to an BTerm context.
    pub fn render(&self, context: &mut BTerm, position: Point) {
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

    /// Appends draw-calls to a batch to render a multi-tile sprite.
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
