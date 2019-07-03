use std::f32::MAX;
use std::collections::HashSet;
use super::BaseMap;

#[allow(dead_code)]
pub struct DijkstraMap {
    pub map : Vec<f32>,
    size_x : i32,
    size_y : i32,
    max_depth : f32
}

#[allow(dead_code)]
impl DijkstraMap {
    pub fn new(size_x : i32, size_y: i32, starts: &Vec<i32>, map: &BaseMap, max_depth : f32) -> DijkstraMap {
        let mut result : Vec<f32> = Vec::new();
        for _i in 0 .. (size_x * size_y) { result.push(MAX) }
        let mut d = DijkstraMap{ map : result, size_x : size_x, size_y : size_y, max_depth : max_depth};
        d.build(starts, map);
        return d;
    }

    pub fn new_empty(size_x : i32, size_y: i32, max_depth : f32) -> DijkstraMap {
        let mut result : Vec<f32> = Vec::new();
        for _i in 0 .. (size_x * size_y) { result.push(MAX) }
        let d = DijkstraMap{ map : result, size_x : size_x, size_y : size_y, max_depth : max_depth};
        return d;
    }

    #[inline(always)]
    fn add_if_open(&self, idx : i32, open_list : &mut Vec<(i32, f32)>, closed_list : &mut HashSet<i32>, new_depth : f32) {
        if new_depth > self.max_depth { return; }
        if closed_list.contains(&idx) { return; }

        closed_list.insert(idx);
        open_list.push((idx, new_depth));
    }

    pub fn build(&mut self, starts: &Vec<i32>, map: &BaseMap) {
        let mapsize : usize = (self.size_x * self.size_y) as usize;
        let mut open_list : Vec<(i32, f32)> = Vec::with_capacity(mapsize);
        let mut closed_list : HashSet<i32> = HashSet::with_capacity(mapsize);

        for start in starts.iter() {
            unsafe { open_list.set_len(0); }
            closed_list.clear();
            open_list.push((*start, 0.0));

            while !open_list.is_empty() {
                let last_idx = open_list.len()-1;
                let current_tile = open_list[last_idx];
                let tile_idx = current_tile.0;
                let depth = current_tile.1;
                unsafe { open_list.set_len(last_idx); }

                if self.map[tile_idx as usize] > depth {
                    self.map[tile_idx as usize] = depth;

                    let exits = map.get_available_exits(tile_idx);
                    for exit in exits.iter() {
                        self.add_if_open(exit.0, &mut open_list, &mut closed_list, depth + exit.1);
                    }
                }
            }
        }
    }

    pub fn find_lowest_exit(&self, position : i32, map: &BaseMap) -> Option<i32> {
        let mut exits = map.get_available_exits(position);

        for exit in exits.iter_mut() {
            exit.1 = self.map[exit.0 as usize] as f32;
        }

        if exits.is_empty() { return None; }
        exits.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap() );

        return Some(exits[0].0);
    }

    pub fn find_highest_exit(&self, position : i32, map: &BaseMap) -> Option<i32> {
        let mut exits = map.get_available_exits(position);

        for exit in exits.iter_mut() {
            exit.1 = self.map[exit.0 as usize] as f32;
        }

        if exits.is_empty() { return None; }
        exits.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap() );

        return Some(exits[0].0);
    }
}

