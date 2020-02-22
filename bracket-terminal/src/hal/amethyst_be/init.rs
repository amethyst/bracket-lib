use crate::hal::BTermPlatform;
use crate::prelude::InitHints;
use crate::prelude::BTerm;

pub struct PlatformGL {
    pub window_title: String,
    pub platform_hints: InitHints,
}

pub fn init_raw<S: ToString>(
    width_pixels: u32,
    height_pixels: u32,
    window_title: S,
    platform_hints: InitHints,
) -> BTerm {
    BTerm {
        backend: BTermPlatform {
            platform: PlatformGL {
                window_title: window_title.to_string(),
                platform_hints,
            },
        },
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
    }
}
