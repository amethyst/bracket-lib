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

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    // Tests that we make a a pair of colors from RGB, and they are both black.
    fn make_rgb_pair() {
        let cp = ColorPair::new(RGB::named(BLACK), RGB::named(BLACK));
        assert!(cp.fg.r < std::f32::EPSILON);
        assert!(cp.fg.g < std::f32::EPSILON);
        assert!(cp.fg.b < std::f32::EPSILON);
        assert!((cp.fg.a - 1.0).abs() < std::f32::EPSILON);
        assert!(cp.bg.r < std::f32::EPSILON);
        assert!(cp.bg.g < std::f32::EPSILON);
        assert!(cp.bg.b < std::f32::EPSILON);
        assert!((cp.bg.a - 1.0).abs() < std::f32::EPSILON);
    }

    #[test]
    // Tests that we make a a pair of colors from RGB, and they are both black.
    fn make_rgba_pair() {
        let cp = ColorPair::new(RGBA::new(), RGBA::new());
        assert!(cp.fg.r < std::f32::EPSILON);
        assert!(cp.fg.g < std::f32::EPSILON);
        assert!(cp.fg.b < std::f32::EPSILON);
        assert!(cp.fg.a < std::f32::EPSILON);
        assert!(cp.bg.r < std::f32::EPSILON);
        assert!(cp.bg.g < std::f32::EPSILON);
        assert!(cp.bg.b < std::f32::EPSILON);
        assert!(cp.bg.a < std::f32::EPSILON);
    }
}
