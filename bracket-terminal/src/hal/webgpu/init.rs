use crate::{BResult, gamestate::BTerm, prelude::BACKEND_INTERNAL};
use super::{BACKEND, InitHints, Shader, WgpuLink, WrappedContext};
use wgpu::{Adapter, Device, Instance, Queue, Surface, SurfaceConfiguration};
use winit::{dpi::LogicalSize, event::*, event_loop::{ControlFlow, EventLoop}, window::{Window, WindowBuilder}};

pub fn init_raw<S: ToString>(
    width_pixels: u32,
    height_pixels: u32,
    window_title: S,
    platform_hints: InitHints,
) -> BResult<BTerm> {
    let el = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_title(window_title.to_string())
        .with_inner_size(LogicalSize::new(
            f64::from(width_pixels),
            f64::from(height_pixels),
        ));
    let window = wb.build(&el).unwrap();

    let (instance, surface, adapter, device, queue, config) = pollster::block_on(init_adapter(&window));

    /*let windowed_context = ContextBuilder::new()
        .with_gl(platform_hints.gl_version)
        .with_gl_profile(platform_hints.gl_profile)
        .with_hardware_acceleration(Some(true))
        .with_vsync(platform_hints.vsync)
        .with_srgb(platform_hints.srgb)
        .build_windowed(wb, &el)?;
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };*/

    /*if platform_hints.fullscreen {
        if let Some(mh) = el.available_monitors().next() {
            windowed_context
                .window()
                .set_fullscreen(Some(winit::window::Fullscreen::Borderless(Some(mh))));
        } else {
            return Err("No available monitor found".into());
        }
    }*/

    // Bunch more goes here

    // Shaders
    let mut shaders: Vec<Shader> = Vec::new();
    shaders.push(Shader::new(&device, include_str!("shader_source/console_with_vg.wgsl")));
    /*shaders.push(Shader::new(
        &gl,
        shader_strings::CONSOLE_NO_BG_VS,
        shader_strings::CONSOLE_NO_BG_FS,
    ));
    shaders.push(Shader::new(
        &gl,
        shader_strings::BACKING_VS,
        shader_strings::BACKING_FS,
    ));
    shaders.push(Shader::new(
        &gl,
        shader_strings::SCANLINES_VS,
        shader_strings::SCANLINES_FS,
    ));
    shaders.push(Shader::new(
        &gl,
        shader_strings::FANCY_CONSOLE_VS,
        shader_strings::FANCY_CONSOLE_FS,
    ));
    shaders.push(Shader::new(
        &gl,
        shader_strings::SPRITE_CONSOLE_VS,
        shader_strings::SPRITE_CONSOLE_FS,
    ));*/
    BACKEND_INTERNAL.lock().shaders = shaders;

    // Store the backend
    let mut be = BACKEND.lock();
    be.context_wrapper = Some(WrappedContext {
        el,
        window,
    });
    be.wgpu = Some(WgpuLink{
        instance, surface, adapter, device, queue, config,
    });
    be.frame_sleep_time = crate::hal::convert_fps_to_wait(platform_hints.frame_sleep_time);
    be.resize_scaling = platform_hints.resize_scaling;

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
        screen_burn_color: bracket_color::prelude::RGB::from_f32(0.0, 1.0, 1.0),
    };
    Ok(bterm)
}

async fn init_adapter(window: &Window) -> (Instance, Surface, Adapter, Device, Queue, SurfaceConfiguration) {
    let size = window.inner_size();

    // The instance is a handle to our GPU
    // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
    let instance = wgpu::Instance::new(wgpu::Backends::all());
    let surface = unsafe { instance.create_surface(window) };
    let adapter = instance.request_adapter(
        &wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        },
    ).await.unwrap();

    let (device, queue) = adapter.request_device(
        &wgpu::DeviceDescriptor {
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::default(),
            label: None,
        },
        None, // Trace path
    ).await.unwrap();

    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface.get_preferred_format(&adapter).unwrap(),
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
    };
    surface.configure(&device, &config);

    (instance, surface, adapter, device, queue, config)
}