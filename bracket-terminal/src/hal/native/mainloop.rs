use super::BACKEND;
use crate::gl_error_wrap;
use crate::hal::*;
use crate::prelude::{BEvent, BTerm, GameState, BACKEND_INTERNAL, INPUT};
use crate::{clear_input_state, BResult};
use bracket_geometry::prelude::Point;
use glow::HasContext;
use glutin::surface::GlSurface;
use std::num::NonZeroU32;
use std::rc::Rc;
use std::time::Instant;
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::ControlFlow,
    keyboard::{KeyCode, PhysicalKey},
};

const TICK_TYPE: ControlFlow = ControlFlow::Poll;

fn largest_active_font() -> (u32, u32) {
    let bi = BACKEND_INTERNAL.lock();
    let mut max_width = 0;
    let mut max_height = 0;
    bi.consoles.iter().for_each(|c| {
        let size = bi.fonts[c.font_index].tile_size;
        if size.0 > max_width {
            max_width = size.0;
        }
        if size.1 > max_height {
            max_height = size.1;
        }
    });
    (max_width, max_height)
}

fn on_resize(
    bterm: &mut BTerm,
    physical_size: PhysicalSize<u32>,
    dpi_scale_factor: f64,
    send_event: bool,
) -> BResult<()> {
    let font_max_size = largest_active_font();
    //println!("{:#?}", physical_size);
    INPUT.lock().set_scale_factor(dpi_scale_factor);
    let mut be = BACKEND.lock();
    be.screen_scaler.change_physical_size_smooth(
        physical_size.width,
        physical_size.height,
        dpi_scale_factor as f32,
        font_max_size,
    );
    let (l, r, t, b) = be.screen_scaler.get_backing_buffer_output_coordinates();
    be.quad_vao = Some(setup_quad_gutter(be.gl.as_ref().unwrap(), l, r, t, b));
    if send_event {
        bterm.resize_pixels(
            physical_size.width as u32,
            physical_size.height as u32,
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
    /*let new_fb = Framebuffer::build_fbo(
        gl,
        physical_size.width as i32,
        physical_size.height as i32
    )?;
    be.backing_buffer = Some(new_fb);*/
    bterm.on_event(BEvent::Resized {
        new_size: Point::new(
            be.screen_scaler.available_width,
            be.screen_scaler.available_height,
        ),
        dpi_scale_factor: dpi_scale_factor as f32,
    });

    let mut bit = BACKEND_INTERNAL.lock();
    if be.resize_scaling && send_event {
        // Framebuffer must be rebuilt because render target changed
        let new_fb = Framebuffer::build_fbo(
            gl,
            be.screen_scaler.available_width as i32,
            be.screen_scaler.available_height as i32,
        )?;
        be.backing_buffer = Some(new_fb);
        be.screen_scaler.logical_size.0 = be.screen_scaler.available_width;
        be.screen_scaler.logical_size.1 = be.screen_scaler.available_height;

        let num_consoles = bit.consoles.len();
        for i in 0..num_consoles {
            let font_size = bit.fonts[bit.consoles[i].font_index].tile_size;
            let chr_w = be.screen_scaler.available_width / font_size.0;
            let chr_h = be.screen_scaler.available_height / font_size.1;
            bit.consoles[i].console.set_char_size(chr_w, chr_h);
        }
    }

    Ok(())
}

struct ResizeEvent {
    physical_size: PhysicalSize<u32>,
    dpi_scale_factor: f64,
    send_event: bool,
}

pub fn main_loop<GS: GameState>(mut bterm: BTerm, mut gamestate: GS) -> BResult<()> {
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
            s.backing = Some(Rc::new(Box::new(f)));
        }
    }

    // We're doing a little dance here to get around lifetime/borrow checking.
    // Removing the context data from BTerm in an atomic swap, so it isn't borrowed after move.
    let wrap = { std::mem::replace(&mut BACKEND.lock().context_wrapper, None) };
    let unwrap = wrap.unwrap();

    let el = unwrap.el;
    let mut window = unwrap.window;
    let mut gl_context = unwrap.gl_context;
    let mut gl_surface = unwrap.gl_surface;

    on_resize(
        &mut bterm,
        window.inner_size(),
        window.scale_factor(),
        true,
    )?; // Additional resize to handle some X11 cases

    let mut queued_resize_event: Option<ResizeEvent> = None;
    #[cfg(feature = "low_cpu")]
    let spin_sleeper = spin_sleep::SpinSleeper::default();
    let my_window_id = window.id();

    el.run(move |event, event_loop| {
        event_loop.set_control_flow(TICK_TYPE);
        let wait_time = BACKEND.lock().frame_sleep_time.unwrap_or(33); // Hoisted to reduce locks

        if bterm.quitting {
            event_loop.exit();
        }

        match event {
            Event::AboutToWait => {
                window.set_cursor_visible(bterm.mouse_visible);
                window.request_redraw();
            }
            Event::WindowEvent { window_id, event } => {
                if window_id != my_window_id {
                    return;
                }

                match event {
                    WindowEvent::RedrawRequested => {
                        let frame_timer = Instant::now();
                        if window.inner_size().width == 0 {
                            return;
                        }

                        let execute_ms = now.elapsed().as_millis() as u64 - prev_ms as u64;
                        if execute_ms >= wait_time {
                            if let Some(resize) = &queued_resize_event {
                                resize_surface(&mut gl_surface, &gl_context, resize.physical_size);
                                on_resize(
                                    &mut bterm,
                                    resize.physical_size,
                                    resize.dpi_scale_factor,
                                    resize.send_event,
                                )
                                .unwrap();
                            }
                            queued_resize_event = None;

                            tock(
                                &mut bterm,
                                window.scale_factor() as f32,
                                &mut gamestate,
                                &mut frames,
                                &mut prev_seconds,
                                &mut prev_ms,
                                &now,
                            );
                            gl_surface.swap_buffers(&gl_context).unwrap();
                            clear_input_state(&mut bterm);
                        }

                        let time_since_last_frame = frame_timer.elapsed().as_millis() as u64;
                        if time_since_last_frame < wait_time {
                            let delay = u64::min(33, wait_time - time_since_last_frame);
                            #[cfg(not(feature = "low_cpu"))]
                            {
                                std::thread::sleep(std::time::Duration::from_millis(delay));
                            }
                            #[cfg(feature = "low_cpu")]
                            spin_sleeper.sleep(std::time::Duration::from_millis(delay));
                        }
                    }
                    WindowEvent::Moved(physical_position) => {
                        bterm.on_event(BEvent::Moved {
                            new_position: Point::new(physical_position.x, physical_position.y),
                        });

                        let scale_factor = window.scale_factor();
                        let physical_size = window.inner_size();
                        resize_surface(&mut gl_surface, &gl_context, physical_size);
                        queued_resize_event = Some(ResizeEvent {
                            physical_size,
                            dpi_scale_factor: scale_factor,
                            send_event: true,
                        });
                    }
                    WindowEvent::Resized(physical_size) => {
                        let scale_factor = window.scale_factor();
                        resize_surface(&mut gl_surface, &gl_context, physical_size);
                        queued_resize_event = Some(ResizeEvent {
                            physical_size,
                            dpi_scale_factor: scale_factor,
                            send_event: true,
                        });
                    }
                    WindowEvent::CloseRequested => {
                        if !INPUT.lock().use_events {
                            event_loop.exit();
                        } else {
                            bterm.on_event(BEvent::CloseRequested);
                        }
                    }
                    WindowEvent::Focused(focused) => {
                        bterm.on_event(BEvent::Focused { focused });
                    }
                    WindowEvent::CursorMoved { position: pos, .. } => {
                        bterm.on_mouse_position(pos.x, pos.y);
                    }
                    WindowEvent::CursorEntered { .. } => bterm.on_event(BEvent::CursorEntered),
                    WindowEvent::CursorLeft { .. } => bterm.on_event(BEvent::CursorLeft),
                    WindowEvent::MouseInput { button, state, .. } => {
                        let button = match button {
                            MouseButton::Left => 0,
                            MouseButton::Right => 1,
                            MouseButton::Middle => 2,
                            MouseButton::Back => 3,
                            MouseButton::Forward => 4,
                            MouseButton::Other(num) => 5 + num as usize,
                        };
                        bterm.on_mouse_button(button, state == ElementState::Pressed);
                    }
                    WindowEvent::ScaleFactorChanged {
                        scale_factor,
                        mut inner_size_writer,
                        ..
                    } => {
                        let physical_size = window.inner_size();
                        let _ = inner_size_writer.request_inner_size(physical_size);
                        resize_surface(&mut gl_surface, &gl_context, physical_size);
                        on_resize(&mut bterm, physical_size, scale_factor, false).unwrap();
                        bterm.on_event(BEvent::ScaleFactorChanged {
                            new_size: Point::new(physical_size.width, physical_size.height),
                            dpi_scale_factor: scale_factor as f32,
                        })
                    }
                    WindowEvent::KeyboardInput { event, .. } => {
                        if let Some(key) = physical_key_to_virtual_keycode(&event.physical_key) {
                            bterm.on_key(key, 0, event.state == ElementState::Pressed);
                        }
                        if event.state == ElementState::Pressed {
                            if let Some(text) = event.text.as_ref() {
                                for ch in text.chars() {
                                    bterm.on_event(BEvent::Character { c: ch });
                                }
                            }
                        }
                    }
                    WindowEvent::ModifiersChanged(modifiers) => {
                        bterm.shift = modifiers.state().shift_key();
                        bterm.alt = modifiers.state().alt_key();
                        bterm.control = modifiers.state().control_key();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    })?;

    Ok(())
}

fn resize_surface(
    surface: &mut glutin::surface::Surface<glutin::surface::WindowSurface>,
    context: &glutin::context::PossiblyCurrentContext,
    size: PhysicalSize<u32>,
) {
    let width = NonZeroU32::new(size.width.max(1)).unwrap();
    let height = NonZeroU32::new(size.height.max(1)).unwrap();
    let _ = surface.resize(context, width, height);
}

fn physical_key_to_virtual_keycode(key: &PhysicalKey) -> Option<VirtualKeyCode> {
    match key {
        PhysicalKey::Code(code) => keycode_to_virtual_keycode(code),
        PhysicalKey::Unidentified(_) => None,
    }
}

fn keycode_to_virtual_keycode(code: &KeyCode) -> Option<VirtualKeyCode> {
    use VirtualKeyCode::*;

    match code {
        KeyCode::Backquote => Some(Grave),
        KeyCode::Backslash => Some(Backslash),
        KeyCode::BracketLeft => Some(LBracket),
        KeyCode::BracketRight => Some(RBracket),
        KeyCode::Comma => Some(Comma),
        KeyCode::Digit0 => Some(Key0),
        KeyCode::Digit1 => Some(Key1),
        KeyCode::Digit2 => Some(Key2),
        KeyCode::Digit3 => Some(Key3),
        KeyCode::Digit4 => Some(Key4),
        KeyCode::Digit5 => Some(Key5),
        KeyCode::Digit6 => Some(Key6),
        KeyCode::Digit7 => Some(Key7),
        KeyCode::Digit8 => Some(Key8),
        KeyCode::Digit9 => Some(Key9),
        KeyCode::Equal => Some(Equals),
        KeyCode::IntlBackslash => Some(Backslash),
        KeyCode::IntlRo => Some(Backslash),
        KeyCode::IntlYen => Some(Backslash),
        KeyCode::KeyA => Some(A),
        KeyCode::KeyB => Some(B),
        KeyCode::KeyC => Some(C),
        KeyCode::KeyD => Some(D),
        KeyCode::KeyE => Some(E),
        KeyCode::KeyF => Some(F),
        KeyCode::KeyG => Some(G),
        KeyCode::KeyH => Some(H),
        KeyCode::KeyI => Some(I),
        KeyCode::KeyJ => Some(J),
        KeyCode::KeyK => Some(K),
        KeyCode::KeyL => Some(L),
        KeyCode::KeyM => Some(M),
        KeyCode::KeyN => Some(N),
        KeyCode::KeyO => Some(O),
        KeyCode::KeyP => Some(P),
        KeyCode::KeyQ => Some(Q),
        KeyCode::KeyR => Some(R),
        KeyCode::KeyS => Some(S),
        KeyCode::KeyT => Some(T),
        KeyCode::KeyU => Some(U),
        KeyCode::KeyV => Some(V),
        KeyCode::KeyW => Some(W),
        KeyCode::KeyX => Some(X),
        KeyCode::KeyY => Some(Y),
        KeyCode::KeyZ => Some(Z),
        KeyCode::Minus => Some(Minus),
        KeyCode::Period => Some(Period),
        KeyCode::Quote => Some(Apostrophe),
        KeyCode::Semicolon => Some(Semicolon),
        KeyCode::Slash => Some(Slash),
        KeyCode::AltLeft => Some(LAlt),
        KeyCode::AltRight => Some(RAlt),
        KeyCode::Backspace => Some(Back),
        KeyCode::CapsLock => Some(Capital),
        KeyCode::ContextMenu => Some(Apps),
        KeyCode::ControlLeft => Some(LControl),
        KeyCode::ControlRight => Some(RControl),
        KeyCode::Enter => Some(Return),
        KeyCode::SuperLeft => Some(LWin),
        KeyCode::SuperRight => Some(RWin),
        KeyCode::ShiftLeft => Some(LShift),
        KeyCode::ShiftRight => Some(RShift),
        KeyCode::Space => Some(Space),
        KeyCode::Tab => Some(Tab),
        KeyCode::ArrowDown => Some(Down),
        KeyCode::ArrowLeft => Some(Left),
        KeyCode::ArrowRight => Some(Right),
        KeyCode::ArrowUp => Some(Up),
        KeyCode::Escape => Some(Escape),
        KeyCode::End => Some(End),
        KeyCode::Home => Some(Home),
        KeyCode::Insert => Some(Insert),
        KeyCode::Delete => Some(Delete),
        KeyCode::PageDown => Some(PageDown),
        KeyCode::PageUp => Some(PageUp),
        KeyCode::PrintScreen => Some(Snapshot),
        KeyCode::ScrollLock => Some(Scroll),
        KeyCode::Pause => Some(Pause),
        KeyCode::F1 => Some(F1),
        KeyCode::F2 => Some(F2),
        KeyCode::F3 => Some(F3),
        KeyCode::F4 => Some(F4),
        KeyCode::F5 => Some(F5),
        KeyCode::F6 => Some(F6),
        KeyCode::F7 => Some(F7),
        KeyCode::F8 => Some(F8),
        KeyCode::F9 => Some(F9),
        KeyCode::F10 => Some(F10),
        KeyCode::F11 => Some(F11),
        KeyCode::F12 => Some(F12),
        KeyCode::F13 => Some(F13),
        KeyCode::F14 => Some(F14),
        KeyCode::F15 => Some(F15),
        KeyCode::F16 => Some(F16),
        KeyCode::F17 => Some(F17),
        KeyCode::F18 => Some(F18),
        KeyCode::F19 => Some(F19),
        KeyCode::F20 => Some(F20),
        KeyCode::F21 => Some(F21),
        KeyCode::F22 => Some(F22),
        KeyCode::F23 => Some(F23),
        KeyCode::F24 => Some(F24),
        KeyCode::Numpad0 => Some(Numpad0),
        KeyCode::Numpad1 => Some(Numpad1),
        KeyCode::Numpad2 => Some(Numpad2),
        KeyCode::Numpad3 => Some(Numpad3),
        KeyCode::Numpad4 => Some(Numpad4),
        KeyCode::Numpad5 => Some(Numpad5),
        KeyCode::Numpad6 => Some(Numpad6),
        KeyCode::Numpad7 => Some(Numpad7),
        KeyCode::Numpad8 => Some(Numpad8),
        KeyCode::Numpad9 => Some(Numpad9),
        KeyCode::NumLock => Some(Numlock),
        KeyCode::NumpadAdd => Some(Add),
        KeyCode::NumpadBackspace => Some(Back),
        KeyCode::NumpadComma => Some(NumpadComma),
        KeyCode::NumpadDecimal => Some(Decimal),
        KeyCode::NumpadDivide => Some(Divide),
        KeyCode::NumpadEnter => Some(NumpadEnter),
        KeyCode::NumpadEqual => Some(NumpadEquals),
        KeyCode::NumpadMultiply => Some(Multiply),
        KeyCode::NumpadSubtract => Some(Subtract),
        _ => None,
    }
}

/// Internal handling of the main loop.
fn tock<GS: GameState>(
    bterm: &mut BTerm,
    scale_factor: f32,
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
    {
        let be = BACKEND.lock();
        be.backing_buffer
            .as_ref()
            .unwrap()
            .bind(be.gl.as_ref().unwrap());
        unsafe {
            be.gl.as_ref().unwrap().viewport(
                0,
                0,
                be.screen_scaler.logical_size.0 as i32,
                be.screen_scaler.logical_size.1 as i32,
            );
        }
    }

    // Clear the backing buffer
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

    {
        // Now we return to the primary screen
        let be = BACKEND.lock();
        be.backing_buffer
            .as_ref()
            .unwrap()
            .default(be.gl.as_ref().unwrap());
        unsafe {
            // And clear it, resetting the viewport
            be.gl.as_ref().unwrap().viewport(
                0,
                0,
                be.screen_scaler.physical_size.0 as i32,
                be.screen_scaler.physical_size.1 as i32,
            );
            be.gl.as_ref().unwrap().clear_color(0.0, 0.0, 0.0, 1.0);
            be.gl.as_ref().unwrap().clear(glow::COLOR_BUFFER_BIT);

            let bi = BACKEND_INTERNAL.lock();
            if bterm.post_scanlines {
                bi.shaders[3].useProgram(be.gl.as_ref().unwrap());
                bi.shaders[3].setVec3(
                    be.gl.as_ref().unwrap(),
                    "screenSize",
                    scale_factor * bterm.width_pixels as f32,
                    scale_factor * bterm.height_pixels as f32,
                    0.0,
                );
                bi.shaders[3].setBool(be.gl.as_ref().unwrap(), "screenBurn", bterm.post_screenburn);
                bi.shaders[3].setVec3(
                    be.gl.as_ref().unwrap(),
                    "screenBurnColor",
                    bterm.screen_burn_color.r,
                    bterm.screen_burn_color.g,
                    bterm.screen_burn_color.b,
                );
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
            let w = (bterm.width_pixels as f32) as u32;
            let h = (bterm.height_pixels as f32) as u32;
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
                    glow::PixelPackData::Slice(pixels),
                );
            }

            image::save_buffer(
                filename,
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
