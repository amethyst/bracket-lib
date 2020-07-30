use bracket_color::prelude::*;
use std::io::{stdout, Write};
use util::print_color;

fn main() {
    let red = RGB::named(RED);
    let blue = RGB::named(YELLOW);
    for i in 1..80 {
        let percent = i as f32 / 80.0;
        let color = red.lerp(blue, percent);
        print_color(color, "*");
    }
    print_color(RGB::named(WHITE), "\nAnd back to white.\n");

    stdout().flush().expect("Flush Fail");
}

mod util;
