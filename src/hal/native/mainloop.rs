use super::super::super::{Console, GameState, Rltk};
use glow::HasContext;
use glutin::{
    event::Event, event::WindowEvent, event_loop::ControlFlow
};
use std::time::Instant;

const TICK_TYPE: ControlFlow = ControlFlow::Poll;

pub fn main_loop<GS: GameState>(mut rltk: Rltk, mut gamestate: GS) {
    unsafe {
        rltk.backend
            .gl
            .viewport(0, 0, rltk.width_pixels as i32, rltk.height_pixels as i32);
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
                        rltk.backend.gl.viewport(
                            0,
                            0,
                            physical.width as i32,
                            physical.height as i32,
                        );
                    }
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
                    if modifiers.shift {
                        rltk.shift = true;
                    }
                    if modifiers.alt {
                        rltk.alt = true;
                    }
                    if modifiers.ctrl {
                        rltk.control = true;
                    }
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
        cons.console.rebuild_if_dirty(&rltk.backend.gl);
    }

    // Clear the screen
    unsafe {
        rltk.backend.gl.clear_color(0.2, 0.3, 0.3, 1.0);
        rltk.backend.gl.clear(glow::COLOR_BUFFER_BIT);
    }

    // Setup render pass
    let gl = &rltk.backend.gl;

    unsafe {
        rltk.shaders[0].useProgram(gl);

        gl.active_texture(glow::TEXTURE0);
        rltk.fonts[0].bind_texture(gl);
        rltk.shaders[0].setInt(gl, "texture1", 0);
        rltk.shaders[0].setVec3(gl, "font", 8.0, 8.0, 0.0);

        gl.bind_vertex_array(Some(rltk.backend.platform.quad_vao));
    }

    // Tell each console to draw itself
    for cons in &mut rltk.consoles {
        let font = &rltk.fonts[cons.font_index];
        let shader = &rltk.shaders[0];
        unsafe {
            gl.active_texture(glow::TEXTURE0);
            font.bind_texture(gl);
            shader.setBool(&rltk.backend.gl, "showScanLines", rltk.post_scanlines);
            shader.setBool(&rltk.backend.gl, "screenBurn", rltk.post_screenburn);
            shader.setVec3(&rltk.backend.gl, "screenSize", rltk.width_pixels as f32, rltk.height_pixels as f32, 0.0);
        }
        cons.console.gl_draw(font, shader, &rltk.backend.gl);
    }
}
