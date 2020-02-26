use super::BACKEND;
use crate::hal::native::{setup_quad, shader::Shader, shader_strings, WrappedContext};
use crate::hal::Framebuffer;
use crate::prelude::{BTerm, InitHints};
use crate::Result;
use glutin::{dpi::LogicalSize, event_loop::EventLoop, window::WindowBuilder, ContextBuilder};

pub fn init_raw<S: ToString>(
    width_pixels: u32,
    height_pixels: u32,
    window_title: S,
    platform_hints: InitHints,
) -> Result<BTerm> {
    let el = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_title(window_title.to_string())
        .with_inner_size(LogicalSize::new(
            f64::from(width_pixels),
            f64::from(height_pixels),
        ));
    let windowed_context = ContextBuilder::new()
        .with_gl(platform_hints.gl_version)
        .with_gl_profile(platform_hints.gl_profile)
        .with_hardware_acceleration(Some(true))
        .with_vsync(platform_hints.vsync)
        .with_srgb(platform_hints.srgb)
        .build_windowed(wb, &el)?;
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    if platform_hints.fullscreen {
        if let Some(mh) = el.available_monitors().nth(0) {
            windowed_context
                .window()
                .set_fullscreen(Some(glutin::window::Fullscreen::Borderless(mh)));
        } else {
            return Err("No available monitor found".into());
        }
    }

    let gl = glow::Context::from_loader_function(|ptr| {
        windowed_context.get_proc_address(ptr) as *const _
    });

    // Load our basic shaders
    let mut shaders: Vec<Shader> = Vec::new();

    shaders.push(Shader::new(
        &gl,
        shader_strings::CONSOLE_WITH_BG_VS,
        shader_strings::CONSOLE_WITH_BG_FS,
    ));
    shaders.push(Shader::new(
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

    // Build the backing frame-buffer
    let initial_dpi_factor = windowed_context.window().scale_factor();
    let backing_fbo = Framebuffer::build_fbo(
        &gl,
        (width_pixels as f64 * initial_dpi_factor) as i32,
        (height_pixels as f64 * initial_dpi_factor) as i32,
    )?;

    // Build a simple quad rendering vao
    let quad_vao = setup_quad(&gl);

    let mut be = BACKEND.lock().unwrap();
    be.gl = Some(gl);
    be.quad_vao = Some(quad_vao);
    be.context_wrapper = Some(WrappedContext {
        el,
        wc: windowed_context,
    });
    be.backing_buffer = Some(backing_fbo);
    be.frame_sleep_time = crate::hal::convert_fps_to_wait(platform_hints.frame_sleep_time);

    let bterm = BTerm {
        width_pixels,
        height_pixels,
        fonts: Vec::new(),
        consoles: Vec::new(),
        shaders,
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
    };
    Ok(bterm)
}
