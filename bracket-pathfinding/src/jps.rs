use super::astar::NavigationPath;
use bracket_algorithm_traits::prelude::{Algorithm2D, BaseMap};
use bracket_geometry::prelude::Point;
use std::cmp::{max, min, Ordering};
use std::collections::{BinaryHeap, HashMap};
use std::convert::TryInto;

/// Direction for calculating jumps
#[derive(Copy, Clone)]
enum Direction {
    Vertical(i32),
    Horizontal(i32),
    Diagonal(i32, i32),
}

/// Bail out if the search exceeds this many steps.
const MAX_ASTAR_STEPS: usize = 65536;

/// Request JPS search. The start and end are specified as index numbers (compatible with your
/// BaseMap implementation), and it requires access to your map so as to call distance and exit determinations.
pub fn jps_search<T, M: BaseMap + Algorithm2D>(start: T, end: T, map: &M) -> NavigationPath
where
    T: TryInto<usize>,
{
    JPS::new(start.try_into().ok().unwrap(), end.try_into().ok().unwrap()).search(map)
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
    h: f32,
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

/// Private structure for calculating an A-Star navigation path.
struct JPS {
    start: usize,
    end: usize,
    open_list: BinaryHeap<Node>,
    parents: HashMap<usize, usize>,
    step_counter: usize,
}

impl JPS {
    /// Creates a new path, with specified starting and ending indices.
    fn new(start: usize, end: usize) -> JPS {
        let mut open_list: BinaryHeap<Node> = BinaryHeap::new();
        open_list.push(Node {
            idx: start,
            f: 0.0,
            g: 0.0,
            h: 0.0,
        });

        JPS {
            start,
            end,
            open_list,
            parents: HashMap::new(),
            step_counter: 0,
        }
    }

    /// Wrapper to the BaseMap's distance function.
    fn distance_to_end(&self, idx: usize, map: &dyn BaseMap) -> f32 {
        map.get_pathing_distance(idx, self.end)
    }

    /// Small helper function to build a new node
    /// from a parent and populate open and parent lists.
    fn build_jump(&mut self, node: &(usize, f32), parent: usize, map: &dyn BaseMap) {
        let distance = self.distance_to_end(node.0, map);
        let new = Node {
            idx: node.0,
            f: distance + node.1,
            g: node.1,
            h: distance,
        };
        self.open_list.push(new);
        self.parents.insert(node.0, parent);
    }

    /// Just converts a tuple into a Direction enum and passes to expand
    fn check_jump<T: BaseMap + Algorithm2D>(
        &mut self,
        node: &Node,
        map: &T,
        direction: (i32, i32),
    ) {
        // Expand depending on direction
        // Diagonal case
        let dir = if direction.0 != 0 && direction.1 != 0 {
            Direction::Diagonal(direction.0, direction.1)
        }
        // Horizontal case
        else if direction.0 != 0 {
            Direction::Horizontal(direction.0)
        }
        // Vertical
        else {
            Direction::Vertical(direction.1)
        };

        self.expand(map, node, dir);
    }

    /// Returns true if any forced neighbours found, false otherwise
    fn forced_horizontal<T: BaseMap + Algorithm2D>(
        &mut self,
        map: &T,
        idx: usize,
        direction: i32,
    ) -> bool {
        let mut forced = false;
        let pos = map.index_to_point2d(idx);

        // This is really messy, needs fixing if possible
        let top = map.point2d_to_index(Point::new(pos.x, max(pos.y - 1, 0)));
        let down = map.point2d_to_index(Point::new(pos.x, min(pos.y + 1, map.dimensions().y)));
        let next_top = map.point2d_to_index(Point::new(pos.x + direction, max(pos.y - 1, 0)));
        let next_down = map.point2d_to_index(Point::new(
            pos.x + direction,
            min(pos.y + 1, map.dimensions().y),
        ));

        let exits = map.get_available_exits(idx);

        // Check if blocked up
        if let Some(jump_point) = exits.iter().find(|x| x.0 == next_top) {
            if exits.iter().find(|x| x.0 == top).is_none() {
                self.build_jump(jump_point, idx, map);
                forced = true;
            }
        }

        // Check if blocked down
        if let Some(jump_point) = exits.iter().find(|x| x.0 == next_down) {
            if exits.iter().find(|x| x.0 == down).is_none() {
                self.build_jump(jump_point, idx, map);
                forced = true;
            }
        }

        forced
    }

    /// Returns true if any forced neighbours found, false otherwise
    fn forced_vertical<T: BaseMap + Algorithm2D>(
        &mut self,
        map: &T,
        idx: usize,
        direction: i32,
    ) -> bool {
        let mut forced = false;
        let pos = map.index_to_point2d(idx);

        // This is really messy, needs fixing if possible
        let left = map.point2d_to_index(Point::new(max(pos.x - 1, 0), pos.y));
        let right = map.point2d_to_index(Point::new(min(pos.x + 1, map.dimensions().x), pos.y));
        let next_left = map.point2d_to_index(Point::new(max(pos.x - 1, 0), pos.y + direction));
        let next_right = map.point2d_to_index(Point::new(
            min(pos.x + 1, map.dimensions().x),
            pos.y + direction,
        ));

        let exits = map.get_available_exits(idx);

        // Check if blocked left
        if let Some(jump_point) = exits.iter().find(|x| x.0 == next_left) {
            if exits.iter().find(|x| x.0 == left).is_none() {
                self.build_jump(jump_point, idx, map);
                forced = true;
            }
        }

        // Check if blocked right
        if let Some(jump_point) = exits.iter().find(|x| x.0 == next_right) {
            if exits.iter().find(|x| x.0 == right).is_none() {
                self.build_jump(jump_point, idx, map);
                forced = true;
            }
        }

        forced
    }

    /// Continues expanding in given direction until
    /// blocked or reaches point of interest
    /// returns true if poi, false if blocked
    fn expand<T: BaseMap + Algorithm2D>(
        &mut self,
        map: &T,
        start_node: &Node,
        direction: Direction,
    ) -> bool {
        let mut current = *start_node;
        let mut can_continue = true;

        while can_continue {
            // Check if goal
            if current.idx == self.end {
                self.open_list.push(current);
                self.parents.insert(current.idx, start_node.idx);
                return true;
            }

            // Otherwise check for forced neighbours
            // and break out of loop if found
            let dir;
            match direction {
                Direction::Vertical(vert) => {
                    dir = (0, vert);
                    // Check for forced neighbours
                    if self.forced_vertical(map, current.idx, vert) {
                        can_continue = false;
                    }
                }
                Direction::Horizontal(hor) => {
                    dir = (hor, 0);
                    // Check for forced neighbours
                    if self.forced_horizontal(map, current.idx, hor) {
                        can_continue = false;
                    }
                }
                Direction::Diagonal(hor, vert) => {
                    dir = (hor, vert);
                    // Expand horizontally - needs to expand vertically and horizontally
                    // for each iteration
                    if self.expand(map, &current, Direction::Horizontal(hor)) {
                        can_continue = false;
                    }
                    if self.expand(map, &current, Direction::Vertical(vert)) {
                        can_continue = false;
                    }
                }
            }

            // If we need to break add current position to parents
            // so we can find our way home later.
            if !can_continue {
                self.parents.insert(current.idx, start_node.idx);
            }

            // If next position is accesible advance current
            // otherwise return false
            let pos = map.index_to_point2d(current.idx);
            let next_position = map.point2d_to_index(Point::new(pos.x + dir.0, pos.y + dir.1));

            let exits = map.get_available_exits(current.idx);
            if let Some(exit) = exits.iter().find(|x| x.0 == next_position) {
                let distance = self.distance_to_end(exit.0, map);
                let g = current.g + exit.1;
                current = Node {
                    idx: next_position,
                    f: distance + g,
                    g,
                    h: distance,
                };
            } else {
                return false;
            }
        }

        self.open_list.push(current);
        self.parents.insert(current.idx, start_node.idx);
        true
    }

    /// Helper function to unwrap a path once we've found the end-point.
    /// This is slightly more complex than its A* equivalent as JPS skips nodes,
    /// so we need to interpolate between steps were necessary to get an actual path.
    fn found_it<T: BaseMap + Algorithm2D>(&self, map: &T) -> NavigationPath {
        let mut result = NavigationPath::new();
        result.success = true;
        result.destination = self.end;

        result.steps.push(self.end);
        let mut current = self.end;
        while current != self.start {
            let pos = map.index_to_point2d(current);
            let parent = self.parents[&current];
            let direction = direction(parent, current, map);
            let mut next =
                map.point2d_to_index(Point::new(pos.x + direction.0, pos.y + direction.1));

            // Push intermediate nodes if any
            while next != parent {
                result.steps.push(next);
                let pos = map.index_to_point2d(next);
                next = map.point2d_to_index(Point::new(pos.x + direction.0, pos.y + direction.1));
            }

            // Push actual steps
            result.steps.push(parent);
            current = parent;
        }
        result.steps.reverse();

        result
    }

    /// Performs an JPS search
    fn search<T: BaseMap + Algorithm2D>(&mut self, map: &T) -> NavigationPath {
        // Initialize empty path and closed list
        let result = NavigationPath::new();
        let mut closed_list = Vec::new();

        // Add start node neighbours to open list
        let start = self.open_list.pop().unwrap();
        for node in map.get_available_exits(start.idx) {
            let distance_to_goal = self.distance_to_end(node.0, map);
            self.open_list.push(Node {
                idx: node.0,
                f: distance_to_goal + node.1,
                g: node.1,
                h: distance_to_goal,
            });
            self.parents.insert(node.0, start.idx);
        }

        // Check each open list item
        while !self.open_list.is_empty() && self.step_counter < MAX_ASTAR_STEPS {
            self.step_counter += 1;

            // Pop Q off of the list
            let q = self.open_list.pop().unwrap();

            // If this is the goal unwind
            if q.idx == self.end {
                let success = self.found_it(map);
                return success;
            }

            // Check if node is on closed list and continue if it is
            if closed_list.contains(&q.idx) {
                continue;
            }

            // Calculate direction and check jump
            let parent = self.parents[&q.idx];
            let direction = direction(q.idx, parent, map);
            self.check_jump(&q, map, direction);

            // Push onto closed list
            closed_list.push(q.idx);
        }
        result
    }
}

fn direction<T: BaseMap + Algorithm2D>(current: usize, parent: usize, map: &T) -> (i32, i32) {
    let current = map.index_to_point2d(current);
    let parent = map.index_to_point2d(parent);

    // Calculate direction - needs cleaning if possible
    let mut direction_x = current.x as i32 - parent.x as i32;
    let mut direction_y = current.y as i32 - parent.y as i32;

    if direction_x < 0 {
        direction_x = -1;
    } else if direction_x > 0 {
        direction_x = 1;
    }
    if direction_y < 0 {
        direction_y = -1;
    } else if direction_y > 0 {
        direction_y = 1;
    }

    (direction_x, direction_y)
}
