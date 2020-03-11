use super::events::*;
use super::*;
use crate::prelude::{
    BEvent, BTerm, FancyConsole, GameState, SimpleConsole, SparseConsole, BACKEND_INTERNAL,
};
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
    let mut be = BACKEND.lock();
    let mut consoles = CONSOLE_BACKING.lock();
    if consoles.is_empty() {
        // Easy case: there are no consoles so we need to make them all.
        for cons in &BACKEND_INTERNAL.lock().consoles {
            let cons_any = cons.console.as_any();
            if let Some(st) = cons_any.downcast_ref::<SimpleConsole>() {
                consoles.push(ConsoleBacking::Simple {
                    backing: SimpleConsoleBackend::new(
                        st.width as usize,
                        st.height as usize,
                        be.gl.as_mut().unwrap(),
                    ),
                });
            } else if let Some(sp) = cons_any.downcast_ref::<SparseConsole>() {
                consoles.push(ConsoleBacking::Sparse {
                    backing: SparseConsoleBackend::new(
                        sp.width as usize,
                        sp.height as usize,
                        be.gl.as_ref().unwrap(),
                    ),
                });
            } else if let Some(sp) = cons_any.downcast_ref::<FancyConsole>() {
                consoles.push(ConsoleBacking::Fancy {
                    backing: FancyConsoleBackend::new(
                        sp.width as usize,
                        sp.height as usize,
                        be.gl.as_ref().unwrap(),
                    ),
                });
            } else {
                panic!("Unknown console type.");
            }
        }
    }
}

fn rebuild_consoles() {
    let mut consoles = CONSOLE_BACKING.lock();
    let mut bi = BACKEND_INTERNAL.lock();
    for (i, c) in consoles.iter_mut().enumerate() {
        let font_index = bi.consoles[i].font_index;
        let glyph_dimensions = bi.fonts[font_index].font_dimensions_glyphs;
        let cons = &mut bi.consoles[i];
        match c {
            ConsoleBacking::Simple { backing } => {
                let mut sc = cons
                    .console
                    .as_any_mut()
                    .downcast_mut::<SimpleConsole>()
                    .unwrap();
                if sc.is_dirty {
                    backing.rebuild_vertices(
                        sc.height,
                        sc.width,
                        &sc.tiles,
                        sc.offset_x,
                        sc.offset_y,
                        sc.scale,
                        sc.scale_center,
                        sc.needs_resize_internal,
                        glyph_dimensions,
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
                        sc.height,
                        sc.width,
                        sc.offset_x,
                        sc.offset_y,
                        sc.scale,
                        sc.scale_center,
                        &sc.tiles,
                        glyph_dimensions,
                    );
                    sc.needs_resize_internal = false;
                }
            }
            ConsoleBacking::Fancy { backing } => {
                let mut fc = bi.consoles[i]
                    .console
                    .as_any_mut()
                    .downcast_mut::<FancyConsole>()
                    .unwrap();
                if fc.is_dirty {
                    fc.tiles.sort_by(|a,b| a.z_order.cmp(&b.z_order));
                    backing.rebuild_vertices(
                        fc.height,
                        fc.width,
                        fc.offset_x,
                        fc.offset_y,
                        fc.scale,
                        fc.scale_center,
                        &fc.tiles,
                        glyph_dimensions,
                    );
                    fc.needs_resize_internal = false;
                }
            }
        }
    }
}

fn render_consoles() -> Result<()> {
    let bi = BACKEND_INTERNAL.lock();
    let mut consoles = CONSOLE_BACKING.lock();
    for (i, c) in consoles.iter_mut().enumerate() {
        let cons = &bi.consoles[i];
        let font = &bi.fonts[cons.font_index];
        let shader = &bi.shaders[cons.shader_index];
        match c {
            ConsoleBacking::Simple { backing } => {
                let sc = bi.consoles[i]
                    .console
                    .as_any()
                    .downcast_ref::<SimpleConsole>()
                    .unwrap();
                backing.gl_draw(font, shader, sc.height, sc.width)?;
            }
            ConsoleBacking::Sparse { backing } => {
                let sc = bi.consoles[i]
                    .console
                    .as_any()
                    .downcast_ref::<SparseConsole>()
                    .unwrap();
                backing.gl_draw(font, shader, &sc.tiles)?;
            }
            ConsoleBacking::Fancy { backing } => {
                let fc = bi.consoles[i]
                    .console
                    .as_any()
                    .downcast_ref::<FancyConsole>()
                    .unwrap();
                backing.gl_draw(font, shader, &fc.tiles)?;
            }
        }
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
    rebuild_consoles();

    // Bind to the backing buffer
    if bterm.post_scanlines {
        let be = BACKEND.lock();
        be.backing_buffer
            .as_ref()
            .unwrap()
            .bind(be.gl.as_ref().unwrap());
    }

    // Clear the screen
    unsafe {
        let be = BACKEND.lock();
        be.gl.as_ref().unwrap().clear_color(0.0, 0.0, 0.0, 1.0);
        be.gl.as_ref().unwrap().clear(glow::COLOR_BUFFER_BIT);
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

    if bterm.post_scanlines {
        // Now we return to the primary screen
        let be = BACKEND.lock();
        be.backing_buffer
            .as_ref()
            .unwrap()
            .default(be.gl.as_ref().unwrap());
        unsafe {
            let bi = BACKEND_INTERNAL.lock();
            if bterm.post_scanlines {
                bi.shaders[3].useProgram(be.gl.as_ref().unwrap());
                bi.shaders[3].setVec3(
                    be.gl.as_ref().unwrap(),
                    "screenSize",
                    bterm.width_pixels as f32,
                    bterm.height_pixels as f32,
                    0.0,
                );
                bi.shaders[3].setBool(be.gl.as_ref().unwrap(), "screenBurn", bterm.post_screenburn);
            } else {
                bi.shaders[2].useProgram(be.gl.as_ref().unwrap());
            }
            be.gl
                .as_ref()
                .unwrap()
                .bind_vertex_array(Some(be.quad_vao.unwrap()));
            be.gl.as_ref().unwrap().bind_texture(
                glow::TEXTURE_2D,
                Some(be.backing_buffer.as_ref().unwrap().texture),
            );
            be.gl.as_ref().unwrap().draw_arrays(glow::TRIANGLES, 0, 6);
        }
    }
}
