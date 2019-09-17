use super::{framebuffer::Framebuffer, quadrender, GameState, Rltk, Shader, shader_strings};
use image::GenericImageView;

#[cfg(not(target_arch = "wasm32"))]
use glutin::{
    dpi::LogicalSize, event::Event, event::WindowEvent, event_loop::ControlFlow,
    event_loop::EventLoop, window::WindowBuilder, ContextBuilder,
};

use glow::HasContext;
use std::time::Instant;

// Glutin version:

/// A helper, to get around difficulties with moving the event loop
/// and window context types.
#[cfg(not(target_arch = "wasm32"))]
pub struct WrappedContext {
    pub el: glutin::event_loop::EventLoop<()>,
    pub wc: glutin::WindowedContext<glutin::PossiblyCurrent>,
}

#[cfg(target_arch = "wasm32")]
pub struct WrappedContext {
}

// Glutin version of initialization
#[cfg(not(target_arch = "wasm32"))]
pub fn init_raw<S: ToString>(
    width_pixels: u32,
    height_pixels: u32,
    window_title: S
) -> Rltk {
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
        shader_strings::CONSOLE_WITH_BG_FS
    ));
    shaders.push(Shader::new(
        &gl,
        shader_strings::CONSOLE_NO_BG_VS,
        shader_strings::CONSOLE_NO_BG_FS
    ));
    shaders.push(Shader::new(
        &gl, 
        shader_strings::BACKING_VS,
        shader_strings::BACKING_FS
    ));
    shaders.push(Shader::new(
        &gl,
        shader_strings::SCANLINES_VS,
        shader_strings::SCANLINES_FS
    ));

    // Build the backing frame-buffer
    let backing_fbo = Framebuffer::build_fbo(&gl, width_pixels as i32, height_pixels as i32);

    // Build a simple quad rendering vao
    let quad_vao = quadrender::setup_quad(&gl);

    Rltk {
        gl,
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
        context_wrapper: Some(WrappedContext {
            el,
            wc: windowed_context,
        }),
        quitting: false,
        backing_buffer: backing_fbo,
        quad_vao,
        post_scanlines: false,
        post_screenburn: false,
    }
}

// Glutin version of main loop
#[cfg(not(target_arch = "wasm32"))]
pub fn main_loop<GS: GameState>(mut rltk: Rltk, mut gamestate: GS) {
    let now = Instant::now();
    let mut prev_seconds = now.elapsed().as_secs();
    let mut prev_ms = now.elapsed().as_millis();
    let mut frames = 0;

    // We're doing a little dance here to get around lifetime/borrow checking.
    // Removing the context data from RLTK in an atomic swap, so it isn't borrowed after move.
    let wrap = std::mem::replace(&mut rltk.context_wrapper, None);
    let unwrap = wrap.unwrap();

    let el = unwrap.el;
    let wc = unwrap.wc;

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if rltk.quitting {
            *control_flow = ControlFlow::Exit;
        }

        match event {
            Event::NewEvents(_) => {
                rltk.left_click = false;
                rltk.key = None;
            }
            Event::EventsCleared => {
                tock(
                    &mut rltk,
                    &mut gamestate,
                    &mut frames,
                    &mut prev_seconds,
                    &mut prev_ms,
                    &now,
                );
                wc.swap_buffers().unwrap();
            }
            Event::LoopDestroyed => (),
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(_logical_size) => {
                    // Commenting out to see if it helps the Linux world
                    //let dpi_factor = wc.window().hidpi_factor();
                    //wc.resize(logical_size.to_physical(dpi_factor));
                }
                WindowEvent::RedrawRequested => {
                    //tock(&mut rltk, &mut gamestate, &mut frames, &mut prev_seconds, &mut prev_ms, &now);
                    wc.swap_buffers().unwrap();
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                WindowEvent::CursorMoved { position: pos, .. } => {
                    rltk.mouse_pos = (pos.x as i32, pos.y as i32);
                }

                WindowEvent::MouseInput { .. } => {
                    rltk.left_click = true;
                }

                WindowEvent::KeyboardInput {
                    input:
                        glutin::event::KeyboardInput {
                            virtual_keycode: Some(virtual_keycode),
                            state: glutin::event::ElementState::Pressed,
                            ..
                        },
                    ..
                } => {
                    rltk.key = Some(*virtual_keycode);
                }

                _ => (),
            },
            _ => (),
        }
    });
}

/// Internal handling of the main loop.
fn tock<GS: GameState>(
    rltk: &mut Rltk,
    gamestate: &mut GS,
    frames: &mut i32,
    prev_seconds: &mut u64,
    prev_ms: &mut u128,
    now: &Instant,
) {
    let now_seconds = now.elapsed().as_secs();
    *frames += 1;

    if now_seconds > *prev_seconds {
        rltk.fps = *frames as f32 / (now_seconds - *prev_seconds) as f32;
        *frames = 0;
        *prev_seconds = now_seconds;
    }

    let now_ms = now.elapsed().as_millis();
    if now_ms > *prev_ms {
        rltk.frame_time_ms = (now_ms - *prev_ms) as f32;
        *prev_ms = now_ms;
    }

    gamestate.tick(rltk);

    // Console structure - doesn't really have to be every frame...
    for cons in &mut rltk.consoles {
        cons.console.rebuild_if_dirty(&rltk.gl);
    }

    // Bind to the backing buffer
    if rltk.post_scanlines {
        rltk.backing_buffer.bind(&rltk.gl);
    }

    // Clear the screen
    unsafe {
        rltk.gl.clear_color(0.2, 0.3, 0.3, 1.0);
        rltk.gl.clear(glow::COLOR_BUFFER_BIT);
    }

    // Tell each console to draw itself
    for cons in &mut rltk.consoles {
        let font = &rltk.fonts[cons.font_index];
        let shader = &rltk.shaders[cons.shader_index];
        cons.console.gl_draw(font, shader, &rltk.gl);
    }

    if rltk.post_scanlines {
        // Now we return to the primary screen
        rltk.backing_buffer.default(&rltk.gl);
        unsafe {
            if rltk.post_scanlines {
                rltk.shaders[3].useProgram(&rltk.gl);
                rltk.shaders[3].setVec3(
                    &rltk.gl,
                    "screenSize",
                    rltk.width_pixels as f32,
                    rltk.height_pixels as f32,
                    0.0,
                );
                rltk.shaders[3].setBool(&rltk.gl, "screenBurn", rltk.post_screenburn);
            } else {
                rltk.shaders[2].useProgram(&rltk.gl);
            }
            rltk.gl.bind_vertex_array(Some(rltk.quad_vao));
            rltk.gl
                .bind_texture(glow::TEXTURE_2D, Some(rltk.backing_buffer.texture));
            rltk.gl.draw_arrays(glow::TRIANGLES, 0, 6);
        }
    }
}

// wasm version of initialization
#[cfg(target_arch = "wasm32")]
pub fn init_raw<S: ToString>(
    width_pixels: u32,
    height_pixels: u32,
    window_title: S
) -> Rltk {
    use wasm_bindgen::JsCast;
    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    let webgl2_context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::WebGl2RenderingContext>()
        .unwrap();

    let gl = glow::Context::from_webgl2_context(webgl2_context);

    // Load our basic shaders
    let mut shaders: Vec<Shader> = Vec::new();

    shaders.push(Shader::new(
        &gl,
        shader_strings::CONSOLE_WITH_BG_VS,
        shader_strings::CONSOLE_WITH_BG_FS
    ));
    shaders.push(Shader::new(
        &gl,
        shader_strings::CONSOLE_NO_BG_VS,
        shader_strings::CONSOLE_NO_BG_FS
    ));
    shaders.push(Shader::new(
        &gl, 
        shader_strings::BACKING_VS,
        shader_strings::BACKING_FS
    ));
    shaders.push(Shader::new(
        &gl,
        shader_strings::SCANLINES_VS,
        shader_strings::SCANLINES_FS
    ));

    // Build the backing frame-buffer
    let backing_fbo = Framebuffer::build_fbo(&gl, width_pixels as i32, height_pixels as i32);

    // Build a simple quad rendering vao
    let quad_vao = quadrender::setup_quad(&gl);

    Rltk {
        gl,
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
        context_wrapper: Some(WrappedContext {}),
        quitting: false,
        backing_buffer: backing_fbo,
        quad_vao,
        post_scanlines: false,
        post_screenburn: false,
    }
}

// WASM version of main loop
#[cfg(target_arch = "wasm32")]
pub fn main_loop<GS: GameState>(mut rltk: Rltk, mut gamestate: GS) {
    use glow::HasRenderLoop;

    let now = Instant::now();
    let mut prev_seconds = now.elapsed().as_secs();
    let mut prev_ms = now.elapsed().as_millis();
    let mut frames = 0;

    // We're doing a little dance here to get around lifetime/borrow checking.
    // Removing the context data from RLTK in an atomic swap, so it isn't borrowed after move.
    let wrap = std::mem::replace(&mut rltk.context_wrapper, None);
    let unwrap = wrap.unwrap();

    let render_loop = glow::RenderLoop::from_request_animation_frame();
    render_loop.run(move |running: &mut bool| {
        tock(
            &mut rltk,
            &mut gamestate,
            &mut frames,
            &mut prev_seconds,
            &mut prev_ms,
            &now,
        );
    });
}

// Generic/macro version of setup_gl_texture for platform binding

macro_rules! setup_gl_texture {
    ($type:ty) => {
        pub fn setup_gl_texture(gl: &glow::Context, bitmap_file: &str) -> $type {
            let texture;

            unsafe {
                texture = gl.create_texture().unwrap();
                gl.bind_texture(glow::TEXTURE_2D, Some(texture));
                gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::REPEAT as i32);
                gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::REPEAT as i32);
                // set texture filtering parameters
                gl.tex_parameter_i32(
                    glow::TEXTURE_2D,
                    glow::TEXTURE_MIN_FILTER,
                    glow::LINEAR as i32,
                );
                gl.tex_parameter_i32(
                    glow::TEXTURE_2D,
                    glow::TEXTURE_MAG_FILTER,
                    glow::LINEAR as i32,
                );

                let img_orig = image::open(std::path::Path::new(&bitmap_file))
                    .expect("Failed to load texture");
                let img = img_orig.flipv();
                let data = img.raw_pixels();
                gl.tex_image_2d(
                    glow::TEXTURE_2D,
                    0,
                    glow::RGB as i32,
                    img.width() as i32,
                    img.height() as i32,
                    0,
                    glow::RGB,
                    glow::UNSIGNED_BYTE,
                    Some(&data),
                );
                //gl.GenerateMipmap(glow::TEXTURE_2D);
            }

            texture
        }
    };
}

// Font support: glutin
#[cfg(not(target_arch = "wasm32"))]
setup_gl_texture!(u32);

// Font support: wasm (glow::WebTextureKey)
#[cfg(target_arch = "wasm32")]
setup_gl_texture!(glow::WebTextureKey);

// For web assembly only, export a copy of the VirtualKeyCode type

#[cfg(target_arch = "wasm32")]
#[derive(Debug, Hash, Ord, PartialOrd, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum VirtualKeyCode {
    /// The '1' key over the letters.
    Key1,
    /// The '2' key over the letters.
    Key2,
    /// The '3' key over the letters.
    Key3,
    /// The '4' key over the letters.
    Key4,
    /// The '5' key over the letters.
    Key5,
    /// The '6' key over the letters.
    Key6,
    /// The '7' key over the letters.
    Key7,
    /// The '8' key over the letters.
    Key8,
    /// The '9' key over the letters.
    Key9,
    /// The '0' key over the 'O' and 'P' keys.
    Key0,

    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    /// The Escape key, next to F1.
    Escape,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,

    /// Print Screen/SysRq.
    Snapshot,
    /// Scroll Lock.
    Scroll,
    /// Pause/Break key, next to Scroll lock.
    Pause,

    /// `Insert`, next to Backspace.
    Insert,
    Home,
    Delete,
    End,
    PageDown,
    PageUp,

    Left,
    Up,
    Right,
    Down,

    /// The Backspace key, right over Enter.
    // TODO: rename
    Back,
    /// The Enter key.
    Return,
    /// The space bar.
    Space,

    /// The "Compose" key on Linux.
    Compose,

    Caret,

    Numlock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,

    AbntC1,
    AbntC2,
    Add,
    Apostrophe,
    Apps,
    At,
    Ax,
    Backslash,
    Calculator,
    Capital,
    Colon,
    Comma,
    Convert,
    Decimal,
    Divide,
    Equals,
    Grave,
    Kana,
    Kanji,
    LAlt,
    LBracket,
    LControl,
    LShift,
    LWin,
    Mail,
    MediaSelect,
    MediaStop,
    Minus,
    Multiply,
    Mute,
    MyComputer,
    NavigateForward,  // also called "Prior"
    NavigateBackward, // also called "Next"
    NextTrack,
    NoConvert,
    NumpadComma,
    NumpadEnter,
    NumpadEquals,
    OEM102,
    Period,
    PlayPause,
    Power,
    PrevTrack,
    RAlt,
    RBracket,
    RControl,
    RShift,
    RWin,
    Semicolon,
    Slash,
    Sleep,
    Stop,
    Subtract,
    Sysrq,
    Tab,
    Underline,
    Unlabeled,
    VolumeDown,
    VolumeUp,
    Wake,
    WebBack,
    WebFavorites,
    WebForward,
    WebHome,
    WebRefresh,
    WebSearch,
    WebStop,
    Yen,
    Copy,
    Paste,
    Cut,
}
