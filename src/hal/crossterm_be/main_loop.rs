use super::super::super::{GameState, Rltk};
use super::VirtualKeyCode;
use crossterm::event::{poll, read, Event};
use crossterm::execute;
use crossterm::terminal::SetSize;
use std::io::{stdout, Write};
use std::time::Duration;
use std::time::Instant;

pub fn main_loop<GS: GameState>(mut rltk: Rltk, mut gamestate: GS) {
    let now = Instant::now();
    let mut prev_seconds = now.elapsed().as_secs();
    let mut prev_ms = now.elapsed().as_millis();
    let mut frames = 0;

    let dummy_shader = super::shader::Shader {};

    while !rltk.quitting {
        let now_seconds = now.elapsed().as_secs();
        frames += 1;

        if now_seconds > prev_seconds {
            rltk.fps = frames as f32 / (now_seconds - prev_seconds) as f32;
            frames = 0;
            prev_seconds = now_seconds;
        }

        let now_ms = now.elapsed().as_millis();
        if now_ms > prev_ms {
            rltk.frame_time_ms = (now_ms - prev_ms) as f32;
            prev_ms = now_ms;
        }

        // Input
        rltk.left_click = false;
        rltk.key = None;
        rltk.shift = false;
        rltk.control = false;
        rltk.alt = false;

        // Input handler goes here
        while poll(Duration::from_secs(0)).unwrap() {
            match read().expect("Uh oh") {
                Event::Mouse(event) => {
                    //println!("{:?}", event);
                    // Button capture goes here
                    // Mouse doesn't seem to support cursor position? That's going to cause issues.
                    match event {
                        crossterm::event::MouseEvent::Down(_button, x, y, _modifiers) => {
                            rltk.left_click = true;
                            rltk.mouse_pos = (x as i32 * 8, y as i32 * 8);
                        }
                        _ => {}
                    }
                }
                Event::Key(key) => {
                    // Including because it eats my ctrl-C to quit!
                    if key.code == crossterm::event::KeyCode::Char('c')
                        && key.modifiers == crossterm::event::KeyModifiers::CONTROL
                    {
                        rltk.quitting = true;
                    }

                    use crossterm::event::KeyCode;
                    match key.code {
                        KeyCode::Left => rltk.key = Some(VirtualKeyCode::Left),
                        KeyCode::Right => rltk.key = Some(VirtualKeyCode::Right),
                        KeyCode::Up => rltk.key = Some(VirtualKeyCode::Up),
                        KeyCode::Down => rltk.key = Some(VirtualKeyCode::Down),
                        KeyCode::Backspace => rltk.key = Some(VirtualKeyCode::Delete),
                        KeyCode::Enter => rltk.key = Some(VirtualKeyCode::Return),
                        KeyCode::Home => rltk.key = Some(VirtualKeyCode::Home),
                        KeyCode::End => rltk.key = Some(VirtualKeyCode::End),
                        KeyCode::PageUp => rltk.key = Some(VirtualKeyCode::PageUp),
                        KeyCode::PageDown => rltk.key = Some(VirtualKeyCode::PageDown),
                        KeyCode::Tab => rltk.key = Some(VirtualKeyCode::Tab),
                        KeyCode::Delete => rltk.key = Some(VirtualKeyCode::Delete),
                        KeyCode::Insert => rltk.key = Some(VirtualKeyCode::Insert),
                        KeyCode::Esc => rltk.key = Some(VirtualKeyCode::Escape),
                        KeyCode::F(1) => rltk.key = Some(VirtualKeyCode::F1),
                        KeyCode::F(2) => rltk.key = Some(VirtualKeyCode::F2),
                        KeyCode::F(3) => rltk.key = Some(VirtualKeyCode::F3),
                        KeyCode::F(4) => rltk.key = Some(VirtualKeyCode::F4),
                        KeyCode::F(5) => rltk.key = Some(VirtualKeyCode::F5),
                        KeyCode::F(6) => rltk.key = Some(VirtualKeyCode::F6),
                        KeyCode::F(7) => rltk.key = Some(VirtualKeyCode::F7),
                        KeyCode::F(8) => rltk.key = Some(VirtualKeyCode::F8),
                        KeyCode::F(9) => rltk.key = Some(VirtualKeyCode::F9),
                        KeyCode::F(10) => rltk.key = Some(VirtualKeyCode::F10),
                        KeyCode::F(11) => rltk.key = Some(VirtualKeyCode::F11),
                        KeyCode::F(12) => rltk.key = Some(VirtualKeyCode::F12),
                        KeyCode::Char('`') => rltk.key = Some(VirtualKeyCode::Grave),
                        KeyCode::Char('1') => rltk.key = Some(VirtualKeyCode::Key1),
                        KeyCode::Char('2') => rltk.key = Some(VirtualKeyCode::Key2),
                        KeyCode::Char('3') => rltk.key = Some(VirtualKeyCode::Key3),
                        KeyCode::Char('4') => rltk.key = Some(VirtualKeyCode::Key4),
                        KeyCode::Char('5') => rltk.key = Some(VirtualKeyCode::Key5),
                        KeyCode::Char('6') => rltk.key = Some(VirtualKeyCode::Key6),
                        KeyCode::Char('7') => rltk.key = Some(VirtualKeyCode::Key7),
                        KeyCode::Char('8') => rltk.key = Some(VirtualKeyCode::Key8),
                        KeyCode::Char('9') => rltk.key = Some(VirtualKeyCode::Key9),
                        KeyCode::Char('0') => rltk.key = Some(VirtualKeyCode::Key0),
                        KeyCode::Char('-') => rltk.key = Some(VirtualKeyCode::Minus),
                        KeyCode::Char('=') => rltk.key = Some(VirtualKeyCode::Equals),
                        KeyCode::Char('a') => rltk.key = Some(VirtualKeyCode::A),
                        KeyCode::Char('b') => rltk.key = Some(VirtualKeyCode::B),
                        KeyCode::Char('c') => rltk.key = Some(VirtualKeyCode::C),
                        KeyCode::Char('d') => rltk.key = Some(VirtualKeyCode::D),
                        KeyCode::Char('e') => rltk.key = Some(VirtualKeyCode::E),
                        KeyCode::Char('f') => rltk.key = Some(VirtualKeyCode::F),
                        KeyCode::Char('g') => rltk.key = Some(VirtualKeyCode::G),
                        KeyCode::Char('h') => rltk.key = Some(VirtualKeyCode::H),
                        KeyCode::Char('i') => rltk.key = Some(VirtualKeyCode::I),
                        KeyCode::Char('j') => rltk.key = Some(VirtualKeyCode::J),
                        KeyCode::Char('k') => rltk.key = Some(VirtualKeyCode::K),
                        KeyCode::Char('l') => rltk.key = Some(VirtualKeyCode::L),
                        KeyCode::Char('m') => rltk.key = Some(VirtualKeyCode::M),
                        KeyCode::Char('n') => rltk.key = Some(VirtualKeyCode::N),
                        KeyCode::Char('o') => rltk.key = Some(VirtualKeyCode::O),
                        KeyCode::Char('p') => rltk.key = Some(VirtualKeyCode::P),
                        KeyCode::Char('q') => rltk.key = Some(VirtualKeyCode::Q),
                        KeyCode::Char('r') => rltk.key = Some(VirtualKeyCode::R),
                        KeyCode::Char('s') => rltk.key = Some(VirtualKeyCode::S),
                        KeyCode::Char('t') => rltk.key = Some(VirtualKeyCode::T),
                        KeyCode::Char('u') => rltk.key = Some(VirtualKeyCode::U),
                        KeyCode::Char('v') => rltk.key = Some(VirtualKeyCode::V),
                        KeyCode::Char('w') => rltk.key = Some(VirtualKeyCode::W),
                        KeyCode::Char('x') => rltk.key = Some(VirtualKeyCode::X),
                        KeyCode::Char('y') => rltk.key = Some(VirtualKeyCode::Y),
                        KeyCode::Char('z') => rltk.key = Some(VirtualKeyCode::Z),
                        KeyCode::Char('[') => rltk.key = Some(VirtualKeyCode::LBracket),
                        KeyCode::Char(']') => rltk.key = Some(VirtualKeyCode::RBracket),
                        KeyCode::Char('\\') => rltk.key = Some(VirtualKeyCode::Backslash),
                        KeyCode::Char(';') => rltk.key = Some(VirtualKeyCode::Semicolon),
                        KeyCode::Char('\'') => rltk.key = Some(VirtualKeyCode::Apostrophe),
                        KeyCode::Char(',') => rltk.key = Some(VirtualKeyCode::Comma),
                        KeyCode::Char('.') => rltk.key = Some(VirtualKeyCode::Period),
                        KeyCode::Char('/') => rltk.key = Some(VirtualKeyCode::Slash),

                        _ => {}
                    }

                    // Modifier handling
                    if key.modifiers == crossterm::event::KeyModifiers::CONTROL {
                        rltk.control = true;
                    }
                    if key.modifiers == crossterm::event::KeyModifiers::SHIFT {
                        rltk.shift = true;
                    }
                    if key.modifiers == crossterm::event::KeyModifiers::ALT {
                        rltk.alt = true;
                    }
                }
                _ => {}
            }
        }

        gamestate.tick(&mut rltk);

        for cons in &mut rltk.consoles {
            cons.console.rebuild_if_dirty(&rltk.backend);
        }

        // Tell each console to draw itself
        for cons in &mut rltk.consoles {
            cons.console
                .gl_draw(&rltk.fonts[cons.font_index], &dummy_shader, &rltk.backend);
        }

        //rltk.backend.platform.window.refresh();
        stdout().flush().expect("Command fail");

        crate::hal::fps_sleep(rltk.backend.platform.frame_sleep_time, &now, prev_ms);
    }

    println!(
        "Returning size to {}x{}",
        rltk.backend.platform.old_width, rltk.backend.platform.old_height
    );
    execute!(
        stdout(),
        SetSize(
            rltk.backend.platform.old_width,
            rltk.backend.platform.old_height
        )
    )
    .expect("Unable to resize");
    execute!(
        stdout(),
        crossterm::style::SetForegroundColor(crossterm::style::Color::Rgb {
            r: 255,
            g: 255,
            b: 255
        })
    )
    .expect("Unable to recolor");
    execute!(
        stdout(),
        crossterm::style::SetBackgroundColor(crossterm::style::Color::Rgb { r: 0, g: 0, b: 0 })
    )
    .expect("Unable to recolor");
    execute!(stdout(), crossterm::cursor::Show).expect("Command fail");
}
