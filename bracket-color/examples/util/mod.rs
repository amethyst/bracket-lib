use bracket_color::prelude::*;
use crossterm::queue;
use crossterm::style::{Print, SetForegroundColor};
use std::io::{stdout, Write};
use std::convert::TryInto;

pub fn print_color(color: RGB, text: &str) {
    queue!(
        stdout(),
        SetForegroundColor(color.try_into().unwrap())
    )
    .expect("Command Fail");
    queue!(stdout(), Print(text)).expect("Command fail");
}
