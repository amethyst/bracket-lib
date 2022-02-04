use bracket_algorithm_traits::prelude::BaseMap;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::convert::TryInto;

/// Bail out if the A* search exceeds this many steps.
const MAX_ASTAR_STEPS: usize = 65536;

/// Request an A-Star search. The start and end are specified as index numbers (compatible with your
/// BaseMap implementation), and it requires access to your map so as to call distance and exit determinations.
pub fn a_star_search<T>(start: T, end: T, map: &dyn BaseMap) -> NavigationPath
where
    T: TryInto<usize>,
{
    AStar::new(start.try_into().ok().unwrap(), end.try_into().ok().unwrap()).search(map)
}

/// Holds the result of an A-Star navigation query.
/// `destination` is the index of the target tile.
/// `success` is true if it reached the target, false otherwise.
/// `steps` is a vector of each step towards the target, *including* the starting position.
#[derive(Clone, Default, Debug)]
pub struct NavigationPath {
    pub destination: usize,
    pub success: bool,
    pub steps: Vec<usize>,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
/// Node is an internal step inside the A-Star path (not exposed/public). Idx is the current cell,
/// f is the total cost, g the neighbor cost, and h the heuristic cost.
/// See: https://en.wikipedia.org/wiki/A*_search_algorithm
struct Node {
    idx: usize,
    f: f32,
    g: f32,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.f == other.f
    }
}

impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, b: &Self) -> Ordering {
        b.f.partial_cmp(&self.f).unwrap()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, b: &Self) -> Option<Ordering> {
        b.f.partial_cmp(&self.f)
    }
}

impl NavigationPath {
    /// Makes a new (empty) NavigationPath
    pub fn new() -> NavigationPath {
        NavigationPath {
            destination: 0,
            success: false,
            steps: Vec::new(),
        }
    }
}

/// Private structure for calculating an A-Star navigation path.
struct AStar {
    start: usize,
    end: usize,
    open_list: BinaryHeap<Node>,
    closed_list: HashMap<usize, f32>,
    parents: HashMap<usize, (usize, f32)>, // (index, cost)
    step_counter: usize,
}

impl AStar {
    /// Creates a new path, with specified starting and ending indices.
    fn new(start: usize, end: usize) -> AStar {
        let mut open_list: BinaryHeap<Node> = BinaryHeap::new();
        open_list.push(Node {
            idx: start,
            f: 0.0,
            g: 0.0,
        });

        AStar {
            start,
            end,
            open_list,
            parents: HashMap::new(),
            closed_list: HashMap::new(),
            step_counter: 0,
        }
    }

    /// Wrapper to the BaseMap's distance function.
    fn distance_to_end(&self, idx: usize, map: &dyn BaseMap) -> f32 {
        map.get_pathing_distance(idx, self.end)
    }

    /// Adds a successor; if we're at the end, marks success.
    fn add_successor(&mut self, q: Node, idx: usize, cost: f32, map: &dyn BaseMap) {
        let distance = self.distance_to_end(idx, map);
        let s = Node {
            idx,
            f: distance + cost,
            g: cost,
        };

        // If a node with the same position as successor is in the open list with a lower f, skip add
        let mut should_add = true;
        if let Some(e) = self.parents.get(&idx) {
            if e.1 < s.f {
                should_add = false;
            }
        }

        // If a node with the same position as successor is in the closed list, with a lower f, skip add
        if should_add && self.closed_list.contains_key(&idx) {
            should_add = false;
        }

        if should_add {
            self.open_list.push(s);
            self.parents.insert(idx, (q.idx, q.f));
        }
    }

    /// Helper function to unwrap a path once we've found the end-point.
    fn found_it(&self) -> NavigationPath {
        let mut result = NavigationPath::new();
        result.success = true;
        result.destination = self.end;

        result.steps.push(self.end);
        let mut current = self.end;
        while current != self.start {
            let parent = self.parents[&current];
            result.steps.insert(0, parent.0);
            current = parent.0;
        }

        result
    }

    /// Performs an A-Star search
    fn search(&mut self, map: &dyn BaseMap) -> NavigationPath {
        let result = NavigationPath::new();
        while !self.open_list.is_empty() && self.step_counter < MAX_ASTAR_STEPS {
            self.step_counter += 1;

            // Pop Q off of the list
            let q = self.open_list.pop().unwrap();
            if q.idx == self.end {
                let success = self.found_it();
                return success;
            }

            // Generate successors
            map.get_available_exits(q.idx)
                .iter()
                .for_each(|s| self.add_successor(q, s.0, s.1 + q.f, map));

            if self.closed_list.contains_key(&q.idx) {
                self.closed_list.remove(&q.idx);
            }
            self.closed_list.insert(q.idx, q.f);
        }
        result
    }
}

#[cfg(test)]
mod test {
    use bracket_geometry::prelude::{Point, DistanceAlg};
    use bracket_algorithm_traits::prelude::{BaseMap, Algorithm2D};
    use smallvec::SmallVec;
    use crate::astar::a_star_search;

    pub const MAP_WIDTH: usize = 80;
    pub const MAP_HEIGHT: usize = 20;
    pub const MAP_TILES: usize = MAP_WIDTH * MAP_HEIGHT;
    pub const START_POINT: Point = Point::constant(0, 0);
    pub const END_POINT: Point = Point::constant(2, 2);

    pub struct Map {
        pub tiles: Vec<char>,
    }

    impl Map {
        pub fn new(walls: Vec<Point>) -> Self {
            let mut tiles = Self {
                tiles: vec!['.'; MAP_TILES],
            };

            for point in walls {
                let idx = tiles.point2d_to_index(point);
                tiles.tiles[idx] = '#';
            }

            tiles
        }

        fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
            let destination = loc + delta;

            if destination.x < 0 || destination.y < 0 {
                return None
            }

            let idx = self.point2d_to_index(destination);
            if self.in_bounds(destination) && self.tiles[idx] == '.' {
                Some(idx)
            } else {
                None
            }
        }
    }

    impl BaseMap for Map {
        fn is_opaque(&self, idx: usize) -> bool {
            self.tiles[idx as usize] == '#'
        }

        fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
            let mut exits = SmallVec::new();
            let location = self.index_to_point2d(idx);

            if let Some(idx) = self.valid_exit(location, Point::new(-1, 0)) {
                exits.push((idx, 1.0))
            }
            if let Some(idx) = self.valid_exit(location, Point::new(1, 0)) {
                exits.push((idx, 1.0))
            }
            if let Some(idx) = self.valid_exit(location, Point::new(0, -1)) {
                exits.push((idx, 1.0))
            }
            if let Some(idx) = self.valid_exit(location, Point::new(0, 1)) {
                exits.push((idx, 1.0))
            }

            if let Some(idx) = self.valid_exit(location, Point::new(-1, -1)) {
                exits.push((idx, 1.4))
            }
            if let Some(idx) = self.valid_exit(location, Point::new(1, -1)) {
                exits.push((idx, 1.4))
            }
            if let Some(idx) = self.valid_exit(location, Point::new(-1, 1)) {
                exits.push((idx, 1.4))
            }
            if let Some(idx) = self.valid_exit(location, Point::new(1, 1)) {
                exits.push((idx, 1.4))
            }

            exits
        }

        fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
            DistanceAlg::Pythagoras
                .distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
        }
    }

    impl Algorithm2D for Map {
        fn dimensions(&self) -> Point {
            Point::new(MAP_WIDTH, MAP_HEIGHT)
        }
    }

    #[test]
    fn test_no_wall_exit() {
        let map = Map::new(vec![]);

        let path = a_star_search(
            map.point2d_to_index(START_POINT),
            map.point2d_to_index(END_POINT),
            &map);
        let steps: Vec<Point> = path.steps.iter().map(|e| map.index_to_point2d(*e)).collect();
        assert!(path.success);
        assert_eq!(steps[0], Point {x: 0, y: 0});
        assert_eq!(steps[1], Point {x: 1, y: 1});
        assert_eq!(steps[2], Point {x: 2, y: 2});
    }

    #[test]
    fn test_one_wall_exit() {
        let map = Map::new(vec![Point::new(1, 1)]);

        let path = a_star_search(
            map.point2d_to_index(START_POINT),
            map.point2d_to_index(END_POINT),
            &map);
        let steps: Vec<Point> = path.steps.iter().map(|e| map.index_to_point2d(*e)).collect();
        assert!(path.success);
        assert_eq!(steps[0], Point {x: 0, y: 0});
        assert_eq!(steps[1], Point {x: 1, y: 0});
        assert_eq!(steps[2], Point {x: 2, y: 1});
        assert_eq!(steps[3], Point {x: 2, y: 2});
    }

    #[test]
    fn test_no_exit() {
        let map = Map::new(vec![Point::new(0, 1), Point::new(1, 1), Point::new(1, 0)]);

        let path = a_star_search(
            map.point2d_to_index(START_POINT),
            map.point2d_to_index(END_POINT),
            &map);

        assert!(!path.success)
    }

}