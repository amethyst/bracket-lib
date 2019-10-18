use super::super::super::{GameState, Rltk};
use super::super::*;
use super::events::*;
use super::VirtualKeyCode;
use glow::HasContext;

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
            rltk.web_button = GLOBAL_BUTTON.clone();
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
            GLOBAL_BUTTON = None;
        }
    });
}

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
        rltk.backend
            .gl
            .viewport(0, 0, rltk.width_pixels as i32, rltk.height_pixels as i32);
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
        rltk.backend
            .platform
            .backing_buffer
            .default(&rltk.backend.gl);
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
            rltk.backend
                .gl
                .bind_vertex_array(Some(rltk.backend.platform.quad_vao));
            rltk.backend.gl.bind_texture(
                glow::TEXTURE_2D,
                Some(rltk.backend.platform.backing_buffer.texture),
            );
            rltk.backend.gl.draw_arrays(glow::TRIANGLES, 0, 6);
        }
    }
}
