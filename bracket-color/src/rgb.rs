use crate::prelude::{HSV, RGBA};
use std::convert::From;
use std::ops;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Copy, Clone, Default, Debug)]
/// Represents an R/G/B triplet, in the range 0..1 (32-bit float)
pub struct RGB {
    /// The red component (0..1)
    pub r: f32,
    /// The green components (0..1)
    pub g: f32,
    /// The blue component (0..1)
    pub b: f32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
/// Error message type when failing to convert a hex code to RGB.
pub enum HtmlColorConversionError {
    /// The HTML string was not a valid length. (Expects #AABBCC)
    InvalidStringLength,
    /// No # was included in the string.
    MissingHash,
    /// An unexpected character (not #, A-F) was detected in the color string.
    InvalidCharacter,
}

// Implement operator overloading

/// Support adding a float to a color. The result is clamped via the constructor.
impl ops::Add<f32> for RGB {
    type Output = Self;
    #[must_use]
    fn add(mut self, rhs: f32) -> Self {
        self.r += rhs;
        self.g += rhs;
        self.b += rhs;
        self
    }
}

/// Support adding an RGB to a color. The result is clamped via the constructor.
impl ops::Add<RGB> for RGB {
    type Output = Self;
    #[must_use]
    fn add(mut self, rhs: Self) -> Self {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
        self
    }
}

/// Support subtracting a float from a color. The result is clamped via the constructor.
impl ops::Sub<f32> for RGB {
    type Output = Self;
    #[must_use]
    fn sub(mut self, rhs: f32) -> Self {
        self.r -= rhs;
        self.g -= rhs;
        self.b -= rhs;
        self
    }
}

/// Support subtracting an RGB from a color. The result is clamped via the constructor.
impl ops::Sub<RGB> for RGB {
    type Output = Self;
    #[must_use]
    fn sub(mut self, rhs: Self) -> Self {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
        self
    }
}

/// Support multiplying a color by a float. The result is clamped via the constructor.
impl ops::Mul<f32> for RGB {
    type Output = Self;
    #[must_use]
    fn mul(mut self, rhs: f32) -> Self {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
        self
    }
}

/// Support multiplying a color by another color. The result is clamped via the constructor.
impl ops::Mul<RGB> for RGB {
    type Output = Self;
    #[must_use]
    fn mul(mut self, rhs: Self) -> Self {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
        self
    }
}

/// Support conversion from a color tuple
impl From<(u8, u8, u8)> for RGB {
    fn from(vals: (u8, u8, u8)) -> Self {
        Self::named(vals)
    }
}

/// Support conversion from HSV
impl From<HSV> for RGB {
    fn from(hsv: HSV) -> Self {
        hsv.to_rgb()
    }
}

/// Support conversion from RGBA
impl From<RGBA> for RGB {
    fn from(item: RGBA) -> Self {
        Self::from_f32(item.r, item.g, item.b)
    }
}

// Support conversion from Bevy
#[cfg(feature = "bevy")]
impl From<bevy::prelude::Color> for RGB {
    fn from(item: bevy::prelude::Color) -> Self {
        Self::from_f32(item.r(), item.g(), item.b())
    }
}

#[cfg(feature = "bevy")]
impl From<RGB> for bevy::prelude::Color {
    fn from(item: RGB) -> Self {
        Self::from([item.r, item.g, item.b])
    }
}

impl RGB {
    /// Constructs a new, zeroed (black) RGB triplet.
    #[must_use]
    pub fn new() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    /// Constructs a new RGB color, from 3 32-bit floats in the range 0..1
    ///
    /// # Arguments
    ///
    /// * `r` - the red component (0..1)
    /// * `g` - the green component (0..1)
    /// * `b` - the blue component (0..1)
    ///
    /// # Example
    ///
    /// ```rust
    /// use bracket_color::prelude::*;
    /// let red = RGB::from_f32(1.0, 0.0, 0.0);
    /// let green = RGB::from_f32(0.0, 1.0, 0.0);
    /// ```
    #[inline]
    #[must_use]
    pub fn from_f32(r: f32, g: f32, b: f32) -> Self {
        let r_clamped = f32::min(1.0, f32::max(0.0, r));
        let g_clamped = f32::min(1.0, f32::max(0.0, g));
        let b_clamped = f32::min(1.0, f32::max(0.0, b));
        Self {
            r: r_clamped,
            g: g_clamped,
            b: b_clamped,
        }
    }

    /// Constructs a new RGB color, from 3 bytes in the range 0..255
    ///
    /// # Arguments
    ///
    /// * `r` - the red component, ranged from 0 to 255
    /// * `g` - the green component, ranged from 0 to 255
    /// * `b` - the blue component, ranged from 0 to 255
    ///
    /// # Example
    ///
    /// ```rust
    /// use bracket_color::prelude::*;
    /// let red = RGB::from_u8(255, 0, 0);
    /// let green = RGB::from_u8(0, 255, 0);
    /// ```
    #[inline]
    #[must_use]
    pub fn from_u8(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: f32::from(r) / 255.0,
            g: f32::from(g) / 255.0,
            b: f32::from(b) / 255.0,
        }
    }

    /// Construct an RGB color from a tuple of u8, or a named constant
    ///
    /// # Arguments
    ///
    /// * `col` a tuple of three `u8` values. See `from_u8`. These are usually provided from the `named` colors list.
    ///
    /// # Example
    ///
    /// ```rust
    /// use bracket_color::prelude::*;
    /// let red = RGB::named(RED);
    /// let green = RGB::named((0, 255, 0));
    /// ```
    #[inline]
    #[must_use]
    pub fn named(col: (u8, u8, u8)) -> Self {
        Self::from_u8(col.0, col.1, col.2)
    }

    /// Constructs from an HTML color code (e.g. "#eeffee")
    ///
    /// # Arguments
    ///
    /// * `code` - an HTML color notation (e.g. "#ffeeff")
    ///
    /// # Example
    ///
    /// ```rust
    /// use bracket_color::prelude::*;
    /// let red = RGB::from_hex("#FF0000");
    /// let green = RGB::from_hex("#00FF00");
    /// ```
    ///
    /// # Errors
    ///
    /// See `HtmlColorConversionError`
    #[allow(clippy::cast_precision_loss)]
    pub fn from_hex<S: AsRef<str>>(code: S) -> Result<Self, HtmlColorConversionError> {
        let mut full_code = code.as_ref().chars();

        if let Some(hash) = full_code.next() {
            if hash != '#' {
                return Err(HtmlColorConversionError::MissingHash);
            }
        } else {
            return Err(HtmlColorConversionError::InvalidStringLength);
        }

        let red1 = match full_code.next() {
            Some(red) => match red.to_digit(16) {
                Some(red) => red * 16,
                None => return Err(HtmlColorConversionError::InvalidCharacter),
            },
            None => return Err(HtmlColorConversionError::InvalidStringLength),
        };
        let red2 = match full_code.next() {
            Some(red) => match red.to_digit(16) {
                Some(red) => red,
                None => return Err(HtmlColorConversionError::InvalidCharacter),
            },
            None => return Err(HtmlColorConversionError::InvalidStringLength),
        };

        let green1 = match full_code.next() {
            Some(green) => match green.to_digit(16) {
                Some(green) => green * 16,
                None => return Err(HtmlColorConversionError::InvalidCharacter),
            },
            None => return Err(HtmlColorConversionError::InvalidStringLength),
        };
        let green2 = match full_code.next() {
            Some(green) => match green.to_digit(16) {
                Some(green) => green,
                None => return Err(HtmlColorConversionError::InvalidCharacter),
            },
            None => return Err(HtmlColorConversionError::InvalidStringLength),
        };

        let blue1 = match full_code.next() {
            Some(blue) => match blue.to_digit(16) {
                Some(blue) => blue * 16,
                None => return Err(HtmlColorConversionError::InvalidCharacter),
            },
            None => return Err(HtmlColorConversionError::InvalidStringLength),
        };
        let blue2 = match full_code.next() {
            Some(blue) => match blue.to_digit(16) {
                Some(blue) => blue,
                None => return Err(HtmlColorConversionError::InvalidCharacter),
            },
            None => return Err(HtmlColorConversionError::InvalidStringLength),
        };

        if full_code.next().is_some() {
            return Err(HtmlColorConversionError::InvalidStringLength);
        }

        Ok(Self {
            r: (red1 + red2) as f32 / 255.0,
            g: (green1 + green2) as f32 / 255.0,
            b: (blue1 + blue2) as f32 / 255.0,
        })
    }

    /// Converts an RGB triple to an HSV triple.
    #[allow(clippy::many_single_char_names)]
    #[must_use]
    pub fn to_hsv(&self) -> HSV {
        let r = self.r;
        let g = self.g;
        let b = self.b;

        let max = f32::max(f32::max(r, g), b);
        let min = f32::min(f32::min(r, g), b);

        let mut h: f32 = max;
        let v: f32 = max;

        let d = max - min;
        let s = if max == 0.0 { 0.0 } else { d / max };

        if (max - min).abs() < std::f32::EPSILON {
            h = 0.0; // Achromatic
        } else {
            if (max - r).abs() < std::f32::EPSILON {
                if g < b {
                    h = (g - b) / d + 6.0;
                } else {
                    h = (g - b) / d;
                }
            } else if (max - g).abs() < std::f32::EPSILON {
                h = (b - r) / d + 2.0;
            } else if (max - b).abs() < std::f32::EPSILON {
                h = (r - g) / d + 4.0;
            }

            h /= 6.0;
        }

        HSV::from_f32(h, s, v)
    }

    /// Converts an RGB to an RGBA
    #[inline]
    #[must_use]
    pub fn to_rgba(&self, alpha: f32) -> RGBA {
        RGBA::from_f32(self.r, self.g, self.b, alpha)
    }

    /// Applies a quick grayscale conversion to the color
    #[inline]
    #[must_use]
    pub fn to_greyscale(&self) -> Self {
        let linear = (self.r * 0.2126) + (self.g * 0.7152) + (self.b * 0.0722);
        Self::from_f32(linear, linear, linear)
    }

    /// Applies a lengthier desaturate (via HSV) to the color
    #[inline]
    #[must_use]
    pub fn desaturate(&self) -> Self {
        let mut hsv = self.to_hsv();
        hsv.s = 0.0;
        hsv.to_rgb()
    }

    /// Lerps by a specified percentage (from 0 to 1) between this color and another
    #[inline]
    #[must_use]
    pub fn lerp(&self, color: Self, percent: f32) -> Self {
        let range = (color.r - self.r, color.g - self.g, color.b - self.b);
        Self {
            r: self.r + range.0 * percent,
            g: self.g + range.1 * percent,
            b: self.b + range.2 * percent,
        }
    }
}

#[cfg(feature = "crossterm")]
mod crossterm_features {
    use super::RGB;
    use crossterm::style::Color;
    use std::convert::TryFrom;

    impl TryFrom<RGB> for Color {
        type Error = &'static str;

        fn try_from(rgb: RGB) -> Result<Self, Self::Error> {
            let (r, g, b) = (rgb.r, rgb.g, rgb.b);
            for c in [r, g, b].iter() {
                if *c < 0.0 {
                    return Err("Value < 0.0 found!");
                }
                if *c > 1.0 {
                    return Err("Value > 1.0 found!");
                }
            }
            let (r, g, b) = ((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8);
            let rgb = Color::Rgb { r, g, b };
            Ok(rgb)
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::prelude::RGB;
        use crossterm::style::Color;
        use std::convert::TryInto;

        #[test]
        fn basic_conversion() {
            let rgb = RGB {
                r: 0.0,
                g: 0.5,
                b: 1.0,
            };
            let rgb: Color = rgb.try_into().unwrap();
            match rgb {
                Color::Rgb { r, g, b } => {
                    assert_eq!(r, 0);
                    assert_eq!(g, 127);
                    assert_eq!(b, 255);
                }
                _ => unreachable!(),
            }
        }

        #[test]
        fn negative_rgb() {
            let rgb = RGB {
                r: 0.0,
                g: 0.5,
                b: -1.0,
            };
            let rgb: Result<Color, _> = rgb.try_into();
            assert!(rgb.is_err());
        }

        #[test]
        fn too_large_rgb() {
            let rgb = RGB {
                r: 0.0,
                g: 0.5,
                b: 1.1,
            };
            let rgb: Result<Color, _> = rgb.try_into();
            assert!(rgb.is_err());
        }
    }
}

// Unit tests for the color system

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    // Tests that we make an RGB triplet at defaults and it is black.
    fn make_rgb_minimal() {
        let black = RGB::new();
        assert!(black.r < std::f32::EPSILON);
        assert!(black.g < std::f32::EPSILON);
        assert!(black.b < std::f32::EPSILON);
    }

    #[test]
    // Tests that we make an HSV triplet at defaults and it is black.
    fn convert_olive_to_rgb() {
        let grey = HSV::from_f32(60.0 / 360.0, 1.0, 0.501_960_8);
        let rgb = grey.to_rgb();
        assert!(f32::abs(rgb.r - 128.0 / 255.0) < std::f32::EPSILON);
        assert!(f32::abs(rgb.g - 128.0 / 255.0) < std::f32::EPSILON);
        assert!(rgb.b < std::f32::EPSILON);
    }

    #[test]
    // Tests that we make an HSV triplet at defaults and it is black.
    fn test_red_hex() {
        let rgb = RGB::from_hex("#FF0000").expect("Invalid hex string");
        assert!(f32::abs(rgb.r - 1.0) < std::f32::EPSILON);
        assert!(rgb.g < std::f32::EPSILON);
        assert!(rgb.b < std::f32::EPSILON);
    }

    #[test]
    // Tests that we make an HSV triplet at defaults and it is black.
    fn test_green_hex() {
        let rgb = RGB::from_hex("#00FF00").expect("Invalid hex string");
        assert!(rgb.r < std::f32::EPSILON);
        assert!(f32::abs(rgb.g - 1.0) < std::f32::EPSILON);
        assert!(rgb.b < std::f32::EPSILON);
    }

    #[test]
    // Tests that we make an HSV triplet at defaults and it is black.
    fn test_blue_hex() {
        let rgb = RGB::from_hex("#0000FF").expect("Invalid hex string");
        assert!(rgb.r < std::f32::EPSILON);
        assert!(rgb.g < std::f32::EPSILON);
        assert!(f32::abs(rgb.b - 1.0) < std::f32::EPSILON);
    }

    #[test]
    // Tests that we make an HSV triplet at defaults and it is black.
    fn test_blue_named() {
        let rgb = RGB::named(BLUE);
        assert!(rgb.r < std::f32::EPSILON);
        assert!(rgb.g < std::f32::EPSILON);
        assert!(f32::abs(rgb.b - 1.0) < std::f32::EPSILON);
    }

    #[test]
    // Test the lerp function
    fn test_lerp() {
        let black = RGB::named(BLACK);
        let white = RGB::named(WHITE);
        assert!(black.lerp(white, 0.0) == black);
        assert!(black.lerp(white, 1.0) == white);
    }
}
