//! WGPU Platform definition

use wgpu::{Adapter, Device, Instance, Queue, Surface, SurfaceConfiguration};
use winit::{event_loop::EventLoop, window::Window};
use super::Framebuffer;

/// Defines the WGPU platform
pub struct PlatformGL {
    /// Wrapper for the winit context
    pub context_wrapper: Option<WrappedContext>,
    /// Contains the WGPU back-end (device, etc.)
    pub wgpu: Option<WgpuLink>,

    /// Target delay per frame
    pub frame_sleep_time: Option<u64>,
    /// Should the back-end resize windows by character (true) or just scale them (false)?
    pub resize_scaling: bool,
    /// Is there a request to resize the console?
    pub resize_request: Option<(u32, u32)>,
    /// Are we requesting a screenshot?
    pub request_screenshot: Option<String>,
}

pub struct WgpuLink {
    pub instance: Instance,
    pub surface: Surface,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
    pub backing_buffer: Framebuffer,
}

unsafe impl Send for PlatformGL {}
unsafe impl Sync for PlatformGL {}

pub struct WrappedContext {
    pub el: EventLoop<()>,
    pub window: Window,
}

pub struct InitHints {
    pub vsync: bool,
    pub fullscreen: bool,
    pub frame_sleep_time: Option<f32>,
    pub resize_scaling: bool,
}

impl InitHints {
    pub fn new() -> Self {
        Self {
            vsync: true,
            fullscreen: false,
            frame_sleep_time: None,
            resize_scaling: false,
        }
    }
}
