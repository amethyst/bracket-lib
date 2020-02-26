use super::events::*;
use crate::prelude::{BTerm, GameState};
use crate::Result;
use glow::HasContext;

pub fn main_loop<GS: GameState>(mut bterm: BTerm, mut gamestate: GS) -> Result<()> {
    use glow::HasRenderLoop;

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
        bterm.left_click = false;
        bterm.key = None;
        unsafe {
            GLOBAL_KEY = None;
            GLOBAL_MODIFIERS = (false, false, false);
            GLOBAL_LEFT_CLICK = false;
            GLOBAL_BUTTON = None;
        }
    });
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
    for cons in &mut bterm.consoles {
        cons.console.rebuild_if_dirty(&bterm.backend);
    }

    // Clear the screen
    unsafe {
        bterm.backend.platform.gl.viewport(
            0,
            0,
            bterm.width_pixels as i32,
            bterm.height_pixels as i32,
        );
        bterm.backend.platform.gl.clear_color(0.2, 0.3, 0.3, 1.0);
        bterm.backend.platform.gl.clear(glow::COLOR_BUFFER_BIT);
    }

    // Setup render pass
    let gl = &bterm.backend.platform.gl;

    unsafe {
        bterm.shaders[0].useProgram(gl);

        gl.active_texture(glow::TEXTURE0);
        bterm.fonts[0].bind_texture(&bterm.backend);
        bterm.shaders[0].setInt(gl, "texture1", 0);
        bterm.shaders[0].setVec3(gl, "font", 8.0, 8.0, 0.0);

        gl.bind_vertex_array(Some(bterm.backend.platform.quad_vao));
    }

    // Tell each console to draw itself
    for cons in &mut bterm.consoles {
        let font = &bterm.fonts[cons.font_index];
        let shader = &bterm.shaders[0];
        unsafe {
            gl.active_texture(glow::TEXTURE0);
            font.bind_texture(&bterm.backend);
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
        cons.console.gl_draw(font, shader, &bterm.backend);
    }
}
