use super::{Rltk, Shader, gl, framebuffer::Framebuffer, quadrender, GameState};
use glutin::dpi::LogicalSize;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use std::time::Instant;
use std::ffi::CString;

/// A helper, to get around difficulties with moving the event loop
/// and window context types.
pub struct WrappedContext {
    pub el: glutin::event_loop::EventLoop<()>,
    pub wc: glutin::WindowedContext<glutin::PossiblyCurrent>,
}

pub fn init_raw<S: ToString>(
    width_pixels: u32,
    height_pixels: u32,
    window_title: S,
    path_to_shaders: S,
) -> Rltk {
    let el = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_title(window_title.to_string())
        .with_inner_size(LogicalSize::new(f64::from(width_pixels), f64::from(height_pixels)));
    let windowed_context = ContextBuilder::new().with_vsync(true).build_windowed(wb, &el).unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    let gl = gl::Gl::load_with(|ptr| windowed_context.get_proc_address(ptr) as *const _);

    // Load our basic shaders
    let mut shaders: Vec<Shader> = Vec::new();

    let shader_path = path_to_shaders.to_string();
    shaders.push(Shader::new(
        &gl,
        "console_with_bg.vs",
        "console_with_bg.fs",
        &shader_path,
    ));
    shaders.push(Shader::new(
        &gl,
        "console_no_bg.vs",
        "console_no_bg.fs",
        &shader_path,
    ));
    shaders.push(Shader::new(&gl, "backing.vs", "backing.fs", &shader_path));
    shaders.push(Shader::new(
        &gl,
        "scanlines.vs",
        "scanlines.fs",
        &shader_path,
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
        *control_flow = ControlFlow::Poll;

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
            Event::LoopDestroyed => return,
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
        rltk.gl.ClearColor(0.2, 0.3, 0.3, 1.0);
        rltk.gl.Clear(gl::COLOR_BUFFER_BIT);
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
                    &CString::new("screenSize").unwrap(),
                    rltk.width_pixels as f32,
                    rltk.height_pixels as f32,
                    0.0,
                );
                rltk.shaders[3].setBool(
                    &rltk.gl,
                    &CString::new("screenBurn").unwrap(),
                    rltk.post_screenburn,
                );
            } else {
                rltk.shaders[2].useProgram(&rltk.gl);
            }
            rltk.gl.BindVertexArray(rltk.quad_vao);
            rltk.gl
                .BindTexture(gl::TEXTURE_2D, rltk.backing_buffer.texture);
            rltk.gl.DrawArrays(gl::TRIANGLES, 0, 6);
        }
    }
}