use bracket_algorithm_traits::prelude::Algorithm2D;
use bracket_geometry::prelude::Point;

use std::collections::HashSet;

mod recursive_shadowcasting;
// Default algorithm / backwards compatibility
pub use recursive_shadowcasting::{field_of_view, field_of_view_set};

/// Enumeration of available FOV algorithms
#[derive(Clone, Copy)]
#[non_exhaustive] // Other algorithms may be added in the future
pub enum FieldOfViewAlg {
    RecursiveShadowcasting,
}

impl FieldOfViewAlg {
    pub fn field_of_view_set(
        self,
        center: Point,
        range: i32,
        fov_check: &dyn Algorithm2D,
    ) -> HashSet<Point> {
        match self {
            FieldOfViewAlg::RecursiveShadowcasting => {
                recursive_shadowcasting::field_of_view_set(center, range, fov_check)
            }
        }
    }
    pub fn field_of_view(
        self,
        start: Point,
        range: i32,
        fov_check: &dyn Algorithm2D,
    ) -> Vec<Point> {
        match self {
            FieldOfViewAlg::RecursiveShadowcasting => {
                recursive_shadowcasting::field_of_view(start, range, fov_check)
            }
        }
    }
}
