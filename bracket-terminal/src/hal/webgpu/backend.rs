//! Defines the BACKEND static used by wgpu.

use crate::hal::{ConsoleBacking, PlatformGL, scaler::ScreenScaler};
use lazy_static::*;
use parking_lot::Mutex;

lazy_static! {
    pub static ref BACKEND: Mutex<PlatformGL> = Mutex::new(PlatformGL {
        context_wrapper: None,
        wgpu: None,
        resize_scaling: false,
        resize_request: None,
        request_screenshot: None,
        frame_sleep_time: None,
        screen_scaler: ScreenScaler::default()
    });
}

lazy_static! {
    pub(crate) static ref CONSOLE_BACKING: Mutex<Vec<ConsoleBacking>> = Mutex::new(Vec::new());
}
