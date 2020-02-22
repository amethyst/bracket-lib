use bracket_algorithm_traits::prelude::BaseMap;
#[cfg(feature = "threaded")]
use rayon::prelude::*;
use std::convert::TryInto;
use std::f32::MAX;
use std::mem;

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

    /// Internal: add a node to the open list if it doesn't exceed max_depth, and isn't on the closed list.
    /// Adds the entry to the closed list.
    fn add_if_open(
        max_depth: f32,
        idx: usize,
        open_list: &mut Vec<(usize, f32)>,
        closed_list: &mut Vec<bool>,
        new_depth: f32,
    ) {
        if new_depth > max_depth {
            return;
        }
        if closed_list[idx as usize] {
            return;
        }

        closed_list[idx as usize] = true;
        open_list.push((idx, new_depth));
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
    fn build_helper(dm: &mut DijkstraMap, starts: &[usize], map: &dyn BaseMap) {
        if starts.len() > rayon::current_num_threads() {
            DijkstraMap::build_parallel(dm, starts, map);
            return;
        }
    }

    #[cfg(not(feature = "threaded"))]
    fn build_helper(_dm: &mut DijkstraMap, _starts: &[usize], _map: &dyn BaseMap) {}

    /// Builds the Dijkstra map: iterate from each starting point, to each exit provided by BaseMap's
    /// exits implementation. Each step adds cost to the current depth, and is discarded if the new
    /// depth is further than the current depth.
    /// If you provide more starting points than you have CPUs, automatically branches to a parallel
    /// version.
    pub fn build(dm: &mut DijkstraMap, starts: &[usize], map: &dyn BaseMap) {
        DijkstraMap::build_helper(dm, starts, map);
        let mapsize: usize = (dm.size_x * dm.size_y) as usize;
        let mut open_list: Vec<(usize, f32)> = Vec::with_capacity(mapsize * 2);
        let mut closed_list: Vec<bool> = vec![false; mapsize];

        for start in starts {
            // Clearing vec in debug mode is stupidly slow, so we do it the hard way!
            unsafe {
                open_list.set_len(0);
            }
            // Zeroing the buffer is far too slow, so we're doing it the C way
            unsafe {
                std::ptr::write_bytes(
                    closed_list.as_mut_ptr(),
                    0,
                    closed_list.len() * mem::size_of::<bool>(),
                );
            }
            open_list.push((*start, 0.0));

            while !open_list.is_empty() {
                let last_idx = open_list.len() - 1;
                let current_tile = open_list[last_idx];
                let tile_idx = current_tile.0;
                let depth = current_tile.1;
                unsafe {
                    open_list.set_len(last_idx);
                }

                if dm.map[tile_idx as usize] > depth {
                    dm.map[tile_idx as usize] = depth;

                    let exits = map.get_available_exits(tile_idx);
                    for exit in exits {
                        DijkstraMap::add_if_open(
                            dm.max_depth,
                            exit.0,
                            &mut open_list,
                            &mut closed_list,
                            depth + 1.0,
                        );
                    }
                }
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

        let exits: Vec<Vec<(usize, f32)>> = (0..mapsize)
            .map(|idx| map.get_available_exits(idx))
            .collect();

        // Run each map in parallel
        layers.par_iter_mut().for_each(|l| {
            let mut open_list: Vec<(usize, f32)> = Vec::with_capacity(mapsize * 2);
            let mut closed_list: Vec<bool> = vec![false; mapsize];

            for start in l.starts.iter().copied() {
                open_list.push((start, 0.0));

                while !open_list.is_empty() {
                    let last_idx = open_list.len() - 1;
                    let current_tile = open_list[last_idx];
                    let tile_idx = current_tile.0;
                    let depth = current_tile.1;
                    unsafe {
                        open_list.set_len(last_idx);
                    }

                    if l.map[tile_idx as usize] > depth {
                        l.map[tile_idx as usize] = depth;

                        let exits = &exits[tile_idx as usize];
                        for exit in exits {
                            DijkstraMap::add_if_open(
                                l.max_depth,
                                exit.0,
                                &mut open_list,
                                &mut closed_list,
                                depth + exit.1,
                            );
                        }
                    }
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
        fn get_available_exits(&self, idx: usize) -> Vec<(usize, f32)> {
            match idx {
                0 => vec![(1, 1.)],
                2 => vec![(1, 1.)],
                _ => vec![(idx - 1, 1.), (idx + 1, 1.)],
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
