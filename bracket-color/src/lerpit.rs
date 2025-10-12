use crate::prelude::{HSV, RGB, RGBA};
use core::iter::{ExactSizeIterator, Iterator};
use std::convert::TryInto;

/// Implements an RGB Lerp as an iterator
pub struct RgbLerp {
    /// Starting color
    start: RGB,
    /// Ending color
    end: RGB,
    /// Number of lerp steps
    n_steps: usize,
    /// Current step (modified by the iterator)
    step: usize,
}

impl RgbLerp {
    /// Creates a new RGB lerp iterator. The iterator smoothly transitions between two colors,
    /// using the specified number of steps.
    ///
    /// # Arguments
    ///
    /// * `start` - the color to start from.
    /// * `end` - the color to end at on the final step.
    /// * `steps` - number of steps to iterate between the start and end colors.
    ///
    /// # Example
    ///
    /// ```rust
    /// use bracket_color::prelude::*;
    /// for color in RgbLerp::new(RGB::named(RED), RGB::named(YELLOW), 20) {
    ///     println!("{:?}", color); // In-between color
    /// }
    /// ```
    #[inline]
    pub fn new<T>(start: RGB, end: RGB, steps: T) -> Self
    where
        T: TryInto<usize>,
    {
        Self {
            start,
            end,
            n_steps: steps
                .try_into()
                .ok()
                .expect("Not a usize-convertible integer"),
            step: 0,
        }
    }
}

impl Iterator for RgbLerp {
    type Item = RGB;

    /// Returns the next step in the iterator
    #[inline]
    #[allow(clippy::cast_precision_loss)]
    fn next(&mut self) -> Option<RGB> {
        if self.step > self.n_steps {
            None
        } else {
            let percent = self.step as f32 / self.n_steps as f32;
            self.step += 1;

            Some(self.start.lerp(self.end, percent))
        }
    }
}

impl ExactSizeIterator for RgbLerp {
    /// Returns the `n_steps` component of the iterator
    #[inline]
    #[must_use]
    fn len(&self) -> usize {
        self.n_steps
    }
}

/// An HSV Lerp - transition from one HSV color to another in a set number of steps.
pub struct HsvLerp {
    /// The starting color
    start: HSV,
    /// The ending color
    end: HSV,
    /// The number of steps to use
    n_steps: usize,
    /// The current step (modified by the iterator)
    step: usize,
}

impl HsvLerp {
    /// Creates a new `HsvLerp` iterator.
    #[inline]
    pub fn new<T>(start: HSV, end: HSV, steps: T) -> Self
    where
        T: TryInto<usize>,
    {
        Self {
            start,
            end,
            n_steps: steps.try_into().ok().expect("Not an integer"),
            step: 0,
        }
    }
}

impl Iterator for HsvLerp {
    type Item = HSV;

    /// Returns the next Lerp step
    #[inline]
    #[allow(clippy::cast_precision_loss)]
    fn next(&mut self) -> Option<HSV> {
        if self.step > self.n_steps {
            None
        } else {
            let percent = self.step as f32 / self.n_steps as f32;
            self.step += 1;

            Some(self.start.lerp(self.end, percent))
        }
    }
}

impl ExactSizeIterator for HsvLerp {
    #[inline]
    #[must_use]
    fn len(&self) -> usize {
        self.n_steps
    }
}

/// Implements an RGBA Lerp as an iterator
pub struct RgbaLerp {
    /// Starting color
    start: RGBA,
    /// Ending color
    end: RGBA,
    /// Number of lerp steps
    n_steps: usize,
    /// Current step (modified by the iterator)
    step: usize,
}

impl RgbaLerp {
    /// Creates a new RGB iterator
    #[inline]
    pub fn new<T>(start: RGBA, end: RGBA, steps: T) -> Self
    where
        T: TryInto<usize>,
    {
        Self {
            start,
            end,
            n_steps: steps
                .try_into()
                .ok()
                .expect("Not a usize-convertible integer"),
            step: 0,
        }
    }
}

impl Iterator for RgbaLerp {
    type Item = RGBA;

    /// Returns the next step in the iterator
    #[inline]
    #[allow(clippy::cast_precision_loss)]
    fn next(&mut self) -> Option<RGBA> {
        if self.step > self.n_steps {
            None
        } else {
            let percent = self.step as f32 / self.n_steps as f32;
            self.step += 1;

            Some(self.start.lerp(self.end, percent))
        }
    }
}

/// Implements an Alpha-Only Lerp as an iterator
pub struct AlphaLerp {
    /// Starting color
    start: RGBA,
    /// Ending color
    end: RGBA,
    /// Number of lerp steps
    n_steps: usize,
    /// Current step (modified by the iterator)
    step: usize,
}

impl AlphaLerp {
    /// Creates a new RGB iterator
    #[inline]
    pub fn new<T>(start: RGBA, end: RGBA, steps: T) -> Self
    where
        T: TryInto<usize>,
    {
        Self {
            start,
            end,
            n_steps: steps
                .try_into()
                .ok()
                .expect("Not a usize-convertible integer"),
            step: 0,
        }
    }
}

impl Iterator for AlphaLerp {
    type Item = RGBA;

    /// Returns the next step in the iterator
    #[inline]
    #[allow(clippy::cast_precision_loss)]
    fn next(&mut self) -> Option<RGBA> {
        if self.step > self.n_steps {
            None
        } else {
            let percent = self.step as f32 / self.n_steps as f32;
            self.step += 1;

            Some(self.start.lerp_alpha(self.end, percent))
        }
    }
}
