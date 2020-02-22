use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io;

/// Structure representing the components of one color
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XpColor {
    /// Red component 0..255
    pub r: u8,
    /// Green component 0..255
    pub g: u8,
    /// Blue component 0..255
    pub b: u8,
}

impl XpColor {
    /// deepest black
    pub const BLACK: Self = Self { r: 0, g: 0, b: 0 };
    /// color 0xff00ff (hot pink) is regarded as transparent
    pub const TRANSPARENT: Self = Self {
        r: 255,
        g: 0,
        b: 255,
    };

    /// Construct a new color from r,g,b values
    #[inline]
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Return whether this color is considered transparent (if this is the background color of a
    /// cell, the layer below it will see through)
    #[inline]
    #[must_use]
    pub fn is_transparent(self) -> bool {
        self == Self::TRANSPARENT
    }

    /// Read a RGB color from a `ReadBytesExt`
    #[inline]
    pub fn read<T: ReadBytesExt>(rdr: &mut T) -> io::Result<Self> {
        let r = rdr.read_u8()?;
        let g = rdr.read_u8()?;
        let b = rdr.read_u8()?;
        Ok(Self { r, g, b })
    }

    /// Write a RGB color to a `WriteBytesExt`
    #[inline]
    pub fn write<T: WriteBytesExt>(self, wr: &mut T) -> io::Result<()> {
        wr.write_u8(self.r)?;
        wr.write_u8(self.g)?;
        wr.write_u8(self.b)?;
        Ok(())
    }
}
