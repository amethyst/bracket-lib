use super::super::super::{Console, GameState, Rltk};
use super::super::*;
use glow::HasContext;
use glutin::{event::DeviceEvent, event::Event, event::WindowEvent, event_loop::ControlFlow};
use std::time::Instant;

const TICK_TYPE: ControlFlow = ControlFlow::Poll;

fn on_resize(rltk: &mut Rltk, physical_size: glutin::dpi::PhysicalSize<u32>) {
    rltk.resize_pixels(physical_size.width as u32, physical_size.height as u32);
    unsafe {
        rltk.backend.platform.gl.viewport(
            0,
            0,
            physical_size.width as i32,
            physical_size.height as i32,
        );
    }
    rltk.backend.platform.backing_buffer = Framebuffer::build_fbo(
        &rltk.backend.platform.gl,
        physical_size.width as i32,
        physical_size.height as i32,
    );
}

pub fn main_loop<GS: GameState>(mut rltk: Rltk, mut gamestate: GS) {
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

    on_resize(&mut rltk, wc.window().inner_size()); // Additional resize to handle some X11 cases

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
            Event::MainEventsCleared => {
                wc.window().request_redraw();
            }
            Event::RedrawRequested { .. } => {
                tock(
                    &mut rltk,
                    &mut gamestate,
                    &mut frames,
                    &mut prev_seconds,
                    &mut prev_ms,
                    &now,
                );
                wc.swap_buffers().unwrap();
                crate::hal::fps_sleep(rltk.backend.platform.frame_sleep_time, &now, prev_ms);
            }
            Event::DeviceEvent {
                event: DeviceEvent::ModifiersChanged(modifiers),
                ..
            } => {
                rltk.shift = modifiers.shift();
                rltk.alt = modifiers.alt();
                rltk.control = modifiers.ctrl();
            }
            Event::LoopDestroyed => (),
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    on_resize(&mut rltk, *physical_size);
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                WindowEvent::CursorMoved { position: pos, .. } => {
                    rltk.mouse_pos = (pos.x as i32, pos.y as i32);
                }

                WindowEvent::MouseInput { .. } => {
                    rltk.left_click = true;
                }

                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    on_resize(&mut rltk, **new_inner_size);
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
        cons.console.rebuild_if_dirty(&rltk.backend);
    }

    // Bind to the backing buffer
    if rltk.post_scanlines {
        rltk.backend
            .platform
            .backing_buffer
            .bind(&rltk.backend.platform.gl);
    }

    // Clear the screen
    unsafe {
        rltk.backend.platform.gl.clear_color(0.0, 0.0, 0.0, 1.0);
        rltk.backend.platform.gl.clear(glow::COLOR_BUFFER_BIT);
    }

    // Tell each console to draw itself
    for cons in &mut rltk.consoles {
        let font = &rltk.fonts[cons.font_index];
        let shader = &rltk.shaders[cons.shader_index];
        cons.console.gl_draw(font, shader, &rltk.backend);
    }

    if rltk.post_scanlines {
        // Now we return to the primary screen
        rltk.backend
            .platform
            .backing_buffer
            .default(&rltk.backend.platform.gl);
        unsafe {
            if rltk.post_scanlines {
                rltk.shaders[3].useProgram(&rltk.backend.platform.gl);
                rltk.shaders[3].setVec3(
                    &rltk.backend.platform.gl,
                    "screenSize",
                    rltk.width_pixels as f32,
                    rltk.height_pixels as f32,
                    0.0,
                );
                rltk.shaders[3].setBool(
                    &rltk.backend.platform.gl,
                    "screenBurn",
                    rltk.post_screenburn,
                );
            } else {
                rltk.shaders[2].useProgram(&rltk.backend.platform.gl);
            }
            rltk.backend
                .platform
                .gl
                .bind_vertex_array(Some(rltk.backend.platform.quad_vao));
            rltk.backend.platform.gl.bind_texture(
                glow::TEXTURE_2D,
                Some(rltk.backend.platform.backing_buffer.texture),
            );
            rltk.backend.platform.gl.draw_arrays(glow::TRIANGLES, 0, 6);
        }
    }
}
