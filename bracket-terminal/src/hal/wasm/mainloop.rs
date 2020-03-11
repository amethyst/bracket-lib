use super::events::*;
use super::*;
use crate::prelude::{BEvent, BTerm, GameState, SimpleConsole, SparseConsole, BACKEND_INTERNAL};
use crate::{clear_input_state, Result};
use glow::HasContext;

pub fn main_loop<GS: GameState>(mut bterm: BTerm, mut gamestate: GS) -> Result<()> {
    use glow::HasRenderLoop;

    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    {
        let be = BACKEND.lock();
        let gl = be.gl.as_ref().unwrap();
        for f in BACKEND_INTERNAL.lock().fonts.iter_mut() {
            f.setup_gl_texture(gl)?;
        }
    }

    let now = wasm_timer::Instant::now();
    let mut prev_seconds = now.elapsed().as_secs();
    let mut prev_ms = now.elapsed().as_millis();
    let mut frames = 0;

    let render_loop = glow::RenderLoop::from_request_animation_frame();
    render_loop.run(move |_running: &mut bool| {
        // Read in event results
        unsafe {
            bterm.key = GLOBAL_KEY;
            bterm.mouse_pos = (GLOBAL_MOUSE_POS.0, GLOBAL_MOUSE_POS.1);
            bterm.left_click = GLOBAL_LEFT_CLICK;
            bterm.shift = GLOBAL_MODIFIERS.0;
            bterm.control = GLOBAL_MODIFIERS.1;
            bterm.alt = GLOBAL_MODIFIERS.2;
            bterm.web_button = GLOBAL_BUTTON.clone();
            bterm.on_mouse_position(GLOBAL_MOUSE_POS.0 as f64, GLOBAL_MOUSE_POS.1 as f64);
        }

        // Call the tock function
        tock(
            &mut bterm,
            &mut gamestate,
            &mut frames,
            &mut prev_seconds,
            &mut prev_ms,
            &now,
        );

        // Clear any input
        clear_input_state(&mut bterm);
        unsafe {
            GLOBAL_KEY = None;
            GLOBAL_MODIFIERS = (false, false, false);
            GLOBAL_LEFT_CLICK = false;
            GLOBAL_BUTTON = None;
        }
    });
    Ok(())
}

fn check_console_backing() {
    let bi = BACKEND_INTERNAL.lock();
    let mut be = BACKEND.lock();
    let mut consoles = CONSOLE_BACKING.lock();
    if consoles.is_empty() {
        // Easy case: there are no consoles so we need to make them all.
        for cons in &bi.consoles {
            let cons_any = cons.console.as_any();
            if let Some(st) = cons_any.downcast_ref::<SimpleConsole>() {
                consoles.push(ConsoleBacking::Simple {
                    backing: SimpleConsoleBackend::new(
                        be.gl.as_mut().unwrap(),
                        st.width as usize,
                        st.height as usize,
                    ),
                });
            } else if let Some(sp) = cons_any.downcast_ref::<SparseConsole>() {
                consoles.push(ConsoleBacking::Sparse {
                    backing: SparseConsoleBackend::new(
                        be.gl.as_ref().unwrap(),
                        sp.width as usize,
                        sp.height as usize,
                    ),
                });
            } else {
                panic!("Unknown console type.");
            }
        }
    }
}

fn rebuild_consoles(bterm: &BTerm) {
    let mut bi = BACKEND_INTERNAL.lock();
    let be = BACKEND.lock();
    let gl = be.gl.as_ref().unwrap();
    let mut consoles = CONSOLE_BACKING.lock();
    for (i, c) in consoles.iter_mut().enumerate() {
        let font = &bi.fonts[bi.consoles[i].font_index];
        let shader = &bi.shaders[0];
        let has_background = bi.consoles[i].shader_index == 0;
        unsafe {
            bi.shaders[0].useProgram(gl);
            gl.active_texture(glow::TEXTURE0);
            font.bind_texture(gl);
            shader.setBool(gl, "showScanLines", bterm.post_scanlines);
            shader.setBool(gl, "screenBurn", bterm.post_screenburn);
            shader.setVec3(
                gl,
                "screenSize",
                bterm.width_pixels as f32,
                bterm.height_pixels as f32,
                0.0,
            );
        }

        match c {
            ConsoleBacking::Simple { backing } => {
                let mut sc = bi.consoles[i]
                    .console
                    .as_any_mut()
                    .downcast_mut::<SimpleConsole>()
                    .unwrap();
                if sc.is_dirty {
                    backing.rebuild_vertices(
                        gl,
                        sc.height,
                        sc.width,
                        &sc.tiles,
                        sc.offset_x,
                        sc.offset_y,
                        sc.scale,
                        sc.scale_center,
                        sc.needs_resize_internal,
                    );
                    sc.needs_resize_internal = false;
                }
            }
            ConsoleBacking::Sparse { backing } => {
                let mut sc = bi.consoles[i]
                    .console
                    .as_any_mut()
                    .downcast_mut::<SparseConsole>()
                    .unwrap();
                if sc.is_dirty {
                    backing.rebuild_vertices(
                        gl,
                        sc.height,
                        sc.width,
                        sc.offset_x,
                        sc.offset_y,
                        sc.scale,
                        sc.scale_center,
                        &sc.tiles,
                        has_background,
                        sc.needs_resize_internal,
                    );
                    sc.needs_resize_internal = false;
                }
            }
        }
    }
}

fn render_consoles() -> Result<()> {
    let bi = BACKEND_INTERNAL.lock();
    let be = BACKEND.lock();
    let gl = be.gl.as_ref().unwrap();
    let mut consoles = CONSOLE_BACKING.lock();
    unsafe {
        gl.enable(glow::BLEND);
        gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
    }
    for (i, c) in consoles.iter_mut().enumerate() {
        let cons = &bi.consoles[i];
        let font = &bi.fonts[cons.font_index];
        let shader = &bi.shaders[0];
        let has_background = bi.consoles[i].shader_index == 0;
        match c {
            ConsoleBacking::Simple { backing } => {
                unsafe {
                    bi.shaders[0].useProgram(gl);
                }
                let sc = bi.consoles[i]
                    .console
                    .as_any()
                    .downcast_ref::<SimpleConsole>()
                    .unwrap();
                backing.gl_draw(font, shader, gl, sc.width, sc.height)?;
            }
            ConsoleBacking::Sparse { backing } => {
                unsafe {
                    bi.shaders[0].useProgram(gl);
                }
                let sc = bi.consoles[i]
                    .console
                    .as_any()
                    .downcast_ref::<SparseConsole>()
                    .unwrap();
                backing.gl_draw(font, shader, gl, &sc.tiles, has_background)?;
            }
        }
    }
    unsafe {
        gl.disable(glow::BLEND);
    }
    Ok(())
}

fn tock<GS: GameState>(
    bterm: &mut BTerm,
    gamestate: &mut GS,
    frames: &mut i32,
    prev_seconds: &mut u64,
    prev_ms: &mut u128,
    now: &wasm_timer::Instant,
) {
    // Check that the console backings match our actual consoles
    check_console_backing();

    let now_seconds = now.elapsed().as_secs();
    *frames += 1;

    if now_seconds > *prev_seconds {
        bterm.fps = *frames as f32 / (now_seconds - *prev_seconds) as f32;
        *frames = 0;
        *prev_seconds = now_seconds;
    }

    let now_ms = now.elapsed().as_millis();
    if now_ms > *prev_ms {
        bterm.frame_time_ms = (now_ms - *prev_ms) as f32;
        *prev_ms = now_ms;
    }

    gamestate.tick(bterm);

    // Console structure - doesn't really have to be every frame...
    rebuild_consoles(bterm);

    {
        let be = BACKEND.lock();
        let gl = be.gl.as_ref().unwrap();

        // Clear the screen
        unsafe {
            gl.viewport(0, 0, bterm.width_pixels as i32, bterm.height_pixels as i32);
            gl.clear_color(0.2, 0.3, 0.3, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);
        }

        // Setup render pass

        unsafe {
            let bi = BACKEND_INTERNAL.lock();
            bi.shaders[0].useProgram(gl);

            gl.active_texture(glow::TEXTURE0);
            bi.fonts[0].bind_texture(gl);
            bi.shaders[0].setInt(gl, "texture1", 0);
            bi.shaders[0].setVec3(gl, "font", 8.0, 8.0, 0.0);

            gl.bind_vertex_array(be.quad_vao);
        }
    }

    // Tell each console to draw itself
    render_consoles().unwrap();

    // If there is a GL callback, call it now
    {
        let be = BACKEND.lock();
        if let Some(callback) = be.gl_callback.as_ref() {
            let gl = be.gl.as_ref().unwrap();
            callback(gamestate, gl);
        }
    }
}
