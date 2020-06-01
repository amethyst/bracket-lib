use bracket_geometry::prelude::*;
use crossterm::{cursor, queue};
use crossterm::style::Print;
use std::io::{stdout, Write};

fn main() {
    let mut angle = Degrees::new(0.0);
    let start_point = Point::new(10, 10);
    while angle.0 < 360.0 {
        queue!(stdout(), cursor::MoveTo(0, 0));
        let mut fake_console: Vec<char> = vec!['.'; 400];

        let end_point = project_angle(Point::new(0,0), 8.0, angle) + start_point;
        let line : Vec<Point> =  Bresenham::new(start_point, end_point).collect();

        for (i,point) in line.iter().enumerate() {
            let idx = ((point.y * 20) + point.x) as usize;
            let character = 48 + i as u8;
            fake_console[idx] = character as char;
        }

        for y in 0..20 {
            let mut line = String::from("");
            let idx = y * 20;
            for x in 0..20 {
                line.push(fake_console[idx + x]);
            }
            line.push('\n');
            queue!(stdout(), Print(&line)).expect("Command fail");
        }
        stdout().flush().expect("Flush Fail");
        angle.0 += 1.0;
        std::thread::sleep_ms(20);
    }
}
