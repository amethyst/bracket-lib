/// Convenience type: you can define an angle in Degrees and it is convertible to Radians
/// (and vice versa)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, Debug)]
pub struct Degrees(pub f32);

/// Convenience type: you can define an angle in Radians and it is convertible to Degrees
/// (and vice versa)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, Debug)]
pub struct Radians(pub f32);

impl Degrees {
    pub fn new(angle: f32) -> Self {
        Self(angle)
    }
}

impl Radians {
    pub fn new(angle: f32) -> Self {
        Self(angle)
    }
}

impl From<Radians> for Degrees {
    fn from(item : Radians) -> Self {
        Self(item.0 * 57.2958)
    }
}

impl From<Degrees> for Radians {
    fn from(item : Degrees) -> Self {
        Self(item.0 * 0.017_453_3)
    }
}