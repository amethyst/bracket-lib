use crate::prelude::RGB;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Copy, Clone, Default, Debug)]
/// Represents an H/S/V triplet, in the range 0..1 (32-bit float)
pub struct HSV {
    pub h: f32,
    pub s: f32,
    pub v: f32,
}

impl HSV {
    /// Constructs a new, zeroed (black) HSV triplet.
    #[must_use]
    pub fn new() -> Self {
        Self {
            h: 0.0,
            s: 0.0,
            v: 0.0,
        }
    }

    /// Constructs a new HSV color, from 3 32-bit floats
    #[inline]
    #[must_use]
    pub const fn from_f32(h: f32, s: f32, v: f32) -> Self {
        Self { h, s, v }
    }

    /// Converts an HSV triple to an RGB triple
    #[allow(clippy::many_single_char_names)] // I like my short names for this one
    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::cast_possible_truncation)]
    #[inline]
    #[must_use]
    pub fn to_rgb(&self) -> RGB {
        let h = self.h;
        let s = self.s;
        let v = self.v;

        let mut r: f32 = 0.0;
        let mut g: f32 = 0.0;
        let mut b: f32 = 0.0;

        let i = f32::floor(h * 6.0) as i32;
        let f = h * 6.0 - i as f32;
        let p = v * (1.0 - s);
        let q = v * (1.0 - f * s);
        let t = v * (1.0 - (1.0 - f) * s);

        match i % 6 {
            0 => {
                r = v;
                g = t;
                b = p;
            }
            1 => {
                r = q;
                g = v;
                b = p;
            }
            2 => {
                r = p;
                g = v;
                b = t;
            }
            3 => {
                r = p;
                g = q;
                b = v;
            }
            4 => {
                r = t;
                g = p;
                b = v;
            }
            5 => {
                r = v;
                g = p;
                b = q;
            }
            _ => {}
        }

        RGB::from_f32(r, g, b)
    }

    #[inline]
    #[must_use]
    pub fn lerp(&self, color: Self, percent: f32) -> Self {
        let range = (color.h - self.h, color.s - self.s, color.v - self.v);
        Self {
            h: self.h + range.0 * percent,
            s: self.s + range.1 * percent,
            v: self.v + range.2 * percent,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    // Tests that we make an HSV triplet at defaults and it is black.
    fn make_hsv_minimal() {
        let black = HSV::new();
        assert!(black.h < std::f32::EPSILON);
        assert!(black.s < std::f32::EPSILON);
        assert!(black.v < std::f32::EPSILON);
    }

    #[test]
    // Tests that we make an HSV triplet at defaults and it is black.
    fn convert_red_to_hsv() {
        let red = RGB::from_f32(1.0, 0.0, 0.0);
        let hsv = red.to_hsv();
        assert!(hsv.h < std::f32::EPSILON);
        assert!(f32::abs(hsv.s - 1.0) < std::f32::EPSILON);
        assert!(f32::abs(hsv.v - 1.0) < std::f32::EPSILON);
    }

    #[test]
    // Tests that we make an HSV triplet at defaults and it is black.
    fn convert_green_to_hsv() {
        let green = RGB::from_f32(0.0, 1.0, 0.0);
        let hsv = green.to_hsv();
        assert!(f32::abs(hsv.h - 120.0 / 360.0) < std::f32::EPSILON);
        assert!(f32::abs(hsv.s - 1.0) < std::f32::EPSILON);
        assert!(f32::abs(hsv.v - 1.0) < std::f32::EPSILON);
    }

    #[test]
    // Tests that we make an HSV triplet at defaults and it is black.
    fn convert_blue_to_hsv() {
        let blue = RGB::from_f32(0.0, 0.0, 1.0);
        let hsv = blue.to_hsv();
        assert!(f32::abs(hsv.h - 240.0 / 360.0) < std::f32::EPSILON);
        assert!(f32::abs(hsv.s - 1.0) < std::f32::EPSILON);
        assert!(f32::abs(hsv.v - 1.0) < std::f32::EPSILON);
    }

    #[test]
    // Tests that we make an HSV triplet at defaults and it is black.
    fn convert_olive_to_hsv() {
        let grey = RGB::from_u8(128, 128, 0);
        let hsv = grey.to_hsv();
        assert!(f32::abs(hsv.h - 60.0 / 360.0) < std::f32::EPSILON);
        assert!(f32::abs(hsv.s - 1.0) < std::f32::EPSILON);
        assert!(f32::abs(hsv.v - 0.5019_608) < std::f32::EPSILON);
    }
}
