use super::{framebuffer::Framebuffer, quadrender, shader_strings, GameState, Rltk, Shader};



#[cfg(not(target_arch = "wasm32"))]
use glutin::{
    dpi::LogicalSize, event::Event, event::WindowEvent, event_loop::ControlFlow,
    event_loop::EventLoop, window::WindowBuilder, ContextBuilder,
};

use glow::HasContext;

#[cfg(not(target_arch = "wasm32"))]
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
pub struct WrappedContext {}

// Glutin version of initialization
#[cfg(not(target_arch = "wasm32"))]
pub fn init_raw<S: ToString>(width_pixels: u32, height_pixels: u32, window_title: S) -> Rltk {
    let el = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_title(window_title.to_string())
        .with_max_inner_size(LogicalSize::new(
            f64::from(width_pixels),
            f64::from(height_pixels),
        ))
        .with_min_inner_size(LogicalSize::new(
            f64::from(width_pixels),
            f64::from(height_pixels),
        ))
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

#[cfg(not(target_arch = "wasm32"))]
const TICK_TYPE : ControlFlow = ControlFlow::Poll;

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
        *control_flow = TICK_TYPE;

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
#[cfg(not(target_arch = "wasm32"))]
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

#[cfg(target_arch = "wasm32")]
fn tock<GS: GameState>(
    rltk: &mut Rltk,
    gamestate: &mut GS,
    frames: &mut i32,
    prev_seconds: &mut u64,
    prev_ms: &mut u128,
    now: &wasm_timer::Instant,
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
pub fn init_raw<S: ToString>(width_pixels: u32, height_pixels: u32, _window_title: S) -> Rltk {
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;

    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    canvas.set_width(width_pixels);
    canvas.set_height(height_pixels);

    // Handle keyboard input
    let key_callback = Closure::wrap(Box::new(|e: web_sys::KeyboardEvent| {
        on_key(e.clone());
    }) as Box<dyn FnMut(_)>);

    let document = web_sys::window().unwrap();
    document.set_onkeydown(Some(key_callback.as_ref().unchecked_ref()));;
    key_callback.forget();

    // Handle mouse moving
    let mousemove_callback = Closure::wrap(Box::new(|e: web_sys::MouseEvent| {
        on_mouse_move(e.clone());
    }) as Box<dyn FnMut(_)>);

    canvas.set_onmousemove(Some(mousemove_callback.as_ref().unchecked_ref()));;
    mousemove_callback.forget();

    // Handle mouse clicking
    let mouseclick_callback = Closure::wrap(Box::new(|e: web_sys::MouseEvent| {
        on_mouse_down(e.clone());
    }) as Box<dyn FnMut(_)>);

    canvas.set_onmousedown(Some(mouseclick_callback.as_ref().unchecked_ref()));;
    mouseclick_callback.forget();

    let webgl2_context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::WebGl2RenderingContext>()
        .unwrap();
    webgl2_context
        .get_extension("EXT_color_buffer_float")
        .expect("Unable to add extensions");

    let gl = glow::Context::from_webgl2_context(webgl2_context);

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
static mut GLOBAL_KEY: Option<VirtualKeyCode> = None;

#[cfg(target_arch = "wasm32")]
fn on_key(key: web_sys::KeyboardEvent) {
    //super::console::log("Key Event");
    unsafe {
        let code = key.key_code();
        match code {
            8 => GLOBAL_KEY = Some(VirtualKeyCode::Back),
            9 => GLOBAL_KEY = Some(VirtualKeyCode::Tab),
            13 => GLOBAL_KEY = Some(VirtualKeyCode::Return),
            20 => GLOBAL_KEY = Some(VirtualKeyCode::Capital),
            27 => GLOBAL_KEY = Some(VirtualKeyCode::Escape),
            32 => GLOBAL_KEY = Some(VirtualKeyCode::Space),
            33 => GLOBAL_KEY = Some(VirtualKeyCode::PageUp),
            34 => GLOBAL_KEY = Some(VirtualKeyCode::PageDown),
            35 => GLOBAL_KEY = Some(VirtualKeyCode::End),
            36 => GLOBAL_KEY = Some(VirtualKeyCode::Home),
            37 => GLOBAL_KEY = Some(VirtualKeyCode::Left),
            38 => GLOBAL_KEY = Some(VirtualKeyCode::Up),
            39 => GLOBAL_KEY = Some(VirtualKeyCode::Right),
            40 => GLOBAL_KEY = Some(VirtualKeyCode::Down),
            45 => GLOBAL_KEY = Some(VirtualKeyCode::Insert),
            46 => GLOBAL_KEY = Some(VirtualKeyCode::Delete),
            48 => GLOBAL_KEY = Some(VirtualKeyCode::Key0),
            49 => GLOBAL_KEY = Some(VirtualKeyCode::Key1),
            50 => GLOBAL_KEY = Some(VirtualKeyCode::Key2),
            51 => GLOBAL_KEY = Some(VirtualKeyCode::Key3),
            52 => GLOBAL_KEY = Some(VirtualKeyCode::Key4),
            53 => GLOBAL_KEY = Some(VirtualKeyCode::Key5),
            54 => GLOBAL_KEY = Some(VirtualKeyCode::Key6),
            55 => GLOBAL_KEY = Some(VirtualKeyCode::Key7),
            56 => GLOBAL_KEY = Some(VirtualKeyCode::Key8),
            57 => GLOBAL_KEY = Some(VirtualKeyCode::Key9),
            65 => GLOBAL_KEY = Some(VirtualKeyCode::A),
            66 => GLOBAL_KEY = Some(VirtualKeyCode::B),
            67 => GLOBAL_KEY = Some(VirtualKeyCode::C),
            68 => GLOBAL_KEY = Some(VirtualKeyCode::D),
            69 => GLOBAL_KEY = Some(VirtualKeyCode::E),
            70 => GLOBAL_KEY = Some(VirtualKeyCode::F),
            71 => GLOBAL_KEY = Some(VirtualKeyCode::G),
            72 => GLOBAL_KEY = Some(VirtualKeyCode::H),
            73 => GLOBAL_KEY = Some(VirtualKeyCode::I),
            74 => GLOBAL_KEY = Some(VirtualKeyCode::J),
            75 => GLOBAL_KEY = Some(VirtualKeyCode::K),
            76 => GLOBAL_KEY = Some(VirtualKeyCode::L),
            77 => GLOBAL_KEY = Some(VirtualKeyCode::M),
            78 => GLOBAL_KEY = Some(VirtualKeyCode::N),
            79 => GLOBAL_KEY = Some(VirtualKeyCode::O),
            80 => GLOBAL_KEY = Some(VirtualKeyCode::P),
            81 => GLOBAL_KEY = Some(VirtualKeyCode::Q),
            82 => GLOBAL_KEY = Some(VirtualKeyCode::R),
            83 => GLOBAL_KEY = Some(VirtualKeyCode::S),
            84 => GLOBAL_KEY = Some(VirtualKeyCode::T),
            85 => GLOBAL_KEY = Some(VirtualKeyCode::U),
            86 => GLOBAL_KEY = Some(VirtualKeyCode::V),
            87 => GLOBAL_KEY = Some(VirtualKeyCode::W),
            88 => GLOBAL_KEY = Some(VirtualKeyCode::X),
            89 => GLOBAL_KEY = Some(VirtualKeyCode::Y),
            90 => GLOBAL_KEY = Some(VirtualKeyCode::Z),
            97 => GLOBAL_KEY = Some(VirtualKeyCode::Numpad1),
            98 => GLOBAL_KEY = Some(VirtualKeyCode::Numpad2),
            99 => GLOBAL_KEY = Some(VirtualKeyCode::Numpad3),
            100 => GLOBAL_KEY = Some(VirtualKeyCode::Numpad4),
            101 => GLOBAL_KEY = Some(VirtualKeyCode::Numpad5),
            102 => GLOBAL_KEY = Some(VirtualKeyCode::Numpad6),
            103 => GLOBAL_KEY = Some(VirtualKeyCode::Numpad7),
            104 => GLOBAL_KEY = Some(VirtualKeyCode::Numpad8),
            105 => GLOBAL_KEY = Some(VirtualKeyCode::Numpad9),
            106 => GLOBAL_KEY = Some(VirtualKeyCode::Multiply),
            107 => GLOBAL_KEY = Some(VirtualKeyCode::Add),
            109 => GLOBAL_KEY = Some(VirtualKeyCode::Subtract),
            111 => GLOBAL_KEY = Some(VirtualKeyCode::Divide),
            186 => GLOBAL_KEY = Some(VirtualKeyCode::Semicolon),
            187 => GLOBAL_KEY = Some(VirtualKeyCode::Equals),
            188 => GLOBAL_KEY = Some(VirtualKeyCode::Comma),
            189 => GLOBAL_KEY = Some(VirtualKeyCode::Minus),
            190 => GLOBAL_KEY = Some(VirtualKeyCode::Period),
            191 => GLOBAL_KEY = Some(VirtualKeyCode::Slash),
            192 => GLOBAL_KEY = Some(VirtualKeyCode::Grave),
            219 => GLOBAL_KEY = Some(VirtualKeyCode::LBracket),
            221 => GLOBAL_KEY = Some(VirtualKeyCode::RBracket),
            222 => GLOBAL_KEY = Some(VirtualKeyCode::Apostrophe),
            _ => {
                GLOBAL_KEY = None;
                super::console::log(&format!("Keycode: {}", code));
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
static mut GLOBAL_MOUSE_POS: (i32, i32) = (0, 0);

#[cfg(target_arch = "wasm32")]
fn on_mouse_move(mouse: web_sys::MouseEvent) {
    unsafe {
        GLOBAL_MOUSE_POS = (mouse.offset_x(), mouse.offset_y());
    }
}

#[cfg(target_arch = "wasm32")]
static mut GLOBAL_LEFT_CLICK: bool = false;

#[cfg(target_arch = "wasm32")]
fn on_mouse_down(_mouse: web_sys::MouseEvent) {
    unsafe {
        GLOBAL_LEFT_CLICK = true;
    }
}

#[cfg(target_arch = "wasm32")]
pub fn main_loop<GS: GameState>(mut rltk: Rltk, mut gamestate: GS) {
    use glow::HasRenderLoop;

    let now = wasm_timer::Instant::now();
    let mut prev_seconds = now.elapsed().as_secs();
    let mut prev_ms = now.elapsed().as_millis();
    let mut frames = 0;

    let render_loop = glow::RenderLoop::from_request_animation_frame();
    render_loop.run(move |_running: &mut bool| {
        // Read in event results
        unsafe {
            rltk.key = GLOBAL_KEY;
            rltk.mouse_pos = (GLOBAL_MOUSE_POS.0, GLOBAL_MOUSE_POS.1);
            rltk.left_click = GLOBAL_LEFT_CLICK;
        }

        // Call the tock function
        tock(
            &mut rltk,
            &mut gamestate,
            &mut frames,
            &mut prev_seconds,
            &mut prev_ms,
            &now,
        );

        // Clear any input
        rltk.left_click = false;
        rltk.key = None;
        unsafe {
            GLOBAL_KEY = None;
            GLOBAL_LEFT_CLICK = false;
        }
    });
}

// For web assembly only, export a copy of the VirtualKeyCode type

#[cfg(target_arch = "wasm32")]
#[derive(Debug, Hash, Ord, PartialOrd, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
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
