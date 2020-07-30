use super::{InitHints, BACKEND};
use crate::prelude::BTerm;
use crate::Result;
use crossterm::{
    execute,
    terminal::{size, SetSize},
};
use std::io::{stdout, Write};

pub fn init_raw<S: ToString>(
    width_pixels: u32,
    height_pixels: u32,
    _window_title: S,
    platform_hints: InitHints,
) -> Result<BTerm> {
    let old_size = size().expect("Unable to get console size");
    execute!(
        stdout(),
        SetSize(width_pixels as u16 / 8, height_pixels as u16 / 8),
    )
    .expect("Console command fail");

    execute!(stdout(), crossterm::cursor::Hide).expect("Command fail");
    execute!(stdout(), crossterm::event::EnableMouseCapture).expect("Command fail");

    let mut be = BACKEND.lock();
    be.old_width = old_size.0;
    be.old_height = old_size.1;
    be.frame_sleep_time = crate::hal::convert_fps_to_wait(platform_hints.frame_sleep_time);

    let bterm = BTerm {
        width_pixels,
        height_pixels,
        original_width_pixels: width_pixels,
        original_height_pixels: height_pixels,
        fps: 0.0,
        frame_time_ms: 0.0,
        active_console: 0,
        key: None,
        mouse_pos: (0, 0),
        left_click: false,
        shift: false,
        control: false,
        alt: false,
        web_button: None,
        quitting: false,
        post_scanlines: false,
        post_screenburn: false,
        screen_burn_color: bracket_color::prelude::RGB::from_f32(0.0, 1.0, 1.0)
    };
    Ok(bterm)
}
