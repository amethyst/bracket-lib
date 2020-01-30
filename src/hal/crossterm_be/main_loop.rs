use super::super::super::{GameState, Rltk};
use super::VirtualKeyCode;
use std::time::Instant;
use std::io::{stdout, Write};
use crossterm::terminal::{SetSize};
use crossterm::{execute, queue};
use crossterm::event::{poll, read, Event};
use std::time::Duration;

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
                        crossterm::event::MouseEvent::Down(button, x, y, modifiers) => {
                            rltk.left_click = true;
                            rltk.mouse_pos = (x as i32 * 8, y as i32 * 8);
                        }
                        _ => {}
                    }
                }
                Event::Key(key) => {
                    // Including because it eats my ctrl-C to quit!
                    if key.code == crossterm::event::KeyCode::Char('c') && key.modifiers == crossterm::event::KeyModifiers::CONTROL {
                        rltk.quitting = true;
                    }
                    // TODO: A whole lot more key conversion

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
    }

    println!("Returning size to {}x{}", rltk.backend.platform.old_width, rltk.backend.platform.old_height);
    execute!(stdout(), SetSize(rltk.backend.platform.old_width, rltk.backend.platform.old_height)).expect("Unable to resize");
    execute!(stdout(), crossterm::style::SetForegroundColor(
        crossterm::style::Color::Rgb{
            r: 255, g: 255, b: 255
        }
    )).expect("Unable to recolor");
    execute!(stdout(), crossterm::style::SetBackgroundColor(
        crossterm::style::Color::Rgb{
            r: 0, g: 0, b: 0
        }
    )).expect("Unable to recolor");
    execute!(stdout(), crossterm::cursor::Show).expect("Command fail");
}
