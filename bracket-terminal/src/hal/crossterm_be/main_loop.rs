use crate::prelude::{BTerm, GameState, VirtualKeyCode, SimpleConsole, SparseConsole, to_char};
use crate::Result;
use crossterm::event::{poll, read, Event};
use crossterm::execute;
use crossterm::terminal::SetSize;
use crossterm::style::Print;
use crossterm::{cursor, queue};
use std::io::{stdout, Write};
use std::time::Duration;
use std::time::Instant;
use super::BACKEND;
use bracket_color::prelude::*;

pub fn main_loop<GS: GameState>(mut bterm: BTerm, mut gamestate: GS) -> Result<()> {
    let now = Instant::now();
    let mut prev_seconds = now.elapsed().as_secs();
    let mut prev_ms = now.elapsed().as_millis();
    let mut frames = 0;

    // Panic handler to reset terminal
    std::panic::set_hook(Box::new(|_| {
        reset_terminal();
    }));

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
        bterm.left_click = false;
        bterm.key = None;
        bterm.shift = false;
        bterm.control = false;
        bterm.alt = false;

        // Input handler goes here
        while poll(Duration::from_secs(0))? {
            match read().expect("Uh oh") {
                Event::Mouse(event) => {
                    //println!("{:?}", event);
                    // Button capture goes here
                    // Mouse doesn't seem to support cursor position? That's going to cause issues.
                    match event {
                        crossterm::event::MouseEvent::Down(_button, x, y, _modifiers) => {
                            bterm.left_click = true;
                            bterm.mouse_pos = (x as i32 * 8, y as i32 * 8);
                        }
                        _ => {}
                    }
                }
                Event::Key(key) => {
                    // Including because it eats my ctrl-C to quit!
                    if key.code == crossterm::event::KeyCode::Char('c')
                        && key.modifiers == crossterm::event::KeyModifiers::CONTROL
                    {
                        break 'main;
                    }

                    use crossterm::event::KeyCode;
                    match key.code {
                        KeyCode::Left => bterm.key = Some(VirtualKeyCode::Left),
                        KeyCode::Right => bterm.key = Some(VirtualKeyCode::Right),
                        KeyCode::Up => bterm.key = Some(VirtualKeyCode::Up),
                        KeyCode::Down => bterm.key = Some(VirtualKeyCode::Down),
                        KeyCode::Backspace => bterm.key = Some(VirtualKeyCode::Delete),
                        KeyCode::Enter => bterm.key = Some(VirtualKeyCode::Return),
                        KeyCode::Home => bterm.key = Some(VirtualKeyCode::Home),
                        KeyCode::End => bterm.key = Some(VirtualKeyCode::End),
                        KeyCode::PageUp => bterm.key = Some(VirtualKeyCode::PageUp),
                        KeyCode::PageDown => bterm.key = Some(VirtualKeyCode::PageDown),
                        KeyCode::Tab => bterm.key = Some(VirtualKeyCode::Tab),
                        KeyCode::Delete => bterm.key = Some(VirtualKeyCode::Delete),
                        KeyCode::Insert => bterm.key = Some(VirtualKeyCode::Insert),
                        KeyCode::Esc => bterm.key = Some(VirtualKeyCode::Escape),
                        KeyCode::F(1) => bterm.key = Some(VirtualKeyCode::F1),
                        KeyCode::F(2) => bterm.key = Some(VirtualKeyCode::F2),
                        KeyCode::F(3) => bterm.key = Some(VirtualKeyCode::F3),
                        KeyCode::F(4) => bterm.key = Some(VirtualKeyCode::F4),
                        KeyCode::F(5) => bterm.key = Some(VirtualKeyCode::F5),
                        KeyCode::F(6) => bterm.key = Some(VirtualKeyCode::F6),
                        KeyCode::F(7) => bterm.key = Some(VirtualKeyCode::F7),
                        KeyCode::F(8) => bterm.key = Some(VirtualKeyCode::F8),
                        KeyCode::F(9) => bterm.key = Some(VirtualKeyCode::F9),
                        KeyCode::F(10) => bterm.key = Some(VirtualKeyCode::F10),
                        KeyCode::F(11) => bterm.key = Some(VirtualKeyCode::F11),
                        KeyCode::F(12) => bterm.key = Some(VirtualKeyCode::F12),
                        KeyCode::Char('`') => bterm.key = Some(VirtualKeyCode::Grave),
                        KeyCode::Char('1') => bterm.key = Some(VirtualKeyCode::Key1),
                        KeyCode::Char('2') => bterm.key = Some(VirtualKeyCode::Key2),
                        KeyCode::Char('3') => bterm.key = Some(VirtualKeyCode::Key3),
                        KeyCode::Char('4') => bterm.key = Some(VirtualKeyCode::Key4),
                        KeyCode::Char('5') => bterm.key = Some(VirtualKeyCode::Key5),
                        KeyCode::Char('6') => bterm.key = Some(VirtualKeyCode::Key6),
                        KeyCode::Char('7') => bterm.key = Some(VirtualKeyCode::Key7),
                        KeyCode::Char('8') => bterm.key = Some(VirtualKeyCode::Key8),
                        KeyCode::Char('9') => bterm.key = Some(VirtualKeyCode::Key9),
                        KeyCode::Char('0') => bterm.key = Some(VirtualKeyCode::Key0),
                        KeyCode::Char('-') => bterm.key = Some(VirtualKeyCode::Minus),
                        KeyCode::Char('=') => bterm.key = Some(VirtualKeyCode::Equals),
                        KeyCode::Char('a') => bterm.key = Some(VirtualKeyCode::A),
                        KeyCode::Char('b') => bterm.key = Some(VirtualKeyCode::B),
                        KeyCode::Char('c') => bterm.key = Some(VirtualKeyCode::C),
                        KeyCode::Char('d') => bterm.key = Some(VirtualKeyCode::D),
                        KeyCode::Char('e') => bterm.key = Some(VirtualKeyCode::E),
                        KeyCode::Char('f') => bterm.key = Some(VirtualKeyCode::F),
                        KeyCode::Char('g') => bterm.key = Some(VirtualKeyCode::G),
                        KeyCode::Char('h') => bterm.key = Some(VirtualKeyCode::H),
                        KeyCode::Char('i') => bterm.key = Some(VirtualKeyCode::I),
                        KeyCode::Char('j') => bterm.key = Some(VirtualKeyCode::J),
                        KeyCode::Char('k') => bterm.key = Some(VirtualKeyCode::K),
                        KeyCode::Char('l') => bterm.key = Some(VirtualKeyCode::L),
                        KeyCode::Char('m') => bterm.key = Some(VirtualKeyCode::M),
                        KeyCode::Char('n') => bterm.key = Some(VirtualKeyCode::N),
                        KeyCode::Char('o') => bterm.key = Some(VirtualKeyCode::O),
                        KeyCode::Char('p') => bterm.key = Some(VirtualKeyCode::P),
                        KeyCode::Char('q') => bterm.key = Some(VirtualKeyCode::Q),
                        KeyCode::Char('r') => bterm.key = Some(VirtualKeyCode::R),
                        KeyCode::Char('s') => bterm.key = Some(VirtualKeyCode::S),
                        KeyCode::Char('t') => bterm.key = Some(VirtualKeyCode::T),
                        KeyCode::Char('u') => bterm.key = Some(VirtualKeyCode::U),
                        KeyCode::Char('v') => bterm.key = Some(VirtualKeyCode::V),
                        KeyCode::Char('w') => bterm.key = Some(VirtualKeyCode::W),
                        KeyCode::Char('x') => bterm.key = Some(VirtualKeyCode::X),
                        KeyCode::Char('y') => bterm.key = Some(VirtualKeyCode::Y),
                        KeyCode::Char('z') => bterm.key = Some(VirtualKeyCode::Z),
                        KeyCode::Char('[') => bterm.key = Some(VirtualKeyCode::LBracket),
                        KeyCode::Char(']') => bterm.key = Some(VirtualKeyCode::RBracket),
                        KeyCode::Char('\\') => bterm.key = Some(VirtualKeyCode::Backslash),
                        KeyCode::Char(';') => bterm.key = Some(VirtualKeyCode::Semicolon),
                        KeyCode::Char('\'') => bterm.key = Some(VirtualKeyCode::Apostrophe),
                        KeyCode::Char(',') => bterm.key = Some(VirtualKeyCode::Comma),
                        KeyCode::Char('.') => bterm.key = Some(VirtualKeyCode::Period),
                        KeyCode::Char('/') => bterm.key = Some(VirtualKeyCode::Slash),

                        _ => {}
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
                _ => {}
            }
        }

        gamestate.tick(&mut bterm);

        let be = BACKEND.lock().unwrap();

        // Tell each console to draw itself
        for cons in &mut bterm.consoles {
            let cons_any = cons.console.as_any();
            if let Some(st) = cons_any.downcast_ref::<SimpleConsole>() {
                if st.is_dirty {
                    let mut idx = 0;
                    let mut last_bg = RGB::new();
                    let mut last_fg = RGB::new();
                    for y in 0..st.height {
                        queue!(stdout(), cursor::MoveTo(0, st.height as u16 - (y as u16 + 1)))
                            .expect("Command fail");
                        for _x in 0..st.width {
                            let t = &st.tiles[idx];
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
                            queue!(stdout(), Print(to_char(t.glyph))).expect("Command fail");
                            idx += 1;
                        }
                    }
                }
            }
            else if let Some(st) = cons_any.downcast_ref::<SparseConsole>() {
                if st.is_dirty {
                    for t in st.tiles.iter() {
                        let x = t.idx as u32 % st.width;
                        let y = t.idx as u32 / st.width;
                        queue!(stdout(), cursor::MoveTo(x as u16, st.height as u16 - (y as u16 + 1) as u16))
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
                        queue!(stdout(), Print(to_char(t.glyph))).expect("Command fail");
                    }
                }
            }
        }

        //bterm.backend.platform.window.refresh();
        stdout().flush().expect("Command fail");

        crate::hal::fps_sleep(be.frame_sleep_time, &now, prev_ms);
    }

    let be = BACKEND.lock().unwrap();
    execute!(
        stdout(),
        SetSize(
            be.old_width,
            be.old_height
        )
    )
    .expect("Unable to resize");
    reset_terminal();
    Ok(())
}

fn reset_terminal() {
    execute!(stdout(), crossterm::style::ResetColor).expect("Command fail");
    execute!(stdout(), crossterm::cursor::Show).expect("Command fail");
}
