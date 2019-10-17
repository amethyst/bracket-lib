use super::{framebuffer::Framebuffer, quadrender, shader_strings, GameState, Rltk, Shader, Console,
RltkPlatform, PlatformGL, hal::WrappedContext};

#[cfg(target_arch = "wasm32")]
use super::hal::VirtualKeyCode;

#[cfg(not(target_arch = "wasm32"))]
use glutin::{
    dpi::LogicalSize, event::Event, event::WindowEvent, event_loop::ControlFlow,
    event_loop::EventLoop, window::WindowBuilder, ContextBuilder,
};

use glow::HasContext;

#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

// Glutin version of initialization
#[cfg(not(target_arch = "wasm32"))]
pub fn init_raw<S: ToString>(width_pixels: u32, height_pixels: u32, window_title: S) -> Rltk {
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
    let quad_vao = quadrender::setup_quad(&gl);

    Rltk {
        backend: RltkPlatform{gl, 
            platform: PlatformGL{ quad_vao: quad_vao,
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

#[cfg(not(target_arch = "wasm32"))]
const TICK_TYPE : ControlFlow = ControlFlow::Poll;

// Glutin version of main loop
#[cfg(not(target_arch = "wasm32"))]
pub fn main_loop<GS: GameState>(mut rltk: Rltk, mut gamestate: GS) {
    unsafe {
        rltk.backend.gl.viewport(0, 0, rltk.width_pixels as i32, rltk.height_pixels as i32);
    }
    let now = Instant::now();
    let mut prev_seconds = now.elapsed().as_secs();
    let mut prev_ms = now.elapsed().as_millis();
    let mut frames = 0;

    // We're doing a little dance here to get around lifetime/borrow checking.
    // Removing the context data from RLTK in an atomic swap, so it isn't borrowed after move.
    let wrap = std::mem::replace(&mut rltk.backend.platform.context_wrapper, None);
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
                rltk.shift = false;
                rltk.control = false;
                rltk.alt = false;
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
                WindowEvent::Resized(logical_size) => {
                    // Commenting out to see if it helps the Linux world
                    let dpi_factor = wc.window().hidpi_factor();
                    let physical = logical_size.to_physical(dpi_factor);
                    wc.resize(physical);
                    rltk.resize_pixels(physical.width as u32, physical.height as u32);
                    unsafe {
                        rltk.backend.gl.viewport(0, 0, physical.width as i32, physical.height as i32);
                    }
                    rltk.backend.platform.backing_buffer = Framebuffer::build_fbo(&rltk.backend.gl, physical.width as i32, physical.height as i32);
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
                            modifiers,
                            ..
                        },
                    ..
                } => {
                    rltk.key = Some(*virtual_keycode);
                    if modifiers.shift { rltk.shift = true; }
                    if modifiers.alt { rltk.alt = true; }
                    if modifiers.ctrl { rltk.control = true; }
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
        cons.console.rebuild_if_dirty(&rltk.backend.gl);
    }

    // Bind to the backing buffer
    if rltk.post_scanlines {
        rltk.backend.platform.backing_buffer.bind(&rltk.backend.gl);
    }

    // Clear the screen
    unsafe {
        rltk.backend.gl.clear_color(0.0, 0.0, 0.0, 1.0);
        rltk.backend.gl.clear(glow::COLOR_BUFFER_BIT);
    }

    // Tell each console to draw itself
    for cons in &mut rltk.consoles {
        let font = &rltk.fonts[cons.font_index];
        let shader = &rltk.shaders[cons.shader_index];
        cons.console.gl_draw(font, shader, &rltk.backend.gl);
    }

    if rltk.post_scanlines {
        // Now we return to the primary screen
        rltk.backend.platform.backing_buffer.default(&rltk.backend.gl);
        unsafe {
            if rltk.post_scanlines {
                rltk.shaders[3].useProgram(&rltk.backend.gl);
                rltk.shaders[3].setVec3(
                    &rltk.backend.gl,
                    "screenSize",
                    rltk.width_pixels as f32,
                    rltk.height_pixels as f32,
                    0.0,
                );
                rltk.shaders[3].setBool(&rltk.backend.gl, "screenBurn", rltk.post_screenburn);
            } else {
                rltk.shaders[2].useProgram(&rltk.backend.gl);
            }
            rltk.backend.gl.bind_vertex_array(Some(rltk.backend.platform.quad_vao));
            rltk.backend.gl
                .bind_texture(glow::TEXTURE_2D, Some(rltk.backend.platform.backing_buffer.texture));
            rltk.backend.gl.draw_arrays(glow::TRIANGLES, 0, 6);
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
        cons.console.rebuild_if_dirty(&rltk.backend.gl);
    }

    // Bind to the backing buffer
    if rltk.post_scanlines {
        rltk.backend.platform.backing_buffer.bind(&rltk.backend.gl);
    }

    // Clear the screen
    unsafe {
        rltk.backend.gl.viewport(0, 0, rltk.width_pixels as i32, rltk.height_pixels as i32);
        rltk.backend.gl.clear_color(0.2, 0.3, 0.3, 1.0);
        rltk.backend.gl.clear(glow::COLOR_BUFFER_BIT);
    }

    // Tell each console to draw itself
    for cons in &mut rltk.consoles {
        let font = &rltk.fonts[cons.font_index];
        let shader = &rltk.shaders[cons.shader_index];
        cons.console.gl_draw(font, shader, &rltk.backend.gl);
    }

    if rltk.post_scanlines {
        // Now we return to the primary screen
        rltk.backend.platform.backing_buffer.default(&rltk.backend.gl);
        unsafe {
            if rltk.post_scanlines {
                rltk.shaders[3].useProgram(&rltk.backend.gl);
                rltk.shaders[3].setVec3(
                    &rltk.backend.gl,
                    "screenSize",
                    rltk.width_pixels as f32,
                    rltk.height_pixels as f32,
                    0.0,
                );
                rltk.shaders[3].setBool(&rltk.backend.gl, "screenBurn", rltk.post_screenburn);
            } else {
                rltk.shaders[2].useProgram(&rltk.backend.gl);
            }
            rltk.backend.gl.bind_vertex_array(Some(rltk.backend.platform.quad_vao));
            rltk.backend.gl
                .bind_texture(glow::TEXTURE_2D, Some(rltk.backend.platform.backing_buffer.texture));
            rltk.backend.gl.draw_arrays(glow::TRIANGLES, 0, 6);
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
        backend: RltkPlatform{ gl, 
            platform: PlatformGL{ 
                quad_vao, 
                context_wrapper: Some(WrappedContext {}), 
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
        alt: false,
        control: false,
        quitting: false,        
        post_scanlines: false,
        post_screenburn: false,
    }
}

// WASM version of main loop
#[cfg(target_arch = "wasm32")]
static mut GLOBAL_KEY: Option<VirtualKeyCode> = None;

#[cfg(target_arch = "wasm32")]
static mut GLOBAL_MODIFIERS: (bool, bool, bool) = (false, false, false);

#[cfg(target_arch = "wasm32")]
fn on_key(key: web_sys::KeyboardEvent) {
    //super::console::log("Key Event");
    unsafe {
        if key.get_modifier_state("Shift") { GLOBAL_MODIFIERS.0 = true; }
        if key.get_modifier_state("Control") { GLOBAL_MODIFIERS.1 = true; }
        if key.get_modifier_state("Alt") { GLOBAL_MODIFIERS.2 = true; }

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
            rltk.shift = GLOBAL_MODIFIERS.0;
            rltk.control = GLOBAL_MODIFIERS.1;
            rltk.alt = GLOBAL_MODIFIERS.2;
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
            GLOBAL_MODIFIERS = (false, false, false);
            GLOBAL_LEFT_CLICK = false;
        }
    });
}

