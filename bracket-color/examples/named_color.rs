use bracket_color::prelude::*;
use std::io::{stdout, Write};
use util::print_color;

fn main() {
    print_color(RGB::named(RED), "Hello RED\n");
    print_color(GREEN.into(), "Hello GREEN\n"); // type inference works too!
    print_color(RGB::named(BLUE), "Hello BLUE\n");
    print_color(RGB::named(WHITE), "And back to white.\n");

    stdout().flush().expect("Flush Fail");
}

mod util;
