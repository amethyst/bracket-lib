use bracket_color::prelude::*;

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

pub fn find_nearest_color(color: RGB, map: &[CursesColor]) -> i16 {
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

    result
}
