use crate::hal::VirtualKeyCode;
use crate::prelude::{BTerm, GameState, to_char, SimpleConsole, SparseConsole};
use crate::Result;
use pancurses::endwin;
use std::time::Instant;
use std::convert::TryInto;
use super::*;

pub fn main_loop<GS: GameState>(mut bterm: BTerm, mut gamestate: GS) -> Result<()> {
    let now = Instant::now();
    let mut prev_seconds = now.elapsed().as_secs();
    let mut prev_ms = now.elapsed().as_millis();
    let mut frames = 0;

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
        let input = BACKEND.lock().unwrap().window.as_ref().unwrap().getch();
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

        let be = BACKEND.lock().unwrap();
        let window = be.window.as_ref().unwrap();

        window.clear();

        // Tell each console to draw itself
        for cons in &mut bterm.consoles {
            let cons_any = cons.console.as_any();
            if let Some(st) = cons_any.downcast_ref::<SimpleConsole>() {
                let mut idx = 0;
                for y in 0..st.height {
                    for x in 0..st.width {
                        let t = &st.tiles[idx];
                        let cp_fg = find_nearest_color(t.fg, &be.color_map);
                        let cp_bg = find_nearest_color(t.bg, &be.color_map);
                        let pair = (cp_bg * 16) + cp_fg;
                        window.attrset(pancurses::COLOR_PAIR(pair.try_into()?));
                        window.mvaddch(st.height as i32 - (y as i32 + 1), x as i32, to_char(t.glyph));
                        idx += 1;
                    }
                }
            }
            else if let Some(st) = cons_any.downcast_ref::<SparseConsole>() {
                for t in st.tiles.iter() {
                    let x = t.idx as u32 % st.width;
                    let y = t.idx as u32 / st.width;
                    let cp_fg = find_nearest_color(t.fg, &be.color_map);
                    let cp_bg = find_nearest_color(t.bg, &be.color_map);
                    let pair = (cp_bg * 16) + cp_fg;
                    window.attrset(pancurses::COLOR_PAIR(pair.try_into()?));
                    window.mvaddch(
                        st.height as i32 - (y as i32 + 1),
                        x as i32,
                        to_char(t.glyph),
                    );
                }
            }
        }

        window.refresh();

        crate::hal::fps_sleep(be.frame_sleep_time, &now, prev_ms);
    }

    endwin();
    Ok(())
}
