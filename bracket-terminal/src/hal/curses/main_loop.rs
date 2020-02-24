use crate::Result;
use crate::prelude::{Console, GameState, BTerm};
use crate::hal::VirtualKeyCode;
use pancurses::{endwin, initscr, noecho, Window};
use std::time::Instant;

pub fn main_loop<GS: GameState>(mut bterm: BTerm, mut gamestate: GS) -> Result<()> {
    let now = Instant::now();
    let mut prev_seconds = now.elapsed().as_secs();
    let mut prev_ms = now.elapsed().as_millis();
    let mut frames = 0;

    let dummy_shader = super::shader::Shader {};

    while !bterm.quitting {
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
        let input = bterm.backend.platform.window.getch();
        if let Some(input) = input {
            //println!("{:?}", input);

            match input {
                pancurses::Input::KeyLeft => bterm.key = Some(VirtualKeyCode::Left),
                pancurses::Input::KeyRight => bterm.key = Some(VirtualKeyCode::Right),
                pancurses::Input::KeyUp => bterm.key = Some(VirtualKeyCode::Up),
                pancurses::Input::KeyDown => bterm.key = Some(VirtualKeyCode::Down),
                pancurses::Input::KeyHome => bterm.key = Some(VirtualKeyCode::Home),
                pancurses::Input::KeyMouse => {
                    if let Ok(mouse_event) = pancurses::getmouse() {
                        if mouse_event.bstate & pancurses::BUTTON1_CLICKED > 0 {
                            bterm.left_click = true;
                        }
                        bterm.mouse_pos = (mouse_event.x, mouse_event.y);
                    }
                }
                pancurses::Input::Character(c) => match c {
                    '`' => bterm.key = Some(VirtualKeyCode::Grave),
                    '1' => bterm.key = Some(VirtualKeyCode::Key1),
                    '2' => bterm.key = Some(VirtualKeyCode::Key2),
                    '3' => bterm.key = Some(VirtualKeyCode::Key3),
                    '4' => bterm.key = Some(VirtualKeyCode::Key4),
                    '5' => bterm.key = Some(VirtualKeyCode::Key5),
                    '6' => bterm.key = Some(VirtualKeyCode::Key6),
                    '7' => bterm.key = Some(VirtualKeyCode::Key7),
                    '8' => bterm.key = Some(VirtualKeyCode::Key8),
                    '9' => bterm.key = Some(VirtualKeyCode::Key9),
                    '0' => bterm.key = Some(VirtualKeyCode::Key0),
                    'a' => bterm.key = Some(VirtualKeyCode::A),
                    'b' => bterm.key = Some(VirtualKeyCode::B),
                    'c' => bterm.key = Some(VirtualKeyCode::C),
                    'd' => bterm.key = Some(VirtualKeyCode::D),
                    'e' => bterm.key = Some(VirtualKeyCode::E),
                    'f' => bterm.key = Some(VirtualKeyCode::F),
                    'g' => bterm.key = Some(VirtualKeyCode::G),
                    'h' => bterm.key = Some(VirtualKeyCode::H),
                    'i' => bterm.key = Some(VirtualKeyCode::I),
                    'j' => bterm.key = Some(VirtualKeyCode::J),
                    'k' => bterm.key = Some(VirtualKeyCode::K),
                    'l' => bterm.key = Some(VirtualKeyCode::L),
                    'm' => bterm.key = Some(VirtualKeyCode::M),
                    'n' => bterm.key = Some(VirtualKeyCode::N),
                    'o' => bterm.key = Some(VirtualKeyCode::O),
                    'p' => bterm.key = Some(VirtualKeyCode::P),
                    'q' => bterm.key = Some(VirtualKeyCode::Q),
                    'r' => bterm.key = Some(VirtualKeyCode::R),
                    's' => bterm.key = Some(VirtualKeyCode::S),
                    't' => bterm.key = Some(VirtualKeyCode::T),
                    'u' => bterm.key = Some(VirtualKeyCode::U),
                    'v' => bterm.key = Some(VirtualKeyCode::V),
                    'w' => bterm.key = Some(VirtualKeyCode::W),
                    'x' => bterm.key = Some(VirtualKeyCode::X),
                    'y' => bterm.key = Some(VirtualKeyCode::Y),
                    'z' => bterm.key = Some(VirtualKeyCode::Z),
                    '\t' => bterm.key = Some(VirtualKeyCode::Tab),
                    '\n' => bterm.key = Some(VirtualKeyCode::Return),
                    ',' => bterm.key = Some(VirtualKeyCode::Comma),
                    '.' => bterm.key = Some(VirtualKeyCode::Period),
                    '/' => bterm.key = Some(VirtualKeyCode::Slash),
                    '[' => bterm.key = Some(VirtualKeyCode::LBracket),
                    ']' => bterm.key = Some(VirtualKeyCode::RBracket),
                    '\\' => bterm.key = Some(VirtualKeyCode::Backslash),
                    _ => {}
                },
                _ => {}
            }
        }

        gamestate.tick(&mut bterm);

        for cons in &mut bterm.consoles {
            cons.console.rebuild_if_dirty(&bterm.backend);
        }

        bterm.backend.platform.window.clear();

        // Tell each console to draw itself
        for cons in &mut bterm.consoles {
            cons.console
                .gl_draw(&bterm.fonts[cons.font_index], &dummy_shader, &bterm.backend);
        }

        bterm.backend.platform.window.refresh();

        crate::hal::fps_sleep(bterm.backend.platform.frame_sleep_time, &now, prev_ms);
    }

    endwin();
    Ok(())
}
