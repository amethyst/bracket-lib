use pancurses::{initscr, noecho, resize_term};
pub use super::*;

pub fn init_raw<S: ToString>(
    width_pixels: u32,
    height_pixels: u32,
    _window_title: S,
    platform_hints: InitHints,
) -> Result<BTerm> {
    let window = initscr();
    resize_term(height_pixels as i32 / 8, width_pixels as i32 / 8);
    noecho();
    window.nodelay(true);
    window.keypad(true);
    pancurses::start_color();
    pancurses::mousemask(
        pancurses::ALL_MOUSE_EVENTS | pancurses::REPORT_MOUSE_POSITION,
        std::ptr::null_mut(),
    );

    // Setup basic color mapping
    let mut color_map = Vec::new();
    for i in 0..16 {
        let color = pancurses::color_content(i);
        color_map.push(CursesColor::new(color.0, color.1, color.2));
    }

    let mut counter = 0;
    for bg in 0..16i16 {
        for fg in 0..16i16 {
            pancurses::init_pair(counter as i16, fg, bg);
            counter += 1;
        }
    }

    let mut be = BACKEND.lock().unwrap();
    be.window = Some(window);
    be.color_map = color_map;
    be.frame_sleep_time = crate::hal::convert_fps_to_wait(platform_hints.frame_sleep_time);

    let bterm = BTerm {
        width_pixels,
        height_pixels,
        fonts: Vec::new(),
        consoles: Vec::new(),
        shaders: Vec::new(),
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
    };
    Ok(bterm)
}