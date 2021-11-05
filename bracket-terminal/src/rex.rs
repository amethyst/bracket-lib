use crate::prelude::{Console, DrawBatch, FontCharType};
use bracket_color::prelude::{ColorPair, RGBA};
use bracket_geometry::prelude::Point;
pub use bracket_rex::prelude::{ XpCell, XpLayer, XpFile, XpColor };

/// Applies an XpFile to a given console, with 0,0 offset by offset_x and offset-y.
pub fn xp_to_console(
    xp: &XpFile,
    mut console: impl AsMut<dyn Console>,
    offset_x: i32,
    offset_y: i32,
) {
    for layer in &xp.layers {
        for y in 0..layer.height {
            for x in 0..layer.width {
                let cell = layer.get(x, y).unwrap();
                if !cell.bg.is_transparent() {
                    console.as_mut().set(
                        x as i32 + offset_x,
                        y as i32 + offset_y,
                        cell.fg.into(),
                        cell.bg.into(),
                        cell.ch as FontCharType,
                    );
                }
            }
        }
    }
}

/// Applies an XpFile to a given draw batch, with 0,0 offset by offset_x and offset-y.
pub fn xp_to_draw_batch(xp: &XpFile, draw_batch: &mut DrawBatch, offset_x: i32, offset_y: i32) {
    for layer in &xp.layers {
        for y in 0..layer.height {
            for x in 0..layer.width {
                let cell = layer.get(x, y).unwrap();
                if !cell.bg.is_transparent() {
                    draw_batch.set(
                        Point::new(x as i32 + offset_x, y as i32 + offset_y),
                        ColorPair::new(RGBA::from(cell.fg), RGBA::from(cell.bg)),
                        cell.ch as FontCharType,
                    );
                }
            }
        }
    }
}
