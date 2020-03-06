use crate::prelude::RGBA;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Copy, Clone, Default, Debug)]
/// Represents two colors together, a foreground and a background.
pub struct ColorPair {
    /// The foreground color
    pub fg: RGBA,
    /// The background color
    pub bg: RGBA,
}

impl ColorPair {
    #[inline]
    #[must_use]
    /// Creates a new `ColorPair`, from two given colors.
    pub fn new<COLOR, COLOR2>(fg: COLOR, bg: COLOR2) -> Self
    where
        COLOR: Into<RGBA>,
        COLOR2: Into<RGBA>,
    {
        Self {
            fg: fg.into(),
            bg: bg.into(),
        }
    }
}
