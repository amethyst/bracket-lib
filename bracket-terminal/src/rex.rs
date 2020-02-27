// The work here is heavily derived from rs-rexpaint, https://gitlab.com/medusacle/rs-rexpaint
// It is Copyright (c) 2018, Mara <cyphergothic@protonmail.com>
// It's under the DWTFYW Public License 2.0, so inclusion in an MIT-licensed program
// isn't a problem.

#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]

use std::io;
use std::io::prelude::*;

use super::embedding;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;

use crate::prelude::Console;
use bracket_color::prelude::{XpColor, RGB};

/// Structure representing a character and its foreground/background color
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XpCell {
    /// Character index
    /// This depends on the font but will usually be a code page 437 character
    /// (one way to convert to a rust unicode character one way is to use
    /// `CP437_WINGDINGS.decode(...)` in the `codepage_437` crate!)
    pub ch: u32,
    /// Foreground color
    pub fg: XpColor,
    /// Background color
    pub bg: XpColor,
}

/// Structure representing a layer
/// Cells are in the same order as in the file, in column-major order (index of position x,y is y*height + x).
#[derive(Debug, Clone, PartialEq)]
pub struct XpLayer {
    /// Width of layer (in cells)
    pub width: usize,
    /// Height of layer (in cells)
    pub height: usize,
    /// Content of layer
    pub cells: Vec<XpCell>,
}

impl XpLayer {
    /// Construct a new XpLayer of width by height. The contents will be empty (black foreground
    /// and background, character 0).
    pub fn new(width: usize, height: usize) -> XpLayer {
        XpLayer {
            width,
            height,
            cells: vec![
                XpCell {
                    ch: 0,
                    fg: XpColor::BLACK,
                    bg: XpColor::BLACK
                };
                width * height
            ],
        }
    }

    /// Get the cell at coordinates (x,y), or None if it is out of range.
    pub fn get(&self, x: usize, y: usize) -> Option<&XpCell> {
        if x < self.width && y < self.height {
            Some(&self.cells[x * self.height + y])
        } else {
            None
        }
    }

    /// Get mutable reference to the cell at coordinates (x,y), or None if it is out of range.
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut XpCell> {
        if x < self.width && y < self.height {
            Some(&mut self.cells[x * self.height + y])
        } else {
            None
        }
    }
}

/// Structure representing a REXPaint image file which is a stack of layers
#[derive(Debug, Clone, PartialEq)]
pub struct XpFile {
    /// Version number from header
    pub version: i32,
    /// Layers of the image
    pub layers: Vec<XpLayer>,
}

impl XpFile {
    /// Construct a new XpFile with one layer of width by height. The contents will be empty (black
    /// foreground and background, character 0).
    pub fn new(width: usize, height: usize) -> XpFile {
        XpFile {
            version: -1,
            layers: vec![XpLayer::new(width, height)],
        }
    }

    /// Helper to read from an BTerm resource
    pub fn from_resource(path: &str) -> io::Result<XpFile> {
        let res = embedding::EMBED
            .lock()
            .unwrap()
            .get_resource(path.to_string());
        match res {
            None => panic!("Unable to open resource"),
            Some(r) => {
                let buffer: Vec<u8> = Vec::from(r);
                let mut bufslice = &*buffer;
                XpFile::read(&mut bufslice)
            }
        }
    }

    /// Read a xp image from a stream
    pub fn read<R: Read>(f: &mut R) -> io::Result<XpFile> {
        let mut rdr = GzDecoder::new(f);
        let version = rdr.read_i32::<LittleEndian>()?;
        let num_layers = rdr.read_u32::<LittleEndian>()?;

        let mut layers = Vec::<XpLayer>::new();
        layers.reserve(num_layers as usize);
        for _ in 0..num_layers {
            let width = rdr.read_u32::<LittleEndian>()? as usize;
            let height = rdr.read_u32::<LittleEndian>()? as usize;

            let mut cells = Vec::<XpCell>::new();
            cells.reserve(width * height);
            for _ in 0..width {
                // column-major order
                for _ in 0..height {
                    let ch = rdr.read_u32::<LittleEndian>()?;
                    let fg = XpColor::read(&mut rdr)?;
                    let bg = XpColor::read(&mut rdr)?;
                    cells.push(XpCell { ch, fg, bg });
                }
            }
            layers.push(XpLayer {
                width,
                height,
                cells,
            });
        }
        Ok(XpFile { version, layers })
    }

    /// Write a xp image to a stream
    pub fn write<W: Write>(&self, f: &mut W) -> io::Result<()> {
        let mut wr = GzEncoder::new(f, Compression::best());
        wr.write_i32::<LittleEndian>(self.version)?; // only supported version is -1
        wr.write_u32::<LittleEndian>(self.layers.len() as u32)?;
        for layer in &self.layers {
            wr.write_u32::<LittleEndian>(layer.width as u32)?;
            wr.write_u32::<LittleEndian>(layer.height as u32)?;

            for cell in &layer.cells {
                wr.write_u32::<LittleEndian>(cell.ch)?;
                cell.fg.write(&mut wr)?;
                cell.bg.write(&mut wr)?;
            }
        }
        Ok(())
    }
}

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
                        RGB::from_xp(cell.fg),
                        RGB::from_xp(cell.bg),
                        cell.ch as u8,
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{Cursor, Seek, SeekFrom};

    const WIDTH: usize = 80;
    const HEIGHT: usize = 60;

    #[test]
    fn test_roundtrip() {
        let mut xp = XpFile::new(WIDTH, HEIGHT);
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let cell = xp.layers[0].get_mut(x, y).unwrap();
                cell.ch = (32 + x + y) as u32;
                cell.fg = XpColor::new(y as u8, 0, 255 - y as u8);
                cell.bg = XpColor::new(x as u8, 0, 255 - x as u8);
            }
        }

        let mut f = Cursor::new(Vec::new());
        xp.write(&mut f).unwrap();
        f.seek(SeekFrom::Start(0)).unwrap();

        let xp2 = XpFile::read(&mut f).unwrap();
        assert_eq!(xp, xp2);
    }

    #[test]
    fn test_image() {
        let mut f = File::open("resources/mltest.xp").unwrap();
        let xp = XpFile::read(&mut f).unwrap();
        assert_eq!(xp.version, -1);
        assert_eq!(xp.layers.len(), 2);
        assert_eq!(xp.layers[0].width, 8);
        assert_eq!(xp.layers[0].height, 4);
        assert_eq!(xp.layers[1].width, 8);
        assert_eq!(xp.layers[1].height, 4);
        assert_eq!(xp.layers[1].get(0, 0).unwrap().fg, XpColor::BLACK);
        assert_eq!(xp.layers[1].get(0, 0).unwrap().bg.is_transparent(), true);
        assert_eq!(xp.layers[1].get(0, 0).unwrap().ch, 32);
        assert_eq!(xp.layers[1].get(2, 2).unwrap().ch, 'B' as u32);
        assert_eq!(xp.layers[0].get(0, 0).unwrap().fg, XpColor::new(0, 0, 255));
        assert_eq!(xp.layers[0].get(0, 0).unwrap().bg, XpColor::BLACK);
        assert_eq!(xp.layers[0].get(0, 0).unwrap().ch, 'A' as u32);
    }
}
