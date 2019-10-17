use glutin::{
    dpi::LogicalSize, event::Event, event::WindowEvent, event_loop::ControlFlow,
    event_loop::EventLoop, window::WindowBuilder, ContextBuilder,
};

#[cfg(not(target_arch = "wasm32"))]
pub fn init_raw<S: ToString>(
    width_pixels: u32,
    height_pixels: u32,
    window_title: S,
) -> super::super::super::Rltk {
    use super::super::super::Rltk;
    use super::super::*;

    let el = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_title(window_title.to_string())
        .with_inner_size(LogicalSize::new(
            f64::from(width_pixels),
            f64::from(height_pixels),
        ));
    let windowed_context = ContextBuilder::new()
        .with_vsync(true)
        .build_windowed(wb, &el)
        .unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    //let gl = glow::glow::load_with(|ptr| windowed_context.get_proc_address(ptr) as *const _);
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
    let backing_fbo = Framebuffer::build_fbo(&gl, width_pixels as i32, height_pixels as i32);

    // Build a simple quad rendering vao
    let quad_vao = setup_quad(&gl);

    Rltk {
        backend: RltkPlatform {
            gl,
            platform: PlatformGL {
                quad_vao: quad_vao,
                context_wrapper: Some(WrappedContext {
                    el,
                    wc: windowed_context,
                }),
                backing_buffer: backing_fbo,
            },
        },
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
        quitting: false,
        post_scanlines: false,
        post_screenburn: false,
    }
}
