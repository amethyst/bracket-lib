use lazy_static::*;
use parking_lot::Mutex;
use crate::hal::{ConsoleBacking, PlatformGL};

lazy_static! {
    pub static ref BACKEND: Mutex<PlatformGL> = Mutex::new(PlatformGL {
        context_wrapper: None,
        wgpu: None,
        resize_scaling: false,
        resize_request: None,
        request_screenshot: None,
        frame_sleep_time: None,
    });
}

lazy_static! {
    pub(crate) static ref CONSOLE_BACKING: Mutex<Vec<ConsoleBacking>> = Mutex::new(Vec::new());
}