// Enable modules based on target architecture
#[cfg(all(feature = "opengl", not(target_arch = "wasm32")))]
mod native;

#[cfg(all(feature = "opengl", not(target_arch = "wasm32")))]
pub use native::*;

#[cfg(all(feature = "opengl", target_arch = "wasm32"))]
mod wasm;

#[cfg(all(feature = "opengl", target_arch = "wasm32"))]
pub use wasm::*;

#[cfg(not(feature = "opengl"))]
#[cfg(all(not(feature = "opengl"), feature = "curses"))]
mod curses;

#[cfg(all(not(feature = "opengl"), feature = "curses"))]
pub use curses::*;

#[cfg(not(feature = "opengl"))]
#[cfg(all(not(feature = "opengl"), feature = "crossterm"))]
mod crossterm_be;

#[cfg(all(not(feature = "opengl"), feature = "crossterm"))]
pub use crossterm_be::*;

#[cfg(all(
    not(feature = "opengl"),
    any(feature = "amethyst_engine_vulkan", feature = "amethyst_engine_metal")
))]
mod amethyst_be;

#[cfg(all(
    not(feature = "opengl"),
    any(feature = "amethyst_engine_vulkan", feature = "amethyst_engine_metal")
))]
pub use amethyst_be::*;

#[cfg(all(
    not(feature = "opengl"),
    not(feature = "curses"),
    not(feature = "amethyst_engine_vulkan"),
    not(feature = "amethyst_engine_metal"),
    not(feature = "crossterm")
))]
mod dummy;

#[cfg(all(
    not(feature = "opengl"),
    not(feature = "curses"),
    not(feature = "amethyst_engine_vulkan"),
    not(feature = "amethyst_engine_metal"),
    not(feature = "crossterm")
))]
pub use dummy::*;

pub use shader::Shader;

/// Provides a base abstract platform for BTerm to run on, with specialized content.
pub struct BTermPlatform {
    pub platform: PlatformGL,
}

#[allow(dead_code)]
fn convert_fps_to_wait(frame_sleep_time: Option<f32>) -> Option<u64> {
    match frame_sleep_time {
        None => None,
        Some(f) => Some((f * 1000.0) as u64),
    }
}

#[allow(dead_code)]
#[inline(always)]
fn fps_sleep(frame_sleep_time: Option<u64>, now: &std::time::Instant, prev_ms: u128) {
    if let Some(wait_time) = frame_sleep_time {
        let execute_ms = now.elapsed().as_millis() as u64 - prev_ms as u64;
        if execute_ms < wait_time {
            std::thread::sleep(std::time::Duration::from_millis(wait_time - execute_ms));
        }
    }
}
