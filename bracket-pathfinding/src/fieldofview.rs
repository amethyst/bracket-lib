use bracket_algorithm_traits::prelude::Algorithm2D;
// use bracket_geometry::prelude::{BresenhamCircleNoDiag, Point, VectorLine};
use bracket_geometry::prelude::Point;
use std::collections::HashSet;

struct ScanFovData<'a> {
    center: Point,
    dimensions: Point,
    range_2: i32,
    fov_check: &'a dyn Algorithm2D,
    visible_points: &'a mut HashSet<Point>,
}

#[allow(non_snake_case)]
impl ScanFovData<'_> {
    fn is_transparent(&self, idx: usize, point: Point) -> bool {
        if self.fov_check.in_bounds(point) {
            !self.fov_check.is_opaque(idx)
        } else {
            false
        }
    }

    fn distance_to_center(&self, point: Point) -> f32 {
        let dx = i32::abs(point.x - self.center.x) as f32 - 0.5;
        let dy = i32::abs(point.y - self.center.y) as f32 - 0.5;
        dx * dx + dy * dy
    }

    fn insert_visible_for_vertical(&mut self, point: Point) -> bool {
        let idx = self.fov_check.point2d_to_index(point);
        let mut is_visible = self.is_transparent(idx, point);

        if self.distance_to_center(point) <= self.range_2 as f32 {
            if point.x != self.center.x {
                self.visible_points.insert(point);
            }
        } else {
            is_visible = false;
        }
        is_visible
    }

    fn insert_visible_for_horizontal(&mut self, point: Point) -> bool {
        let idx = self.fov_check.point2d_to_index(point);
        let mut is_visible = self.is_transparent(idx, point);

        if self.distance_to_center(point) <= self.range_2 as f32 {
            if self.center.y != point.y {
                self.visible_points.insert(point);
            }
        } else {
            is_visible = false;
        }
        is_visible
    }

    fn scan_N2NE(&mut self, distance: i32, start_slope: f32, end_slope: f32) {
        let mut start_slope = start_slope;

        if distance * distance > self.range_2 {
            return;
        }

        let mut current = Point::new(0, self.center.y - distance);
        if current.y < 0 {
            return;
        }

        current.x = self.center.x + (start_slope * distance as f32 + 0.5) as i32;
        if current.x >= self.dimensions.x {
            return;
        }

        let mut end_x = self.center.x + (end_slope * distance as f32 + 0.5) as i32;
        if end_x >= self.dimensions.x {
            end_x = self.dimensions.x - 1;
        }

        let idx = self.fov_check.point2d_to_index(current);
        let mut last_visible = self.is_transparent(idx, current);
        for current_x in current.x..=end_x {
            current.x = current_x;

            let is_visible = self.insert_visible_for_vertical(current);

            if last_visible && !is_visible {
                self.scan_N2NE(
                    distance + 1,
                    start_slope,
                    ((current.x - self.center.x) as f32 - 0.5) / (distance as f32 + 0.5),
                );
            } else if !last_visible && is_visible {
                start_slope = ((current.x - self.center.x) as f32 - 0.5) / (distance as f32 - 0.5);
            }
            last_visible = is_visible;
        }
        if last_visible {
            self.scan_N2NE(distance + 1, start_slope, end_slope);
        }
    }

    fn scan_N2NW(&mut self, distance: i32, start_slope: f32, end_slope: f32) {
        let mut start_slope = start_slope;

        if distance * distance > self.range_2 {
            return;
        }

        let mut current = Point::new(0, self.center.y - distance);
        if current.y < 0 {
            return;
        }

        current.x = self.center.x - (start_slope * distance as f32 + 0.5) as i32;
        if current.x < 0 {
            return;
        }

        let mut end_x = self.center.x - (end_slope * distance as f32 + 0.5) as i32;
        if end_x < 0 {
            end_x = 0;
        }

        let idx = self.fov_check.point2d_to_index(current);
        let mut last_visible = self.is_transparent(idx, current);
        while current.x >= end_x {
            let is_visible = self.insert_visible_for_vertical(current);

            if last_visible && !is_visible {
                self.scan_N2NW(
                    distance + 1,
                    start_slope,
                    ((self.center.x - current.x) as f32 - 0.5) / (distance as f32 + 0.5),
                );
            } else if !last_visible && is_visible {
                start_slope = ((self.center.x - current.x) as f32 - 0.5) / (distance as f32 - 0.5);
            }
            last_visible = is_visible;
            current.x -= 1;
        }
        if last_visible {
            self.scan_N2NW(distance + 1, start_slope, end_slope);
        }
    }

    fn scan_S2SE(&mut self, distance: i32, start_slope: f32, end_slope: f32) {
        let mut start_slope = start_slope;

        if distance * distance > self.range_2 {
            return;
        }

        let mut current = Point::new(0, self.center.y + distance);
        if current.y >= self.dimensions.y {
            return;
        }

        current.x = self.center.x + (start_slope * distance as f32 + 0.5) as i32;
        if current.x >= self.dimensions.x {
            return;
        }

        let mut end_x = self.center.x + (end_slope * distance as f32 + 0.5) as i32;
        if end_x >= self.dimensions.x {
            end_x = self.dimensions.x - 1;
        }

        let idx = self.fov_check.point2d_to_index(current);
        let mut last_visible = self.is_transparent(idx, current);
        for current_x in current.x..=end_x {
            current.x = current_x;

            let is_visible = self.insert_visible_for_vertical(current);

            if last_visible && !is_visible {
                self.scan_S2SE(
                    distance + 1,
                    start_slope,
                    ((current.x - self.center.x) as f32 - 0.5) / (distance as f32 + 0.5),
                );
            } else if !last_visible && is_visible {
                start_slope = ((current.x - self.center.x) as f32 - 0.5) / (distance as f32 - 0.5);
            }
            last_visible = is_visible;
        }
        if last_visible {
            self.scan_S2SE(distance + 1, start_slope, end_slope);
        }
    }

    fn scan_S2SW(&mut self, distance: i32, start_slope: f32, end_slope: f32) {
        let mut start_slope = start_slope;

        if distance * distance > self.range_2 {
            return;
        }

        let mut current = Point::new(0, self.center.y + distance);
        if current.y >= self.dimensions.y {
            return;
        }

        current.x = self.center.x - (start_slope * distance as f32 + 0.5) as i32;
        if current.x < 0 {
            return;
        }

        let mut end_x = self.center.x - (end_slope * distance as f32 + 0.5) as i32;
        if end_x < 0 {
            end_x = 0;
        }

        let idx = self.fov_check.point2d_to_index(current);
        let mut last_visible = self.is_transparent(idx, current);
        while current.x >= end_x {
            let is_visible = self.insert_visible_for_vertical(current);

            if last_visible && !is_visible {
                self.scan_S2SW(
                    distance + 1,
                    start_slope,
                    ((self.center.x - current.x) as f32 - 0.5) / (distance as f32 + 0.5),
                );
            } else if !last_visible && is_visible {
                start_slope = ((self.center.x - current.x) as f32 - 0.5) / (distance as f32 - 0.5);
            }
            last_visible = is_visible;
            current.x -= 1;
        }
        if last_visible {
            self.scan_S2SW(distance + 1, start_slope, end_slope);
        }
    }

    fn scan_E2SE(&mut self, distance: i32, start_slope: f32, end_slope: f32) {
        let mut start_slope = start_slope;

        if distance * distance > self.range_2 {
            return;
        }

        let mut current = Point::new(self.center.x + distance, 0);
        if current.x >= self.dimensions.x {
            return;
        }

        current.y = self.center.y + (start_slope * distance as f32 + 0.5) as i32;
        if current.y >= self.dimensions.y {
            return;
        }

        let mut end_y = self.center.y + (end_slope * distance as f32 + 0.5) as i32;
        if end_y >= self.dimensions.y {
            end_y = self.dimensions.y - 1;
        }

        let idx = self.fov_check.point2d_to_index(current);
        let mut last_visible = self.is_transparent(idx, current);
        for current_y in current.y..=end_y {
            current.y = current_y;

            let is_visible = self.insert_visible_for_horizontal(current);

            if last_visible && !is_visible {
                self.scan_E2SE(
                    distance + 1,
                    start_slope,
                    ((current.y - self.center.y) as f32 - 0.5) / (distance as f32 + 0.5),
                );
            } else if !last_visible && is_visible {
                start_slope = ((current.y - self.center.y) as f32 - 0.5) / (distance as f32 - 0.5);
            }
            last_visible = is_visible;
        }
        if last_visible {
            self.scan_E2SE(distance + 1, start_slope, end_slope);
        }
    }

    fn scan_E2NE(&mut self, distance: i32, start_slope: f32, end_slope: f32) {
        let mut start_slope = start_slope;

        if distance * distance > self.range_2 {
            return;
        }

        let mut current = Point::new(self.center.x + distance, 0);
        if current.x >= self.dimensions.x {
            return;
        }

        current.y = self.center.y - (start_slope * distance as f32 + 0.5) as i32;
        if current.y < 0 {
            return;
        }

        let mut end_y = self.center.y - (end_slope * distance as f32 + 0.5) as i32;
        if end_y < 0 {
            end_y = 0;
        }

        let idx = self.fov_check.point2d_to_index(current);
        let mut last_visible = self.is_transparent(idx, current);
        while current.y >= end_y {
            let is_visible = self.insert_visible_for_horizontal(current);

            if last_visible && !is_visible {
                self.scan_E2NE(
                    distance + 1,
                    start_slope,
                    ((self.center.y - current.y) as f32 - 0.5) / (distance as f32 + 0.5),
                );
            } else if !last_visible && is_visible {
                start_slope = ((self.center.y - current.y) as f32 - 0.5) / (distance as f32 - 0.5);
            }
            last_visible = is_visible;
            current.y -= 1;
        }
        if last_visible {
            self.scan_E2NE(distance + 1, start_slope, end_slope);
        }
    }

    fn scan_W2SW(&mut self, distance: i32, start_slope: f32, end_slope: f32) {
        let mut start_slope = start_slope;

        if distance * distance > self.range_2 {
            return;
        }

        let mut current = Point::new(self.center.x - distance, 0);
        if current.x < 0 {
            return;
        }

        current.y = self.center.y + (start_slope * distance as f32 + 0.5) as i32;
        if current.y >= self.dimensions.y {
            return;
        }

        let mut end_y = self.center.y + (end_slope * distance as f32 + 0.5) as i32;
        if end_y >= self.dimensions.y {
            end_y = self.dimensions.y - 1;
        }

        let idx = self.fov_check.point2d_to_index(current);
        let mut last_visible = self.is_transparent(idx, current);
        for current_y in current.y..=end_y {
            current.y = current_y;

            let is_visible = self.insert_visible_for_horizontal(current);

            if last_visible && !is_visible {
                self.scan_W2SW(
                    distance + 1,
                    start_slope,
                    ((current.y - self.center.y) as f32 - 0.5) / (distance as f32 + 0.5),
                );
            } else if !last_visible && is_visible {
                start_slope = ((current.y - self.center.y) as f32 - 0.5) / (distance as f32 - 0.5);
            }
            last_visible = is_visible;
        }
        if last_visible {
            self.scan_W2SW(distance + 1, start_slope, end_slope);
        }
    }

    fn scan_W2NW(&mut self, distance: i32, start_slope: f32, end_slope: f32) {
        let mut start_slope = start_slope;

        if distance * distance > self.range_2 {
            return;
        }

        let mut current = Point::new(self.center.x - distance, 0);
        if current.x < 0 {
            return;
        }

        current.y = self.center.y - (start_slope * distance as f32 + 0.5) as i32;
        if current.y < 0 {
            return;
        }

        let mut end_y = self.center.y - (end_slope * distance as f32 + 0.5) as i32;
        if end_y < 0 {
            end_y = 0;
        }

        let idx = self.fov_check.point2d_to_index(current);
        let mut last_visible = self.is_transparent(idx, current);
        while current.y >= end_y {
            let is_visible = self.insert_visible_for_horizontal(current);

            if last_visible && !is_visible {
                self.scan_W2NW(
                    distance + 1,
                    start_slope,
                    ((self.center.y - current.y) as f32 - 0.5) / (distance as f32 + 0.5),
                );
            } else if !last_visible && is_visible {
                start_slope = ((self.center.y - current.y) as f32 - 0.5) / (distance as f32 - 0.5);
            }
            last_visible = is_visible;
            current.y -= 1;
        }
        if last_visible {
            self.scan_W2NW(distance + 1, start_slope, end_slope);
        }
    }
}

/// Calculates field-of-view for a map that supports Algorithm2D, returning a HashSet. This is a bit faster
/// than coercing the results into a vector, since internally it uses the set for de-duplication.
#[rustfmt::skip]
pub fn field_of_view_set(center: Point, range: i32, fov_check: &dyn Algorithm2D) -> HashSet<Point> {
    let mut visible_points: HashSet<Point> =
        HashSet::with_capacity(((range * 2) * (range * 2)) as usize);

    visible_points.insert(center);

    /* N, NE, E, SE, S, SW, W, NW */
    const SECTORS: [(i32, i32); 8] = [ (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), ];

    let r2 = range * range;
    let dimensions = fov_check.dimensions();

    // Add visibility for every 45 degree line:
    let mut visibility_per_sector = [false; 8];
    for (i, (dx, dy)) in SECTORS.iter().enumerate() {
        let mut current = center;
        loop {
            current = Point::new(current.x + dx, current.y + dy);
            if current.x < 0 || current.x >= dimensions.x
                || current.y < 0 || current.y >= dimensions.y
            {
                break;
            }
            let x2 = current.x - center.x;
            let x2 = x2 * x2;
            let y2 = current.y - center.y;
            let y2 = y2 * y2;
            if x2 + y2 > r2 {
                break;
            }

            let idx = fov_check.point2d_to_index(current);
            visible_points.insert(current);
            if fov_check.is_opaque(idx) {
                break;
            }
            visibility_per_sector[i] = true;
        }
    }

    let mut scanner = ScanFovData {
        center,
        dimensions,
        range_2: r2,
        fov_check,
        visible_points: &mut visible_points,
    };
    if visibility_per_sector[0] {
        scanner.scan_N2NW(1, 0., 1.);
        scanner.scan_N2NE(1, 0., 1.);
    }

    if visibility_per_sector[2] {
        scanner.scan_E2NE(1, 0., 1.);
        scanner.scan_E2SE(1, 0., 1.);
    }

    if visibility_per_sector[4] {
        scanner.scan_S2SE(1, 0., 1.);
        scanner.scan_S2SW(1, 0., 1.);
    }

    if visibility_per_sector[6] {
        scanner.scan_W2SW(1, 0., 1.);
        scanner.scan_W2NW(1, 0., 1.);
    }

    visible_points
        .iter()
        .map(|p| *p)
        .filter(|p| fov_check.in_bounds(*p))
        .collect()
}

/// Calculates field-of-view for a map that supports Algorithm2D.
pub fn field_of_view(start: Point, range: i32, fov_check: &dyn Algorithm2D) -> Vec<Point> {
    field_of_view_set(start, range, fov_check)
        .into_iter()
        .collect()
}

#[cfg(test)]
mod tests {

    use crate::prelude::*;
    use bracket_algorithm_traits::prelude::{Algorithm2D, BaseMap};
    use bracket_geometry::prelude::{BresenhamCircle, Point};
    use std::cmp::max;
    use std::collections::HashSet;
    use std::hash::Hash;

    const TESTMAP_W: usize = 20;
    const TESTMAP_H: usize = 20;
    const TESTMAP_TILES: usize = (TESTMAP_W * TESTMAP_H) as usize;

    struct Map {
        pub tiles: Vec<bool>,
    }

    impl Map {
        fn new() -> Map {
            Map {
                tiles: vec![false; TESTMAP_TILES],
            }
        }
    }

    // The map needs to be see-through for the tests to check FOV
    impl BaseMap for Map {
        fn is_opaque(&self, idx: usize) -> bool {
            self.tiles[idx]
        }
    }

    impl Algorithm2D for Map {
        fn dimensions(&self) -> Point {
            Point::new(TESTMAP_W, TESTMAP_H)
        }
    }

    fn has_unique_elements<T>(iter: T) -> bool
    where
        T: IntoIterator,
        T::Item: Eq + Hash,
    {
        let mut uniq = HashSet::new();
        iter.into_iter().all(move |x| uniq.insert(x))
    }

    // Tests that we are correctly de-duplicating field of view
    #[test]
    fn fov_dupes() {
        let map = Map::new();

        let visible = field_of_view(Point::new(10, 10), 8, &map);

        assert!(has_unique_elements(&visible));
    }

    // Tests that the bounds-checking trait is applying properly to field-of-view checks
    #[test]
    fn fov_bounds_check() {
        let map = Map::new();

        let visible = field_of_view(Point::new(2, 2), 8, &map);

        for t in visible.iter() {
            assert!(t.x >= 0);
            assert!(t.x < TESTMAP_W as i32 - 1);
            assert!(t.y >= 0);
            assert!(t.y < TESTMAP_H as i32 - 1);
        }
    }

    // Tests that the FOV scan does not miss any interior points
    #[test]
    fn fov_inclusive() {
        for radius in 4..=9 {
            let map = Map::new();
            let dimensions = map.dimensions();
            let c = Point::new(10, 10);
            let visible = field_of_view(c, radius, &map);
            // let max_radius_sq: i32 = visible.iter().fold(0, |max_r2, p| {
            let max_radius_sq: i32 = BresenhamCircle::new(c, radius).fold(0, |max_r2, p| {
                let r2 = (p.x - c.x) * (p.x - c.x) + (p.y - c.y) * (p.y - c.y);
                max(r2, max_r2)
            });
            /*
            for y in 0..dimensions.y {
                let mut s = "".to_string();
                for x in 0..dimensions.x {
                    let point = Point::new(x, y);
                    let c = if visible.contains(&point) {
                        '.'
                    } else {
                        '#'
                    };
                    s.push(c);
                }
                println!("{}", s);
            }
            */
            for x in 0..dimensions.x {
                for y in 0..dimensions.y {
                    let r2 = (x - c.x) * (x - c.x) + (y - c.y) * (y - c.y);
                    let point = Point::new(x, y);
                    assert!(
                        r2 >= max_radius_sq || visible.contains(&point),
                        format!("Interior point ({:?}) not in FOV({})", point, radius)
                    );
                }
            }
        }
    }

    #[test]
    fn fov_corridor() {
        let mut map = Map::new();
        let c = Point::new(10, 10);
        let radius: i32 = 5;

        for i in 0..20 {
            let idx = 9 * 20 + i;
            map.tiles[idx] = true;
            let idx = 11 * 20 + i;
            map.tiles[idx] = true;
        }

        let visible = field_of_view(c, radius, &map);
        for i in 1..radius * 2 - 2 {
            let pos = Point::new(c.x - radius + i, c.y);
            assert!(visible.contains(&pos));
            let pos = Point::new(c.x - radius + i, c.y - 1);
            assert!(visible.contains(&pos), format!("{:?} not in result", pos));
            let pos = Point::new(c.x - radius + i, c.y + 1);
            assert!(visible.contains(&pos));
        }
    }
}
