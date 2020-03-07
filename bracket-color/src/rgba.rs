use super::rgb::{HtmlColorConversionError, RGB};
#[cfg(feature = "rex")]
use crate::prelude::XpColor;
use std::convert::From;
use std::ops;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Copy, Clone, Default, Debug)]
/// Represents an R/G/B triplet, in the range 0..1 (32-bit float)
pub struct RGBA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

// Implement operator overloading

/// Support adding a float to a color. The result is clamped via the constructor.
impl ops::Add<f32> for RGBA {
    type Output = Self;
    #[must_use]
    fn add(mut self, rhs: f32) -> Self {
        self.r += rhs;
        self.g += rhs;
        self.b += rhs;
        self.a += rhs;
        self
    }
}

/// Support adding an RGB to a color. The result is clamped via the constructor.
impl ops::Add<RGBA> for RGBA {
    type Output = Self;
    #[must_use]
    fn add(mut self, rhs: Self) -> Self {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
        self.a += rhs.a;
        self
    }
}

/// Support subtracting a float from a color. The result is clamped via the constructor.
impl ops::Sub<f32> for RGBA {
    type Output = Self;
    #[must_use]
    fn sub(mut self, rhs: f32) -> Self {
        self.r -= rhs;
        self.g -= rhs;
        self.b -= rhs;
        self.a -= rhs;
        self
    }
}

/// Support subtracting an RGB from a color. The result is clamped via the constructor.
impl ops::Sub<RGBA> for RGBA {
    type Output = Self;
    #[must_use]
    fn sub(mut self, rhs: Self) -> Self {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
        self.a -= rhs.a;
        self
    }
}

/// Support multiplying a color by a float. The result is clamped via the constructor.
impl ops::Mul<f32> for RGBA {
    type Output = Self;
    #[must_use]
    fn mul(mut self, rhs: f32) -> Self {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
        self.a *= rhs;
        self
    }
}

/// Support multiplying a color by another color. The result is clamped via the constructor.
impl ops::Mul<RGBA> for RGBA {
    type Output = Self;
    #[must_use]
    fn mul(mut self, rhs: Self) -> Self {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
        self.a *= rhs.a;
        self
    }
}

impl RGBA {
    /// Constructs a new, zeroed (black) RGB triplet.
    #[must_use]
    pub fn new() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
        }
    }

    /// Constructs a new RGB color, from 3 32-bit floats in the range 0..1
    #[inline]
    #[must_use]
    pub fn from_f32(r: f32, g: f32, b: f32, a: f32) -> Self {
        let r_clamped = f32::min(1.0, f32::max(0.0, r));
        let g_clamped = f32::min(1.0, f32::max(0.0, g));
        let b_clamped = f32::min(1.0, f32::max(0.0, b));
        let a_clamped = f32::min(1.0, f32::max(0.0, a));
        Self {
            r: r_clamped,
            g: g_clamped,
            b: b_clamped,
            a: a_clamped,
        }
    }

    /// Constructs a new RGB color, from 3 bytes in the range 0..255
    #[inline]
    #[must_use]
    pub fn from_u8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: f32::from(r) / 255.0,
            g: f32::from(g) / 255.0,
            b: f32::from(b) / 255.0,
            a: f32::from(a) / 255.0,
        }
    }

    /// Construct an RGB color from a tuple of u8, or a named constant
    #[inline]
    #[must_use]
    pub fn named(col: (u8, u8, u8)) -> Self {
        Self::from_u8(col.0, col.1, col.2, 255)
    }

    /// Constructs from an HTML color code (e.g. "#eeffeeff")
    ///
    /// # Errors
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

        let alpha1 = match full_code.next() {
            Some(alpha) => match alpha.to_digit(16) {
                Some(alpha) => alpha * 16,
                None => return Err(HtmlColorConversionError::InvalidCharacter),
            },
            None => return Err(HtmlColorConversionError::InvalidStringLength),
        };
        let alpha2 = match full_code.next() {
            Some(alpha) => match alpha.to_digit(16) {
                Some(alpha) => alpha,
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
            a: (alpha1 + alpha2) as f32 / 255.0,
        })
    }

    /// Converts to an RGB, dropping the alpha component
    #[inline]
    #[must_use]
    pub fn to_rgb(&self) -> RGB {
        RGB::from_f32(self.r, self.g, self.b)
    }

    /// Applies a quick grayscale conversion to the color
    #[inline]
    #[must_use]
    pub fn to_greyscale(&self) -> Self {
        let linear = (self.r * 0.2126) + (self.g * 0.7152) + (self.b * 0.0722);
        Self::from_f32(linear, linear, linear, self.a)
    }

    /// Applies a lengthier desaturate (via HSV) to the color
    #[inline]
    #[must_use]
    pub fn desaturate(&self) -> Self {
        let mut hsv = self.to_rgb().to_hsv();
        hsv.s = 0.0;
        hsv.to_rgb().to_rgba(self.a)
    }

    /// Lerps by a specified percentage (from 0 to 1) between this color and another
    #[inline]
    #[must_use]
    pub fn lerp(&self, color: Self, percent: f32) -> Self {
        let range = (
            color.r - self.r,
            color.g - self.g,
            color.b - self.b,
            color.a - self.a,
        );
        Self {
            r: self.r + range.0 * percent,
            g: self.g + range.1 * percent,
            b: self.b + range.2 * percent,
            a: self.a + range.3 * percent,
        }
    }

    /// Lerps only the alpha channel, by a specified percentage (from 0 to 1) between this color and another
    #[inline]
    #[must_use]
    pub fn lerp_alpha(&self, color: Self, percent: f32) -> Self {
        let range = color.a - self.a;
        Self {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a + range * percent,
        }
    }

    /// Converts an RGB to an xp file color component
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    #[cfg(feature = "rex")]
    #[must_use]
    pub fn to_xp(&self) -> XpColor {
        XpColor::new(
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
        )
    }
}

impl From<RGB> for RGBA {
    fn from(item: RGB) -> Self {
        Self::from_f32(item.r, item.g, item.b, 1.0)
    }
}
