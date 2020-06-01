use crate::prelude::BTerm;
use crate::prelude::InitHints;
use crate::Result;
use parking_lot::Mutex;

pub struct PlatformGL {
    pub window_title: String,
    pub platform_hints: InitHints,
}

lazy_static! {
    pub static ref BACKEND: Mutex<PlatformGL> = Mutex::new(PlatformGL {
        window_title: "".to_string(),
        platform_hints: InitHints::new()
    });
}

unsafe impl Send for PlatformGL {}
unsafe impl Sync for PlatformGL {}

pub fn init_raw<S: ToString>(
    width_pixels: u32,
    height_pixels: u32,
    window_title: S,
    platform_hints: InitHints,
) -> Result<BTerm> {
    let mut be = BACKEND.lock();
    be.window_title = window_title.to_string();
    be.platform_hints = platform_hints;

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
