use bracket_color::prelude::*;
use std::collections::HashMap;
use parking_lot::Mutex;

pub struct CursesColor {
    rf: f32,
    gf: f32,
    bf: f32,
}

impl CursesColor {
    pub fn new(red: i16, green: i16, blue: i16) -> CursesColor {
        CursesColor {
            rf: red as f32 / 1000.0,
            gf: green as f32 / 1000.0,
            bf: blue as f32 / 1000.0,
        }
    }
}

lazy_static! {
    static ref COLOR_CACHE: Mutex<HashMap<u32, i16>> = Mutex::new(HashMap::new());
}

pub fn find_nearest_color(color: RGBA, map: &[CursesColor]) -> i16 {
    let key = (color.r * 255.0) as u32 +
        (((color.g * 255.0) as u32) << 8u32) +
        (((color.b * 255.0) as u32) << 16u32) +
        (((color.a * 255.0) as u32) << 24u32)
    ;
    {
        let cache = COLOR_CACHE.lock();
        if let Some(col) = cache.get(&key) {
            return *col;
        }
    }

    let mut result = -1;
    let mut best_diff = std::f32::MAX;

    for (i, cc) in map.iter().enumerate() {
        let diff_r = f32::abs(color.r - cc.rf);
        let diff_g = f32::abs(color.g - cc.gf);
        let diff_b = f32::abs(color.b - cc.bf);
        let total_diff = diff_r + diff_g + diff_b;

        if total_diff < best_diff {
            result = i as i16;
            best_diff = total_diff;
        }
    }

    COLOR_CACHE.lock().insert(key, result);

    result
}
