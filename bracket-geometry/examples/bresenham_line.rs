use bracket_geometry::prelude::*;
use crossterm::queue;
use crossterm::style::Print;
use std::io::{stdout, Write};

fn main() {
    let mut fake_console: Vec<char> = vec!['.'; 100];

    let line : Vec<Point> =  Bresenham::new(Point::new(0, 6), Point::new(6, 0)).collect();

    for (i,point) in line.iter().enumerate() {
        let idx = ((point.y * 10) + point.x) as usize;
        let character = 48 + i as u8;
        fake_console[idx] = character as char;
    }

    for y in 0..10 {
        let mut line = String::from("");
        let idx = y * 10;
        for x in 0..10 {
            line.push(fake_console[idx + x]);
        }
        line.push('\n');
        queue!(stdout(), Print(&line)).expect("Command fail");
    }
    stdout().flush().expect("Flush Fail");

    line.iter().for_each(|p| println!("{:?}", p));
}
