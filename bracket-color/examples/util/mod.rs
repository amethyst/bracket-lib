use bracket_color::prelude::*;
use crossterm::queue;
use crossterm::style::{Print, SetForegroundColor};
use std::convert::TryInto;
use std::io::{stdout, Write};

pub fn print_color(color: RGB, text: &str) {
    queue!(stdout(), SetForegroundColor(color.try_into().unwrap())).expect("Command Fail");
    queue!(stdout(), Print(text)).expect("Command fail");
}
