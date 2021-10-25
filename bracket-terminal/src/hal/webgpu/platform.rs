use wgpu::{Adapter, Device, Instance, Queue, Surface, SurfaceConfiguration};
use winit::{event_loop::EventLoop, window::Window};

pub struct PlatformGL {
    pub context_wrapper: Option<WrappedContext>,
    pub wgpu: Option<WgpuLink>,

    pub frame_sleep_time: Option<u64>,
    pub resize_scaling: bool,
    pub resize_request: Option<(u32, u32)>,
    pub request_screenshot: Option<String>,
}

pub struct WgpuLink {
    pub instance: Instance,
    pub surface: Surface,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
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
