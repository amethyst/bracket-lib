use super::keycode_to_key;
use super::{virtual_key_code_to_scan, BACKEND};
use crate::consoles::Console;
use crate::prelude::{
    to_char, BEvent, BTerm, GameState, SimpleConsole, SparseConsole, VirtualKeyCode,
    BACKEND_INTERNAL,
};
use crate::{clear_input_state, BResult};
use bracket_color::prelude::*;
use crossterm::event::{poll, read, Event};
use crossterm::execute;
use crossterm::style::Print;
use crossterm::terminal::SetSize;
use crossterm::{cursor, queue};
use std::collections::HashSet;
use std::io::{stdout, Write};
use std::time::Duration;
use std::time::Instant;

pub fn main_loop<GS: GameState>(mut bterm: BTerm, mut gamestate: GS) -> BResult<()> {
    let now = Instant::now();
    let mut prev_seconds = now.elapsed().as_secs();
    let mut prev_ms = now.elapsed().as_millis();
    let mut frames = 0;

    // Panic handler to reset terminal
    ctrlc::set_handler(move || {
        reset_terminal();
    });

    let mut key_map: HashSet<crossterm::event::KeyCode> = HashSet::new();
    let mut keys_this_frame: HashSet<crossterm::event::KeyCode> = HashSet::new();
    let mut output_buffer: Option<Vec<OutputBuffer>> = None;

    crossterm::terminal::enable_raw_mode().expect("Raw mode failed");

    'main: while !bterm.quitting {
        let now_seconds = now.elapsed().as_secs();
        frames += 1;

        if now_seconds > prev_seconds {
            bterm.fps = frames as f32 / (now_seconds - prev_seconds) as f32;
            frames = 0;
            prev_seconds = now_seconds;
        }

        let now_ms = now.elapsed().as_millis();
        if now_ms > prev_ms {
            bterm.frame_time_ms = (now_ms - prev_ms) as f32;
            prev_ms = now_ms;
        }

        // Input
        clear_input_state(&mut bterm);

        // Input handler goes here
        keys_this_frame.clear();
        while poll(Duration::from_millis(0))? {
            match read().expect("Uh oh") {
                Event::Mouse(event) => {
                    //println!("{:?}", event);
                    // Button capture goes here
                    // Mouse doesn't seem to support cursor position? That's going to cause issues.
                    match event.kind {
                        crossterm::event::MouseEventKind::Down(button) => {
                            bterm.left_click = true;
                            bterm.mouse_pos = (event.column as i32 * 8, event.row as i32 * 8);
                            bterm.on_mouse_position(
                                event.column as f64 * 8.0,
                                event.row as f64 * 8.0,
                            );
                            bterm.on_mouse_button(button as usize, true);
                        }
                        crossterm::event::MouseEventKind::Up(button) => {
                            bterm.on_mouse_button(button as usize, false);
                        }
                        crossterm::event::MouseEventKind::Drag(..) => {
                            bterm.on_mouse_position(
                                event.column as f64 * 8.0,
                                event.row as f64 * 8.0,
                            );
                        }
                        _ => {
                            //eprintln!("{:?}", event);
                        }
                    }
                }
                Event::Key(key) => {
                    // Including because it eats my ctrl-C to quit!
                    if key.code == crossterm::event::KeyCode::Char('c')
                        && key.modifiers == crossterm::event::KeyModifiers::CONTROL
                    {
                        break 'main;
                    }
                    keys_this_frame.insert(key.code);
                    if !key_map.contains(&key.code) {
                        key_map.insert(key.code);
                        if let Some(key) = keycode_to_key(key.code) {
                            bterm.on_key(key, virtual_key_code_to_scan(key), true);
                            // How do I get the scancode?
                        }
                    }

                    // Modifier handling
                    if key.modifiers == crossterm::event::KeyModifiers::CONTROL {
                        bterm.control = true;
                    }
                    if key.modifiers == crossterm::event::KeyModifiers::SHIFT {
                        bterm.shift = true;
                    }
                    if key.modifiers == crossterm::event::KeyModifiers::ALT {
                        bterm.alt = true;
                    }
                }
                Event::Resize(x, y) => {
                    bterm.on_event(BEvent::Resized {
                        new_size: bracket_geometry::prelude::Point::new(x, y),
                        dpi_scale_factor: 1.0,
                    });
                }
                _ => {}
            }
        }

        let keys_released = key_map
            .iter()
            .filter(|k| !keys_this_frame.contains(k))
            .map(|k| *k)
            .collect::<Vec<crossterm::event::KeyCode>>();
        for key in keys_released {
            key_map.remove(&key);
            if let Some(key) = keycode_to_key(key) {
                bterm.on_key(key, virtual_key_code_to_scan(key), false);
            }
        }

        gamestate.tick(&mut bterm);

        if output_buffer.is_none() {
            output_buffer = Some(full_redraw()?);
        } else {
            partial_redraw(output_buffer.as_mut().unwrap());
        }

        crate::hal::fps_sleep(BACKEND.lock().frame_sleep_time, &now, prev_ms);
    }

    let be = BACKEND.lock();
    execute!(stdout(), SetSize(be.old_width, be.old_height)).expect("Unable to resize");
    reset_terminal();
    Ok(())
}

fn reset_terminal() {
    execute!(stdout(), crossterm::style::ResetColor).expect("Command fail");
    execute!(stdout(), crossterm::cursor::Show).expect("Command fail");
    execute!(stdout(), crossterm::terminal::LeaveAlternateScreen).expect("Command fail");
    execute!(stdout(), crossterm::event::DisableMouseCapture).expect("Command fail");
    crossterm::terminal::disable_raw_mode();
}

#[derive(Clone, PartialEq)]
struct OutputBuffer {
    glyph: char,
    fg: RGBA,
    bg: RGBA,
}

impl Default for OutputBuffer {
    fn default() -> Self {
        Self {
            glyph: ' ',
            fg: RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
            bg: RGBA::from_f32(0.0, 0.0, 0.0, 0.0),
        }
    }
}

fn full_redraw() -> BResult<Vec<OutputBuffer>> {
    let be = BACKEND.lock();
    let mut bi = BACKEND_INTERNAL.lock();

    let (width, height) = crossterm::terminal::size()?;
    let mut buffer = vec![OutputBuffer::default(); height as usize * width as usize];

    // Tell each console to draw itself
    for cons in &mut bi.consoles {
        let cons_any = cons.console.as_any_mut();
        if let Some(st) = cons_any.downcast_mut::<SimpleConsole>() {
            if st.is_dirty {
                st.clear_dirty();
                let mut idx = 0;
                let mut last_bg = RGBA::new();
                let mut last_fg = RGBA::new();
                for y in 0..st.height {
                    queue!(
                        stdout(),
                        cursor::MoveTo(0, st.height as u16 - (y as u16 + 1))
                    )
                    .expect("Command fail");
                    let mut buf_idx = (st.height as u16 - (y as u16 + 1)) as usize * width as usize;
                    for x in 0..st.width {
                        let t = &st.tiles[idx];
                        if t.fg != last_fg {
                            queue!(
                                stdout(),
                                crossterm::style::SetForegroundColor(
                                    crossterm::style::Color::Rgb {
                                        r: (t.fg.r * 255.0) as u8,
                                        g: (t.fg.g * 255.0) as u8,
                                        b: (t.fg.b * 255.0) as u8,
                                    }
                                )
                            )
                            .expect("Command fail");
                            last_fg = t.fg;
                        }
                        if t.bg != last_bg {
                            queue!(
                                stdout(),
                                crossterm::style::SetBackgroundColor(
                                    crossterm::style::Color::Rgb {
                                        r: (t.bg.r * 255.0) as u8,
                                        g: (t.bg.g * 255.0) as u8,
                                        b: (t.bg.b * 255.0) as u8,
                                    }
                                )
                            )
                            .expect("Command fail");
                            last_bg = t.bg;
                        }
                        queue!(stdout(), Print(to_char(t.glyph as u8))).expect("Command fail");
                        buffer[buf_idx].glyph = to_char(t.glyph as u8);
                        buffer[buf_idx].fg = t.fg;
                        buffer[buf_idx].bg = t.bg;
                        idx += 1;
                        buf_idx += 1;
                    }
                }
            }
        } else if let Some(st) = cons_any.downcast_mut::<SparseConsole>() {
            if st.is_dirty {
                st.clear_dirty();
                for t in st.tiles.iter() {
                    let x = t.idx as u32 % st.width;
                    let y = t.idx as u32 / st.width;
                    queue!(
                        stdout(),
                        cursor::MoveTo(x as u16, st.height as u16 - (y as u16 + 1) as u16)
                    )
                    .expect("Command fail");
                    queue!(
                        stdout(),
                        crossterm::style::SetForegroundColor(crossterm::style::Color::Rgb {
                            r: (t.fg.r * 255.0) as u8,
                            g: (t.fg.g * 255.0) as u8,
                            b: (t.fg.b * 255.0) as u8,
                        })
                    )
                    .expect("Command fail");
                    queue!(
                        stdout(),
                        crossterm::style::SetBackgroundColor(crossterm::style::Color::Rgb {
                            r: (t.bg.r * 255.0) as u8,
                            g: (t.bg.g * 255.0) as u8,
                            b: (t.bg.b * 255.0) as u8,
                        })
                    )
                    .expect("Command fail");
                    queue!(stdout(), Print(to_char(t.glyph as u8))).expect("Command fail");
                    let buf_idx =
                        (((st.height as u16 - (y as u16 + 1)) * height) + x as u16) as usize;
                    buffer[buf_idx].glyph = to_char(t.glyph as u8);
                    buffer[buf_idx].fg = t.fg;
                    buffer[buf_idx].bg = t.bg;
                }
            }
        }
    }

    //bterm.backend.platform.window.refresh();
    stdout().flush().expect("Command fail");

    Ok(buffer)
}

fn partial_redraw(buffer: &mut Vec<OutputBuffer>) {
    let be = BACKEND.lock();
    let mut bi = BACKEND_INTERNAL.lock();

    let (width, height) = crossterm::terminal::size().expect("Failed to get size");
    let mut dirty = Vec::new();

    // Iterate all consoles, rendering to the buffer and denoting dirty
    for cons in &mut bi.consoles {
        let cons_any = cons.console.as_any_mut();
        if let Some(st) = cons_any.downcast_mut::<SimpleConsole>() {
            if st.is_dirty {
                st.clear_dirty();
                let mut idx = 0;
                for y in 0..st.height {
                    let mut buf_idx = (st.height as u16 - (y as u16 + 1)) as usize * width as usize;
                    for x in 0..st.width {
                        let t = &st.tiles[idx];
                        let new_output = OutputBuffer {
                            glyph: to_char(t.glyph as u8),
                            fg: t.fg,
                            bg: t.bg,
                        };
                        if buffer[buf_idx] != new_output {
                            buffer[buf_idx] = new_output;
                            dirty.push(buf_idx);
                        }
                        idx += 1;
                        buf_idx += 1;
                    }
                }
            }
        } else if let Some(st) = cons_any.downcast_mut::<SparseConsole>() {
            if st.is_dirty {
                st.clear_dirty();
                for t in st.tiles.iter() {
                    let x = t.idx as u32 % st.width;
                    let y = t.idx as u32 / st.width;
                    let buf_idx =
                        (((st.height as u16 - (y as u16 + 1)) * height) + x as u16) as usize;
                    let new_output = OutputBuffer {
                        glyph: to_char(t.glyph as u8),
                        fg: t.fg,
                        bg: t.bg,
                    };
                    if buffer[buf_idx] != new_output {
                        buffer[buf_idx] = new_output;
                        dirty.push(buf_idx);
                    }
                }
            }
        }
    }

    // Render just the dirty tiles
    let mut last_bg = RGBA::new();
    let mut last_fg = RGBA::new();
    dirty.iter().for_each(|idx| {
        let x = idx % width as usize;
        let y = idx / width as usize;
        let t = &buffer[*idx];

        queue!(stdout(), cursor::MoveTo(x as u16, y as u16)).expect("Command fail");

        if t.fg != last_fg {
            queue!(
                stdout(),
                crossterm::style::SetForegroundColor(crossterm::style::Color::Rgb {
                    r: (t.fg.r * 255.0) as u8,
                    g: (t.fg.g * 255.0) as u8,
                    b: (t.fg.b * 255.0) as u8,
                })
            )
            .expect("Command fail");
            last_fg = t.fg;
        }

        if t.bg != last_bg {
            queue!(
                stdout(),
                crossterm::style::SetBackgroundColor(crossterm::style::Color::Rgb {
                    r: (t.bg.r * 255.0) as u8,
                    g: (t.bg.g * 255.0) as u8,
                    b: (t.bg.b * 255.0) as u8,
                })
            )
            .expect("Command fail");
            last_bg = t.bg;
        }
        queue!(stdout(), Print(t.glyph)).expect("Command fail");
    });
}
