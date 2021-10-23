use lazy_static::*;
use parking_lot::Mutex;
use crate::hal::PlatformGL;

lazy_static! {
    pub static ref BACKEND: Mutex<PlatformGL> = Mutex::new(PlatformGL {
    });
}