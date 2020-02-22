use crate::prelude::RGB;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Copy, Clone, Default, Debug)]
/// Represents two colors together, a foreground and a background.
pub struct ColorPair {
    /// The foreground color
    pub fg: RGB,
    /// The background color
    pub bg: RGB,
}

impl ColorPair {
    #[inline]
    #[must_use]
    pub fn new(fg: RGB, bg: RGB) -> Self {
        Self { fg, bg }
    }
}
