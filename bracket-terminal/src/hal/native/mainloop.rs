use super::BACKEND;
use crate::gl_error_wrap;
use crate::hal::*;
use crate::prelude::{BEvent, BTerm, GameState, BACKEND_INTERNAL, INPUT};
use crate::{clear_input_state, Result};
use bracket_geometry::prelude::Point;
use glow::HasContext;
use glutin::{event::Event, event::MouseButton, event::WindowEvent, event_loop::ControlFlow};
use std::time::Instant;

const TICK_TYPE: ControlFlow = ControlFlow::Poll;

fn on_resize(
    bterm: &mut BTerm,
    physical_size: glutin::dpi::PhysicalSize<u32>,
    dpi_scale_factor: f64,
    send_event: bool,
) -> Result<()> {
    INPUT.lock().set_scale_factor(dpi_scale_factor);
    let mut be = BACKEND.lock();
    if send_event {
        bterm.resize_pixels(
            (physical_size.width as f64 / dpi_scale_factor) as u32,
            (physical_size.height as f64 / dpi_scale_factor) as u32,
            be.resize_scaling,
        );
    }
    let gl = be.gl.as_ref().unwrap();
    unsafe {
        gl_error_wrap!(
            gl,
            gl.viewport(
                0,
                0,
                physical_size.width as i32,
                physical_size.height as i32,
            )
        );
    }
    let new_fb =
        Framebuffer::build_fbo(gl, physical_size.width as i32, physical_size.height as i32)?;
    be.backing_buffer = Some(new_fb);
    bterm.on_event(BEvent::Resized {
        new_size: Point::new(physical_size.width, physical_size.height),
        dpi_scale_factor: dpi_scale_factor as f32,
    });

    let mut bit = BACKEND_INTERNAL.lock();
    if be.resize_scaling && send_event {
        let num_consoles = bit.consoles.len();
        for i in 0..num_consoles {
            let font_size = bit.fonts[bit.consoles[i].font_index].tile_size;
            let chr_w = (physical_size.width as f64 / dpi_scale_factor) as u32 / font_size.0;
            let chr_h = (physical_size.height as f64 / dpi_scale_factor) as u32 / font_size.1;
            bit.consoles[i].console.set_char_size(chr_w, chr_h);
        }
    }

    Ok(())
}

pub fn main_loop<GS: GameState>(mut bterm: BTerm, mut gamestate: GS) -> Result<()> {
    let now = Instant::now();
    let mut prev_seconds = now.elapsed().as_secs();
    let mut prev_ms = now.elapsed().as_millis();
    let mut frames = 0;

    {
        let be = BACKEND.lock();
        let gl = be.gl.as_ref().unwrap();
        let mut bit = BACKEND_INTERNAL.lock();
        for f in bit.fonts.iter_mut() {
            f.setup_gl_texture(gl)?;
        }

        for s in bit.sprite_sheets.iter_mut() {
            let mut f = Font::new(&s.filename.to_string(), 1, 1, (1, 1));
            f.setup_gl_texture(gl)?;
            s.backing = Some(f);
        }
    }

    // We're doing a little dance here to get around lifetime/borrow checking.
    // Removing the context data from BTerm in an atomic swap, so it isn't borrowed after move.
    let wrap = { std::mem::replace(&mut BACKEND.lock().context_wrapper, None) };
    let unwrap = wrap.unwrap();

    let el = unwrap.el;
    let wc = unwrap.wc;

    on_resize(
        &mut bterm,
        wc.window().inner_size(),
        wc.window().scale_factor(),
        false,
    )?; // Additional resize to handle some X11 cases

    el.run(move |event, _, control_flow| {
        *control_flow = TICK_TYPE;

        if bterm.quitting {
            *control_flow = ControlFlow::Exit;
        }

        /*let rr = BACKEND.lock().resize_request;
        if let Some(rr) = rr {
            wc.window().set_inner_size(glutin::dpi::PhysicalSize::new(rr.0, rr.1));
        }*/

        match event {
            Event::NewEvents(_) => {
                clear_input_state(&mut bterm);
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
                crate::hal::fps_sleep(BACKEND.lock().frame_sleep_time, &now, prev_ms);
            }
            Event::LoopDestroyed => (),
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Moved(physical_position) => {
                    bterm.on_event(BEvent::Moved {
                        new_position: Point::new(physical_position.x, physical_position.y),
                    });
                }
                WindowEvent::Resized(physical_size) => {
                    on_resize(&mut bterm, *physical_size, wc.window().scale_factor(), true)
                        .unwrap();
                }
                WindowEvent::CloseRequested => {
                    // If not using events, just close. Otherwise, push the event
                    if !INPUT.lock().use_events {
                        *control_flow = ControlFlow::Exit;
                    } else {
                        bterm.on_event(BEvent::CloseRequested);
                    }
                }
                WindowEvent::ReceivedCharacter(char) => {
                    bterm.on_event(BEvent::Character { c: *char });
                }
                WindowEvent::Focused(focused) => {
                    bterm.on_event(BEvent::Focused { focused: *focused });
                }
                WindowEvent::CursorMoved { position: pos, .. } => {
                    let scale_factor = wc.window().scale_factor();
                    bterm.on_mouse_position(pos.x / scale_factor, pos.y / scale_factor);
                }
                WindowEvent::CursorEntered { .. } => bterm.on_event(BEvent::CursorEntered),
                WindowEvent::CursorLeft { .. } => bterm.on_event(BEvent::CursorLeft),

                WindowEvent::MouseInput { button, state, .. } => {
                    let button = match button {
                        MouseButton::Left => 0,
                        MouseButton::Right => 1,
                        MouseButton::Middle => 2,
                        MouseButton::Other(num) => 3 + *num as usize,
                    };
                    bterm.on_mouse_button(button, *state == glutin::event::ElementState::Pressed);
                }

                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    let sf = wc.window().scale_factor();
                    on_resize(&mut bterm, **new_inner_size, sf, false).unwrap();
                    bterm.on_event(BEvent::ScaleFactorChanged {
                        new_size: Point::new(new_inner_size.width, new_inner_size.height),
                        dpi_scale_factor: sf as f32,
                    })
                }

                WindowEvent::KeyboardInput {
                    input:
                        glutin::event::KeyboardInput {
                            virtual_keycode: Some(virtual_keycode),
                            state,
                            scancode,
                            ..
                        },
                    ..
                } => bterm.on_key(
                    *virtual_keycode,
                    *scancode,
                    *state == glutin::event::ElementState::Pressed,
                ),

                WindowEvent::ModifiersChanged(modifiers) => {
                    bterm.shift = modifiers.shift();
                    bterm.alt = modifiers.alt();
                    bterm.control = modifiers.ctrl();
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

    // Run the main loop
    gamestate.tick(bterm);

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
                bi.shaders[3].setVec3(be.gl.as_ref().unwrap(), "screenBurnColor", bterm.screen_burn_color.r, bterm.screen_burn_color.g, bterm.screen_burn_color.b);
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

    // Screenshot handler
    {
        let mut be = BACKEND.lock();
        if let Some(filename) = &be.request_screenshot {
            let w = bterm.width_pixels;
            let h = bterm.height_pixels;
            let gl = be.gl.as_ref().unwrap();

            let mut img = image::DynamicImage::new_rgba8(w, h);
            let pixels = img.as_mut_rgba8().unwrap();

            unsafe {
                gl.pixel_store_i32(glow::PACK_ALIGNMENT, 1);
                gl.read_pixels(
                    0,
                    0,
                    w as i32,
                    h as i32,
                    glow::RGBA,
                    glow::UNSIGNED_BYTE,
                    pixels,
                );
            }

            image::save_buffer(
                &filename,
                &image::imageops::flip_vertical(&img),
                w,
                h,
                image::ColorType::Rgba8,
            )
            .expect("Failed to save buffer to the specified path");
        }
        be.request_screenshot = None;
    }
}
