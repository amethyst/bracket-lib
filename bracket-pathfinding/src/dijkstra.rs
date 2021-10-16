use bracket_algorithm_traits::prelude::BaseMap;
#[cfg(feature = "threaded")]
use rayon::prelude::*;
#[allow(unused_imports)]
use smallvec::SmallVec;
use std::collections::VecDeque;
use std::convert::TryInto;
use std::f32::MAX;

/// Representation of a Dijkstra flow map.
/// map is a vector of floats, having a size equal to size_x * size_y (one per tile).
/// size_x and size_y are stored for overflow avoidance.
/// max_depth is the maximum number of iterations this search shall support.
pub struct DijkstraMap {
    pub map: Vec<f32>,
    size_x: usize,
    size_y: usize,
    max_depth: f32,
}

/// Used internally when constructing maps in parallel
#[cfg(feature = "threaded")]
struct ParallelDm {
    map: Vec<f32>,
    max_depth: f32,
    starts: Vec<usize>,
}

// This is chosen arbitrarily. Whether it's better to
// run threaded or not would depend on map structure,
// map size, number of starts, and probably several
// other parameters. Might want to make this choice
// an explicit part of the API?
#[allow(dead_code)]
const THREADED_REQUIRED_STARTS: usize = 4;

#[derive(PartialEq)]
enum RunThreaded {
    True,
    False,
}

#[allow(dead_code)]
impl DijkstraMap {
    /// Construct a new Dijkstra map, ready to run. You must specify the map size, and link to an implementation
    /// of a BaseMap trait that can generate exits lists. It then builds the map, giving you a result.
    pub fn new<T>(
        size_x: T,
        size_y: T,
        starts: &[usize],
        map: &dyn BaseMap,
        max_depth: f32,
    ) -> DijkstraMap
    where
        T: TryInto<usize>,
    {
        let sz_x: usize = size_x.try_into().ok().unwrap();
        let sz_y: usize = size_y.try_into().ok().unwrap();
        let result: Vec<f32> = vec![MAX; sz_x * sz_y];
        let mut d = DijkstraMap {
            map: result,
            size_x: sz_x,
            size_y: sz_y,
            max_depth,
        };
        DijkstraMap::build(&mut d, starts, map);
        d
    }

    /// Construct a new Dijkstra map, ready to run. You must specify the map size, and link to an implementation
    /// of a BaseMap trait that can generate exits lists. It then builds the map, giving you a result.
    /// Starts is provided as a set of tuples, two per tile. The first is the tile index, the second the starting
    /// weight (defaults to 0.0 on new)
    pub fn new_weighted<T>(
        size_x: T,
        size_y: T,
        starts: &[(usize, f32)],
        map: &dyn BaseMap,
        max_depth: f32,
    ) -> DijkstraMap
    where
        T: TryInto<usize>,
    {
        let sz_x: usize = size_x.try_into().ok().unwrap();
        let sz_y: usize = size_y.try_into().ok().unwrap();
        let result: Vec<f32> = vec![MAX; sz_x * sz_y];
        let mut d = DijkstraMap {
            map: result,
            size_x: sz_x,
            size_y: sz_y,
            max_depth,
        };
        DijkstraMap::build_weighted(&mut d, starts, map);
        d
    }

    /// Creates an empty Dijkstra map node.
    pub fn new_empty<T>(size_x: T, size_y: T, max_depth: f32) -> DijkstraMap
    where
        T: TryInto<usize>,
    {
        let sz_x: usize = size_x.try_into().ok().unwrap();
        let sz_y: usize = size_y.try_into().ok().unwrap();
        let result: Vec<f32> = vec![MAX; sz_x * sz_y];
        DijkstraMap {
            map: result,
            size_x: sz_x,
            size_y: sz_y,
            max_depth,
        }
    }

    /// Clears the Dijkstra map. Uses a parallel for each for performance.
    #[cfg(feature = "threaded")]
    pub fn clear(dm: &mut DijkstraMap) {
        dm.map.par_iter_mut().for_each(|x| *x = MAX);
    }

    #[cfg(not(feature = "threaded"))]
    pub fn clear(dm: &mut DijkstraMap) {
        dm.map.iter_mut().for_each(|x| *x = MAX);
    }

    #[cfg(feature = "threaded")]
    fn build_helper(dm: &mut DijkstraMap, starts: &[usize], map: &dyn BaseMap) -> RunThreaded {
        if starts.len() >= THREADED_REQUIRED_STARTS {
            DijkstraMap::build_parallel(dm, starts, map);
            return RunThreaded::True;
        }
        RunThreaded::False
    }

    #[cfg(not(feature = "threaded"))]
    fn build_helper(_dm: &mut DijkstraMap, _starts: &[usize], _map: &dyn BaseMap) -> RunThreaded {
        RunThreaded::False
    }

    #[cfg(feature = "threaded")]
    fn build_helper_weighted(dm: &mut DijkstraMap, starts: &[(usize, f32)], map: &dyn BaseMap) -> RunThreaded {
        if starts.len() >= THREADED_REQUIRED_STARTS {
            DijkstraMap::build_parallel_weighted(dm, starts, map);
            return RunThreaded::True;
        }
        RunThreaded::False
    }

    /// Builds the Dijkstra map: iterate from each starting point, to each exit provided by BaseMap's
    /// exits implementation. Each step adds cost to the current depth, and is discarded if the new
    /// depth is further than the current depth.
    /// WARNING: Will give incorrect results when used with non-uniform exit costs. Much slower
    /// algorithm required to support that.
    /// Automatically branches to a parallel version if you provide more than 4 starting points
    pub fn build(dm: &mut DijkstraMap, starts: &[usize], map: &dyn BaseMap) {
        let threaded = DijkstraMap::build_helper(dm, starts, map);
        if threaded == RunThreaded::True {
            return;
        }
        let mapsize: usize = (dm.size_x * dm.size_y) as usize;
        let mut open_list: VecDeque<(usize, f32)> = VecDeque::with_capacity(mapsize);

        for start in starts {
            open_list.push_back((*start, 0.0));
        }

        while let Some((tile_idx, depth)) = open_list.pop_front() {
            let exits = map.get_available_exits(tile_idx);
            for (new_idx, add_depth) in exits {
                let new_depth = depth + add_depth;
                let prev_depth = dm.map[new_idx];
                if new_depth >= prev_depth {
                    continue;
                }
                if new_depth >= dm.max_depth {
                    continue;
                }
                dm.map[new_idx] = new_depth;
                open_list.push_back((new_idx, new_depth));
            }
        }
    }

    /// Builds the Dijkstra map: iterate from each starting point, to each exit provided by BaseMap's
    /// exits implementation. Each step adds cost to the current depth, and is discarded if the new
    /// depth is further than the current depth.
    /// WARNING: Will give incorrect results when used with non-uniform exit costs. Much slower
    /// algorithm required to support that.
    /// Automatically branches to a parallel version if you provide more than 4 starting points
    pub fn build_weighted(dm: &mut DijkstraMap, starts: &[(usize, f32)], map: &dyn BaseMap) {
        let mapsize: usize = (dm.size_x * dm.size_y) as usize;
        let mut open_list: VecDeque<(usize, f32)> = VecDeque::with_capacity(mapsize);

        for start in starts {
            open_list.push_back(*start);
        }

        while let Some((tile_idx, depth)) = open_list.pop_front() {
            let exits = map.get_available_exits(tile_idx);
            for (new_idx, add_depth) in exits {
                let new_depth = depth + add_depth;
                let prev_depth = dm.map[new_idx];
                if new_depth >= prev_depth {
                    continue;
                }
                if new_depth >= dm.max_depth {
                    continue;
                }
                dm.map[new_idx] = new_depth;
                open_list.push_back((new_idx, new_depth));
            }
        }
    }

    /// Implementation of Parallel Dijkstra.
    #[cfg(feature = "threaded")]
    fn build_parallel(dm: &mut DijkstraMap, starts: &[usize], map: &dyn BaseMap) {
        let mapsize: usize = (dm.size_x * dm.size_y) as usize;
        let mut layers: Vec<ParallelDm> = Vec::with_capacity(starts.len());
        for start_chunk in starts.chunks(rayon::current_num_threads()) {
            let mut layer = ParallelDm {
                map: vec![MAX; mapsize],
                max_depth: dm.max_depth,
                starts: Vec::new(),
            };
            layer
                .starts
                .extend(start_chunk.iter().copied().map(|x| x as usize));
            layers.push(layer);
        }

        let exits: Vec<SmallVec<[(usize, f32); 10]>> = (0..mapsize)
            .map(|idx| map.get_available_exits(idx))
            .collect();

        // Run each map in parallel
        layers.par_iter_mut().for_each(|l| {
            let mut open_list: VecDeque<(usize, f32)> = VecDeque::with_capacity(mapsize);

            for start in l.starts.iter().copied() {
                open_list.push_back((start, 0.0));
            }

            while let Some((tile_idx, depth)) = open_list.pop_front() {
                let exits = &exits[tile_idx];
                for (new_idx, add_depth) in exits {
                    let new_idx = *new_idx;
                    let new_depth = depth + add_depth;
                    let prev_depth = l.map[new_idx];
                    if new_depth >= prev_depth {
                        continue;
                    }
                    if new_depth >= l.max_depth {
                        continue;
                    }
                    l.map[new_idx] = new_depth;
                    open_list.push_back((new_idx, new_depth));
                }
            }
        });

        // Recombine down to a single result
        for l in layers {
            for i in 0..mapsize {
                dm.map[i] = f32::min(dm.map[i], l.map[i]);
            }
        }
    }

        #[cfg(feature = "threaded")]
    fn build_parallel(dm: &mut DijkstraMap, starts: &[usize], map: &dyn BaseMap) {
        let mapsize: usize = (dm.size_x * dm.size_y) as usize;
        let mut layers: Vec<ParallelDm> = Vec::with_capacity(starts.len());
        for start_chunk in starts.chunks(rayon::current_num_threads()) {
            let mut layer = ParallelDm {
                map: vec![MAX; mapsize],
                max_depth: dm.max_depth,
                starts: Vec::new(),
            };
            layer
                .starts
                .extend(start_chunk.iter().copied().map(|x| x as usize));
            layers.push(layer);
        }

        let exits: Vec<SmallVec<[(usize, f32); 10]>> = (0..mapsize)
            .map(|idx| map.get_available_exits(idx))
            .collect();

        // Run each map in parallel
        layers.par_iter_mut().for_each(|l| {
            let mut open_list: VecDeque<(usize, f32)> = VecDeque::with_capacity(mapsize);

            for start in l.starts.iter().copied() {
                open_list.push_back((start, 0.0));
            }

            while let Some((tile_idx, depth)) = open_list.pop_front() {
                let exits = &exits[tile_idx];
                for (new_idx, add_depth) in exits {
                    let new_idx = *new_idx;
                    let new_depth = depth + add_depth;
                    let prev_depth = l.map[new_idx];
                    if new_depth >= prev_depth {
                        continue;
                    }
                    if new_depth >= l.max_depth {
                        continue;
                    }
                    l.map[new_idx] = new_depth;
                    open_list.push_back((new_idx, new_depth));
                }
            }
        });

        // Recombine down to a single result
        for l in layers {
            for i in 0..mapsize {
                dm.map[i] = f32::min(dm.map[i], l.map[i]);
            }
        }
    }

    /// Helper for traversing maps as path-finding. Provides the index of the lowest available
    /// exit from the specified position index, or None if there isn't one.
    /// You would use this for pathing TOWARDS a starting node.
    #[cfg(feature = "threaded")]
    pub fn find_lowest_exit(dm: &DijkstraMap, position: usize, map: &dyn BaseMap) -> Option<usize> {
        let mut exits = map.get_available_exits(position);

        if exits.is_empty() {
            return None;
        }

        exits.par_sort_by(|a, b| {
            dm.map[a.0 as usize]
                .partial_cmp(&dm.map[b.0 as usize])
                .unwrap()
        });

        Some(exits[0].0)
    }

    #[cfg(not(feature = "threaded"))]
    pub fn find_lowest_exit(dm: &DijkstraMap, position: usize, map: &dyn BaseMap) -> Option<usize> {
        let mut exits = map.get_available_exits(position);

        if exits.is_empty() {
            return None;
        }

        exits.sort_by(|a, b| {
            dm.map[a.0 as usize]
                .partial_cmp(&dm.map[b.0 as usize])
                .unwrap()
        });

        Some(exits[0].0)
    }

    /// Helper for traversing maps as path-finding. Provides the index of the highest available
    /// exit from the specified position index, or None if there isn't one.
    /// You would use this for pathing AWAY from a starting node, for example if you are running
    /// away.
    #[cfg(feature = "threaded")]
    pub fn find_highest_exit(
        dm: &DijkstraMap,
        position: usize,
        map: &dyn BaseMap,
    ) -> Option<usize> {
        let mut exits = map.get_available_exits(position);

        if exits.is_empty() {
            return None;
        }

        exits.par_sort_by(|a, b| {
            dm.map[b.0 as usize]
                .partial_cmp(&dm.map[a.0 as usize])
                .unwrap()
        });

        Some(exits[0].0)
    }

    #[cfg(not(feature = "threaded"))]
    pub fn find_highest_exit(
        dm: &DijkstraMap,
        position: usize,
        map: &dyn BaseMap,
    ) -> Option<usize> {
        let mut exits = map.get_available_exits(position);

        if exits.is_empty() {
            return None;
        }

        exits.sort_by(|a, b| {
            dm.map[b.0 as usize]
                .partial_cmp(&dm.map[a.0 as usize])
                .unwrap()
        });

        Some(exits[0].0)
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;
    use bracket_algorithm_traits::prelude::*;
    // 1 by 3 stripe of tiles
    struct MiniMap;
    impl BaseMap for MiniMap {
        fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
            match idx {
                0 => smallvec![(1, 1.)],
                2 => smallvec![(1, 1.)],
                _ => smallvec![(idx - 1, 1.), (idx + 1, 2.)],
            }
        }
    }
    #[test]
    fn test_highest_exit() {
        let map = MiniMap {};
        let exits_map = DijkstraMap::new(3, 1, &[0], &map, 10.);
        let target = DijkstraMap::find_highest_exit(&exits_map, 0, &map);
        assert_eq!(target, Some(1));
        let target = DijkstraMap::find_highest_exit(&exits_map, 1, &map);
        assert_eq!(target, Some(2));
    }
}
