use crate::Result;
use crate::prelude::{Console, GameState, BTerm};
use crate::hal::*;
use super::BACKEND;
use glow::HasContext;
use glutin::{event::DeviceEvent, event::Event, event::WindowEvent, event_loop::ControlFlow};
use std::time::Instant;

const TICK_TYPE: ControlFlow = ControlFlow::Poll;

fn on_resize(bterm: &mut BTerm, physical_size: glutin::dpi::PhysicalSize<u32>) -> Result<()> {
    bterm.resize_pixels(physical_size.width as u32, physical_size.height as u32);
    let mut be = BACKEND.lock().unwrap();
    let gl = be.gl.as_ref().unwrap();
    unsafe {
        gl.viewport(
            0,
            0,
            physical_size.width as i32,
            physical_size.height as i32,
        );
    }
    let new_fb = Framebuffer::build_fbo(
        gl,
        physical_size.width as i32,
        physical_size.height as i32,
    )?;
    be.backing_buffer = Some(new_fb);
    Ok(())
}

pub fn main_loop<GS: GameState>(mut bterm: BTerm, mut gamestate: GS) -> Result<()> {
    let now = Instant::now();
    let mut prev_seconds = now.elapsed().as_secs();
    let mut prev_ms = now.elapsed().as_millis();
    let mut frames = 0;

    // We're doing a little dance here to get around lifetime/borrow checking.
    // Removing the context data from BTerm in an atomic swap, so it isn't borrowed after move.
    let wrap = std::mem::replace(&mut BACKEND.lock().unwrap().context_wrapper, None);
    let unwrap = wrap.unwrap();

    let el = unwrap.el;
    let wc = unwrap.wc;

    on_resize(&mut bterm, wc.window().inner_size())?; // Additional resize to handle some X11 cases

    el.run(move |event, _, control_flow| {
        *control_flow = TICK_TYPE;

        if bterm.quitting {
            *control_flow = ControlFlow::Exit;
        }

        match event {
            Event::NewEvents(_) => {
                bterm.left_click = false;
                bterm.key = None;
            }
            Event::MainEventsCleared => {
                wc.window().request_redraw();
            }
            Event::RedrawRequested { .. } => {
                tock(
                    &mut bterm,
                    &mut gamestate,
                    &mut frames,
                    &mut prev_seconds,
                    &mut prev_ms,
                    &now,
                );
                wc.swap_buffers().unwrap();
                crate::hal::fps_sleep(BACKEND.lock().unwrap().frame_sleep_time, &now, prev_ms);
            }
            Event::DeviceEvent {
                event: DeviceEvent::ModifiersChanged(modifiers),
                ..
            } => {
                bterm.shift = modifiers.shift();
                bterm.alt = modifiers.alt();
                bterm.control = modifiers.ctrl();
            }
            Event::LoopDestroyed => (),
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    on_resize(&mut bterm, *physical_size).unwrap();
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                WindowEvent::CursorMoved { position: pos, .. } => {
                    bterm.mouse_pos = (pos.x as i32, pos.y as i32);
                }

                WindowEvent::MouseInput { .. } => {
                    bterm.left_click = true;
                }

                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    on_resize(&mut bterm, **new_inner_size).unwrap();
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
                    bterm.key = Some(*virtual_keycode);
                }

                _ => (),
            },
            _ => (),
        }
    });
}

/// Internal handling of the main loop.
fn tock<GS: GameState>(
    bterm: &mut BTerm,
    gamestate: &mut GS,
    frames: &mut i32,
    prev_seconds: &mut u64,
    prev_ms: &mut u128,
    now: &Instant,
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
        //cons.console.rebuild_if_dirty(&bterm.backend);
        // TODO: Replace me
    }

    // Bind to the backing buffer
    if bterm.post_scanlines {
        let be = BACKEND.lock().unwrap();
        be
            .backing_buffer
            .as_ref()
            .unwrap()
            .bind(be.gl.as_ref().unwrap());
    }

    // Clear the screen
    unsafe {
        let be = BACKEND.lock().unwrap();
        be.gl.as_ref().unwrap().clear_color(0.0, 0.0, 0.0, 1.0);
        be.gl.as_ref().unwrap().clear(glow::COLOR_BUFFER_BIT);
    }

    // Tell each console to draw itself
    for cons in &mut bterm.consoles {
        let font = &bterm.fonts[cons.font_index];
        let shader = &bterm.shaders[cons.shader_index];
        //cons.console.gl_draw(font, shader, &bterm.backend);
        // TODO: Replace me
    }

    if bterm.post_scanlines {
        // Now we return to the primary screen
        let be = BACKEND.lock().unwrap();
        be
            .backing_buffer
            .as_ref()
            .unwrap()
            .default(be.gl.as_ref().unwrap());
        unsafe {
            if bterm.post_scanlines {
                bterm.shaders[3].useProgram(be.gl.as_ref().unwrap());
                bterm.shaders[3].setVec3(
                    be.gl.as_ref().unwrap(),
                    "screenSize",
                    bterm.width_pixels as f32,
                    bterm.height_pixels as f32,
                    0.0,
                );
                bterm.shaders[3].setBool(
                    be.gl.as_ref().unwrap(),
                    "screenBurn",
                    bterm.post_screenburn,
                );
            } else {
                bterm.shaders[2].useProgram(be.gl.as_ref().unwrap());
            }
            be
                .gl
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
