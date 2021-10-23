// Enable modules based on target architecture
#[cfg(feature = "opengl")]
mod gl_common;

#[cfg(feature = "opengl")]
pub use gl_common::*;

#[cfg(all(feature = "opengl", not(target_arch = "wasm32")))]
mod native;

#[cfg(all(feature = "opengl", not(target_arch = "wasm32")))]
pub use native::*;

#[cfg(all(feature = "opengl", target_arch = "wasm32"))]
mod wasm;

#[cfg(all(feature = "opengl", target_arch = "wasm32"))]
pub use wasm::*;

#[cfg(all(not(feature = "opengl"), feature = "webgpu"))]
mod webgpu;

#[cfg(all(not(feature = "opengl"), feature = "webgpu"))]
pub use webgpu::*;

#[cfg(not(feature = "opengl"))]
#[cfg(all(not(feature = "opengl"), feature = "curses"))]
mod curses;

#[cfg(all(not(feature = "opengl"), feature = "curses"))]
pub use curses::*;

#[cfg(not(feature = "opengl"))]
#[cfg(all(not(feature = "opengl"), feature = "cross_term"))]
mod crossterm_be;

#[cfg(all(not(feature = "opengl"), feature = "cross_term"))]
pub use crossterm_be::*;

#[cfg(all(
    not(feature = "opengl"),
    not(feature = "curses"),
    not(feature = "webgpu"),
    not(feature = "crossterm")
))]
mod dummy;

#[cfg(all(
    not(feature = "opengl"),
    not(feature = "curses"),
    not(feature = "webgpu"),
    not(feature = "crossterm")
))]
pub use dummy::*;

/// Provides a base abstract platform for BTerm to run on, with specialized content.
pub struct BTermPlatform {
    pub platform: PlatformGL,
}

#[allow(dead_code)]
fn convert_fps_to_wait(frame_sleep_time: Option<f32>) -> Option<u64> {
    frame_sleep_time.map(|f| (f * 1000.0) as u64)
}

#[allow(dead_code)]
fn fps_sleep(frame_sleep_time: Option<u64>, now: &std::time::Instant, prev_ms: u128) {
    if let Some(wait_time) = frame_sleep_time {
        let execute_ms = now.elapsed().as_millis() as u64 - prev_ms as u64;
        if execute_ms < wait_time {
            std::thread::sleep(std::time::Duration::from_millis(wait_time - execute_ms));
        }
    }
}
