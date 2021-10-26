use super::{CONSOLE_BACKING, ConsoleBacking, Font, SimpleConsoleBackend, SparseConsoleBackend};
use crate::{BResult, gamestate::{BTerm, GameState}, input::{clear_input_state, BEvent}, prelude::{BACKEND, BACKEND_INTERNAL, INPUT, SimpleConsole, SparseConsole}};
use bracket_geometry::prelude::Point;
use std::time::Instant;
use winit::{dpi::PhysicalSize, event::*, event_loop::ControlFlow};

const TICK_TYPE: ControlFlow = ControlFlow::Poll;

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
        let wgpu = be.wgpu.as_ref().unwrap();
        let mut bit = BACKEND_INTERNAL.lock();
        for f in bit.fonts.iter_mut() {
            f.setup_wgpu_texture(wgpu)?;
        }

        for s in bit.sprite_sheets.iter_mut() {
            let mut f = Font::new(&s.filename.to_string(), 1, 1, (1, 1));
            f.setup_wgpu_texture(wgpu)?;
            s.backing = Some(f);
        }
    }

    // We're doing a little dance here to get around lifetime/borrow checking.
    // Removing the context data from BTerm in an atomic swap, so it isn't borrowed after move.
    let wrap = { std::mem::replace(&mut BACKEND.lock().context_wrapper, None) };
    let unwrap = wrap.unwrap();

    let el = unwrap.el;
    let window = unwrap.window;

    let mut queued_resize_event: Option<ResizeEvent> = None;
    let spin_sleeper = spin_sleep::SpinSleeper::default();
    let my_window_id = window.id();

    el.run(move |event, _, control_flow| {
        let wait_time = BACKEND.lock().frame_sleep_time.unwrap_or(33); // Hoisted to reduce locks
        *control_flow = TICK_TYPE;

        if bterm.quitting {
            *control_flow = ControlFlow::Exit;
        }

        match &event {
            Event::RedrawEventsCleared => {
                let frame_timer = Instant::now();
                if window.inner_size().width == 0 || window.inner_size().height == 0 {
                    return;
                }

                let execute_ms = now.elapsed().as_millis() as u64 - prev_ms as u64;
                if execute_ms >= wait_time {
                    if queued_resize_event.is_some() {
                        if let Some(resize) = &queued_resize_event {
                            //window..resize(resize.physical_size);
                            on_resize(
                                &mut bterm,
                                resize.physical_size,
                                resize.dpi_scale_factor,
                                resize.send_event,
                            )
                            .unwrap();
                        }
                        queued_resize_event = None;
                    }

                    tock(
                        &mut bterm,
                        window.scale_factor() as f32,
                        &mut gamestate,
                        &mut frames,
                        &mut prev_seconds,
                        &mut prev_ms,
                        &now,
                    );
                    //wc.swap_buffers().unwrap();
                    // Moved from new events, which doesn't make sense
                    clear_input_state(&mut bterm);
                }

                // Wait for an appropriate amount of time
                let time_since_last_frame = frame_timer.elapsed().as_millis() as u64;
                if time_since_last_frame < wait_time {
                    let delay = u64::min(33, wait_time - time_since_last_frame);
                    spin_sleeper.sleep(std::time::Duration::from_millis(delay));
                }
            }
            Event::WindowEvent { event, window_id } => {
                // Fast return for other windows
                if *window_id != my_window_id {
                    //println!("Dropped event from other window");
                    return;
                }

                // Handle Window Events
                match event {
                    WindowEvent::Moved(physical_position) => {
                        bterm.on_event(BEvent::Moved {
                            new_position: Point::new(physical_position.x, physical_position.y),
                        });

                        let scale_factor = window.scale_factor();
                        let physical_size = window.inner_size();
                        //wc.resize(physical_size);
                        //on_resize(&mut bterm, physical_size, scale_factor, true).unwrap();
                        queued_resize_event = Some(ResizeEvent {
                            physical_size,
                            dpi_scale_factor: scale_factor,
                            send_event: true,
                        });
                    }
                    WindowEvent::Resized(_physical_size) => {
                        let scale_factor = window.scale_factor();
                        let physical_size = window.inner_size();
                        //wc.resize(physical_size);
                        //on_resize(&mut bterm, physical_size, scale_factor, true).unwrap();
                        queued_resize_event = Some(ResizeEvent {
                            physical_size,
                            dpi_scale_factor: scale_factor,
                            send_event: true,
                        });
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
                        bterm.on_mouse_position(pos.x, pos.y);
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
                        bterm.on_mouse_button(button, *state == ElementState::Pressed);
                    }

                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        let scale_factor = window.scale_factor();
                        let physical_size = window.inner_size();
                        //wc.resize(physical_size);
                        on_resize(&mut bterm, physical_size, scale_factor, false).unwrap();
                        bterm.on_event(BEvent::ScaleFactorChanged {
                            new_size: Point::new(new_inner_size.width, new_inner_size.height),
                            dpi_scale_factor: scale_factor as f32,
                        })
                    }

                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(virtual_keycode),
                                state,
                                scancode,
                                ..
                            },
                        ..
                    } => bterm.on_key(*virtual_keycode, *scancode, *state == ElementState::Pressed),

                    WindowEvent::ModifiersChanged(modifiers) => {
                        bterm.shift = modifiers.shift();
                        bterm.alt = modifiers.alt();
                        bterm.control = modifiers.ctrl();
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    });
}

fn on_resize(
    bterm: &mut BTerm,
    physical_size: PhysicalSize<u32>,
    dpi_scale_factor: f64,
    send_event: bool,
) -> BResult<()> {
    //println!("{:#?}", physical_size);
    INPUT.lock().set_scale_factor(dpi_scale_factor);
    let mut be = BACKEND.lock();
    if send_event {
        bterm.resize_pixels(
            physical_size.width as u32,
            physical_size.height as u32,
            be.resize_scaling,
        );
    }

    // WGPU resizing
    if let Some(mut wgpu) = be.wgpu.as_mut() {
        wgpu.config.width = physical_size.width;
        wgpu.config.height = physical_size.height;
        wgpu.surface.configure(&wgpu.device, &wgpu.config);
    }

    // Backing buffer resizing

    // Messaging
    bterm.on_event(BEvent::Resized {
        new_size: Point::new(physical_size.width, physical_size.height),
        dpi_scale_factor: dpi_scale_factor as f32,
    });

    // Consoles
    let mut bit = BACKEND_INTERNAL.lock();
    if be.resize_scaling && send_event {
        let num_consoles = bit.consoles.len();
        for i in 0..num_consoles {
            let font_size = bit.fonts[bit.consoles[i].font_index].tile_size;
            let chr_w = physical_size.width as u32 / font_size.0;
            let chr_h = physical_size.height as u32 / font_size.1;
            bit.consoles[i].console.set_char_size(chr_w, chr_h);
        }
    }

    Ok(())
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
    if bterm.post_scanlines {
        /*let be = BACKEND.lock();
        be.backing_buffer
            .as_ref()
            .unwrap()
            .bind(be.gl.as_ref().unwrap());
        */
    }

    // Clear the screen
    //clear_screen_pass();
    /*unsafe {
        let be = BACKEND.lock();
        be.gl.as_ref().unwrap().clear_color(0.0, 0.0, 0.0, 1.0);
        be.gl.as_ref().unwrap().clear(glow::COLOR_BUFFER_BIT);
    }*/

    // Run the main loop
    gamestate.tick(bterm);

    // Tell each console to draw itself
    render_consoles().unwrap();

    // If there is a GL callback, call it now
    /*{
        let be = BACKEND.lock();
        if let Some(callback) = be.gl_callback.as_ref() {
            let gl = be.gl.as_ref().unwrap();
            callback(gamestate, gl);
        }
    }*/

    /*if bterm.post_scanlines {
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
                &filename,
                &image::imageops::flip_vertical(&img),
                w,
                h,
                image::ColorType::Rgba8,
            )
            .expect("Failed to save buffer to the specified path");
        }
        be.request_screenshot = None;
    }*/
}

pub(crate) fn rebuild_consoles() {
    let mut consoles = CONSOLE_BACKING.lock();
    let mut bi = BACKEND_INTERNAL.lock();
    //let ss = bi.sprite_sheets.clone();
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
                    let be = BACKEND.lock();
                    let wgpu = be.wgpu.as_ref().unwrap();
                    backing.rebuild_vertices(
                        wgpu,
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
                    let be = BACKEND.lock();
                    let wgpu = be.wgpu.as_ref().unwrap();
                      backing.rebuild_vertices(
                          wgpu,
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
              /*ConsoleBacking::Fancy { backing } => {
                  let mut fc = bi.consoles[i]
                      .console
                      .as_any_mut()
                      .downcast_mut::<FlexiConsole>()
                      .unwrap();
                  if fc.is_dirty {
                      fc.tiles.sort_by(|a, b| a.z_order.cmp(&b.z_order));
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
              ConsoleBacking::Sprite { backing } => {
                  let mut sc = bi.consoles[i]
                      .console
                      .as_any_mut()
                      .downcast_mut::<SpriteConsole>()
                      .unwrap();
                  if sc.is_dirty {
                      sc.sprites.sort_by(|a, b| a.z_order.cmp(&b.z_order));
                      backing.rebuild_vertices(
                          sc.height,
                          sc.width,
                          &sc.sprites,
                          &ss[sc.sprite_sheet],
                      );
                      sc.needs_resize_internal = false;
                  }
              }*/
        }
    }
}

pub(crate) fn render_consoles() -> BResult<()> {
    let bi = BACKEND_INTERNAL.lock();
    let mut consoles = CONSOLE_BACKING.lock();
    let output = BACKEND.lock().wgpu.as_ref().unwrap().surface.get_current_texture()?;
    for (i, c) in consoles.iter_mut().enumerate() {
        let cons = &bi.consoles[i];
        let font = &bi.fonts[cons.font_index];
        match c {
            ConsoleBacking::Simple { backing } => {
                backing.wgpu_draw(&output, font)?;
            }
            ConsoleBacking::Sparse { backing } => {
                  backing.wgpu_draw(&output, font)?;
            }/*
              ConsoleBacking::Fancy { backing } => {
                  backing.gl_draw(font, shader)?;
              }
              ConsoleBacking::Sprite { backing } => {
                  backing.gl_draw(bi.sprite_sheets[0].backing.as_ref().unwrap(), shader)?;
              }*/
        }
    }
    output.present();
    Ok(())
}

pub(crate) fn check_console_backing() {
    let be = BACKEND.lock();
    let mut consoles = CONSOLE_BACKING.lock();
    if consoles.is_empty() {
        // Easy case: there are no consoles so we need to make them all.
        let bit = BACKEND_INTERNAL.lock();
        for cons in &bit.consoles {
            let cons_any = cons.console.as_any();
            if let Some(st) = cons_any.downcast_ref::<SimpleConsole>() {
                consoles.push(ConsoleBacking::Simple {
                    backing: SimpleConsoleBackend::new(
                        st.width as usize,
                        st.height as usize,
                        be.wgpu.as_ref().unwrap(),
                        &bit.shaders[cons.shader_index],
                        &bit.fonts[cons.font_index],
                    ),
                });
            } else if let Some(sp) = cons_any.downcast_ref::<SparseConsole>() {
                  consoles.push(ConsoleBacking::Sparse {
                      backing: SparseConsoleBackend::new(
                          be.wgpu.as_ref().unwrap(),
                          &bit.shaders[cons.shader_index],
                        &bit.fonts[cons.font_index]
                      ),
                  });
              }/* else if let Some(sp) = cons_any.downcast_ref::<FlexiConsole>() {
                  consoles.push(ConsoleBacking::Fancy {
                      backing: FancyConsoleBackend::new(
                          sp.width as usize,
                          sp.height as usize,
                          be.gl.as_ref().unwrap(),
                      ),
                  });
              } else if let Some(sp) = cons_any.downcast_ref::<SpriteConsole>() {
                  consoles.push(ConsoleBacking::Sprite {
                      backing: SpriteConsoleBackend::new(
                          sp.width as usize,
                          sp.height as usize,
                          be.gl.as_ref().unwrap(),
                      ),
                  });
              } else {
                  panic!("Unknown console type.");
              }*/
        }
    }
}

fn clear_screen_pass() -> Result<(), wgpu::SurfaceError> {
    let mut be = BACKEND.lock();
    if let Some(wgpu) = be.wgpu.as_mut() {
        let output = wgpu.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = wgpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
        }

        // submit will accept anything that implements IntoIter
        wgpu.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    } else {
        Err(wgpu::SurfaceError::OutOfMemory)
    }
}
