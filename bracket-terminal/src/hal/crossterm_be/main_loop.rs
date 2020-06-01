use super::{virtual_key_code_to_scan, BACKEND};
use crate::prelude::{
    to_char, BEvent, BTerm, GameState, SimpleConsole, SparseConsole, VirtualKeyCode,
    BACKEND_INTERNAL,
};
use crate::{clear_input_state, Result};
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

pub fn main_loop<GS: GameState>(mut bterm: BTerm, mut gamestate: GS) -> Result<()> {
    let now = Instant::now();
    let mut prev_seconds = now.elapsed().as_secs();
    let mut prev_ms = now.elapsed().as_millis();
    let mut frames = 0;

    // Panic handler to reset terminal
    std::panic::set_hook(Box::new(|_| {
        reset_terminal();
    }));

    let mut key_map: HashSet<crossterm::event::KeyCode> = HashSet::new();
    let mut keys_this_frame: HashSet<crossterm::event::KeyCode> = HashSet::new();

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
        while poll(Duration::from_secs(0))? {
            match read().expect("Uh oh") {
                Event::Mouse(event) => {
                    //println!("{:?}", event);
                    // Button capture goes here
                    // Mouse doesn't seem to support cursor position? That's going to cause issues.
                    match event {
                        crossterm::event::MouseEvent::Down(button, x, y, _modifiers) => {
                            bterm.left_click = true;
                            bterm.mouse_pos = (x as i32 * 8, y as i32 * 8);
                            bterm.on_mouse_position(x as f64 * 8.0, y as f64 * 8.0);
                            bterm.on_mouse_button(button as usize, true);
                        }
                        crossterm::event::MouseEvent::Up(button, _x, _y, _modifiers) => {
                            bterm.on_mouse_button(button as usize, false);
                        }
                        crossterm::event::MouseEvent::Drag(_button, x, y, _modifiers) => {
                            bterm.on_mouse_position(x as f64 * 8.0, y as f64 * 8.0);
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

        let be = BACKEND.lock();
        let mut bi = BACKEND_INTERNAL.lock();

        // Tell each console to draw itself
        for cons in &mut bi.consoles {
            let cons_any = cons.console.as_any();
            if let Some(st) = cons_any.downcast_ref::<SimpleConsole>() {
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
                        for _x in 0..st.width {
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
                            idx += 1;
                        }
                    }
                }
            } else if let Some(st) = cons_any.downcast_ref::<SparseConsole>() {
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
                    }
                }
            }
        }

        //bterm.backend.platform.window.refresh();
        stdout().flush().expect("Command fail");

        crate::hal::fps_sleep(be.frame_sleep_time, &now, prev_ms);
    }

    let be = BACKEND.lock();
    execute!(stdout(), SetSize(be.old_width, be.old_height)).expect("Unable to resize");
    reset_terminal();
    Ok(())
}

fn reset_terminal() {
    execute!(stdout(), crossterm::style::ResetColor).expect("Command fail");
    execute!(stdout(), crossterm::cursor::Show).expect("Command fail");
}

fn keycode_to_key(c: crossterm::event::KeyCode) -> Option<VirtualKeyCode> {
    use crossterm::event::KeyCode;
    match c {
        KeyCode::Left => Some(VirtualKeyCode::Left),
        KeyCode::Right => Some(VirtualKeyCode::Right),
        KeyCode::Up => Some(VirtualKeyCode::Up),
        KeyCode::Down => Some(VirtualKeyCode::Down),
        KeyCode::Backspace => Some(VirtualKeyCode::Delete),
        KeyCode::Enter => Some(VirtualKeyCode::Return),
        KeyCode::Home => Some(VirtualKeyCode::Home),
        KeyCode::End => Some(VirtualKeyCode::End),
        KeyCode::PageUp => Some(VirtualKeyCode::PageUp),
        KeyCode::PageDown => Some(VirtualKeyCode::PageDown),
        KeyCode::Tab => Some(VirtualKeyCode::Tab),
        KeyCode::Delete => Some(VirtualKeyCode::Delete),
        KeyCode::Insert => Some(VirtualKeyCode::Insert),
        KeyCode::Esc => Some(VirtualKeyCode::Escape),
        KeyCode::F(1) => Some(VirtualKeyCode::F1),
        KeyCode::F(2) => Some(VirtualKeyCode::F2),
        KeyCode::F(3) => Some(VirtualKeyCode::F3),
        KeyCode::F(4) => Some(VirtualKeyCode::F4),
        KeyCode::F(5) => Some(VirtualKeyCode::F5),
        KeyCode::F(6) => Some(VirtualKeyCode::F6),
        KeyCode::F(7) => Some(VirtualKeyCode::F7),
        KeyCode::F(8) => Some(VirtualKeyCode::F8),
        KeyCode::F(9) => Some(VirtualKeyCode::F9),
        KeyCode::F(10) => Some(VirtualKeyCode::F10),
        KeyCode::F(11) => Some(VirtualKeyCode::F11),
        KeyCode::F(12) => Some(VirtualKeyCode::F12),
        KeyCode::Char('`') => Some(VirtualKeyCode::Grave),
        KeyCode::Char('1') => Some(VirtualKeyCode::Key1),
        KeyCode::Char('2') => Some(VirtualKeyCode::Key2),
        KeyCode::Char('3') => Some(VirtualKeyCode::Key3),
        KeyCode::Char('4') => Some(VirtualKeyCode::Key4),
        KeyCode::Char('5') => Some(VirtualKeyCode::Key5),
        KeyCode::Char('6') => Some(VirtualKeyCode::Key6),
        KeyCode::Char('7') => Some(VirtualKeyCode::Key7),
        KeyCode::Char('8') => Some(VirtualKeyCode::Key8),
        KeyCode::Char('9') => Some(VirtualKeyCode::Key9),
        KeyCode::Char('0') => Some(VirtualKeyCode::Key0),
        KeyCode::Char('-') => Some(VirtualKeyCode::Minus),
        KeyCode::Char('=') => Some(VirtualKeyCode::Equals),
        KeyCode::Char('a') => Some(VirtualKeyCode::A),
        KeyCode::Char('b') => Some(VirtualKeyCode::B),
        KeyCode::Char('c') => Some(VirtualKeyCode::C),
        KeyCode::Char('d') => Some(VirtualKeyCode::D),
        KeyCode::Char('e') => Some(VirtualKeyCode::E),
        KeyCode::Char('f') => Some(VirtualKeyCode::F),
        KeyCode::Char('g') => Some(VirtualKeyCode::G),
        KeyCode::Char('h') => Some(VirtualKeyCode::H),
        KeyCode::Char('i') => Some(VirtualKeyCode::I),
        KeyCode::Char('j') => Some(VirtualKeyCode::J),
        KeyCode::Char('k') => Some(VirtualKeyCode::K),
        KeyCode::Char('l') => Some(VirtualKeyCode::L),
        KeyCode::Char('m') => Some(VirtualKeyCode::M),
        KeyCode::Char('n') => Some(VirtualKeyCode::N),
        KeyCode::Char('o') => Some(VirtualKeyCode::O),
        KeyCode::Char('p') => Some(VirtualKeyCode::P),
        KeyCode::Char('q') => Some(VirtualKeyCode::Q),
        KeyCode::Char('r') => Some(VirtualKeyCode::R),
        KeyCode::Char('s') => Some(VirtualKeyCode::S),
        KeyCode::Char('t') => Some(VirtualKeyCode::T),
        KeyCode::Char('u') => Some(VirtualKeyCode::U),
        KeyCode::Char('v') => Some(VirtualKeyCode::V),
        KeyCode::Char('w') => Some(VirtualKeyCode::W),
        KeyCode::Char('x') => Some(VirtualKeyCode::X),
        KeyCode::Char('y') => Some(VirtualKeyCode::Y),
        KeyCode::Char('z') => Some(VirtualKeyCode::Z),
        KeyCode::Char('[') => Some(VirtualKeyCode::LBracket),
        KeyCode::Char(']') => Some(VirtualKeyCode::RBracket),
        KeyCode::Char('\\') => Some(VirtualKeyCode::Backslash),
        KeyCode::Char(';') => Some(VirtualKeyCode::Semicolon),
        KeyCode::Char('\'') => Some(VirtualKeyCode::Apostrophe),
        KeyCode::Char(',') => Some(VirtualKeyCode::Comma),
        KeyCode::Char('.') => Some(VirtualKeyCode::Period),
        KeyCode::Char('/') => Some(VirtualKeyCode::Slash),

        _ => None,
    }
}
