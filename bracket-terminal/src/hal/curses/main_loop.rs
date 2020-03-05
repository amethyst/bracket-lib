use super::*;
use crate::hal::VirtualKeyCode;
use crate::prelude::{
    to_char, BEvent, BTerm, GameState, SimpleConsole, SparseConsole, BACKEND_INTERNAL,
};
use crate::{clear_input_state, Result};
use pancurses::endwin;
use std::convert::TryInto;
use std::time::Instant;
use std::collections::HashSet;

pub fn main_loop<GS: GameState>(mut bterm: BTerm, mut gamestate: GS) -> Result<()> {
    let now = Instant::now();
    let mut prev_seconds = now.elapsed().as_secs();
    let mut prev_ms = now.elapsed().as_millis();
    let mut frames = 0;

    let mut button_map : HashSet<usize> = HashSet::new();
    let mut key_map : HashSet<char> = HashSet::new();
    let mut keys_this_frame : HashSet<char> = HashSet::new();

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
        clear_input_state(&mut bterm);
        let mut buttons_this_frame = (false, false, false);
        keys_this_frame.clear();
        loop {
            let input = BACKEND.lock().unwrap().window.as_ref().unwrap().getch();
            if let Some(input) = input {
                match input {
                    pancurses::Input::Character(c) => {
                        keys_this_frame.insert(c);
                        if !key_map.contains(&c) {
                            bterm.on_event(BEvent::Character { c });
                            if let Some(key) = char_to_keycode(c) {
                                bterm.on_key(key, virtual_key_code_to_scan(key), true); // How do I get the scancode?
                                key_map.insert(c);
                            }
                        }
                    }
                    pancurses::Input::KeyLeft => bterm.on_key(VirtualKeyCode::Left, 0, true),
                    pancurses::Input::KeyRight => bterm.on_key(VirtualKeyCode::Right, 0, true),
                    pancurses::Input::KeyUp => bterm.on_key(VirtualKeyCode::Up, 0, true),
                    pancurses::Input::KeyDown => bterm.on_key(VirtualKeyCode::Down, 0, true),
                    pancurses::Input::KeyHome => bterm.on_key(VirtualKeyCode::Home, 0, true),
                    pancurses::Input::KeyMouse => {
                        if let Ok(mouse_event) = pancurses::getmouse() {
                            if mouse_event.bstate & pancurses::BUTTON1_CLICKED > 0 {
                                if !button_map.contains(&0) {
                                    bterm.on_mouse_button(0, true);
                                    button_map.insert(0);
                                    buttons_this_frame.0 = true;
                                }
                            }
                            if mouse_event.bstate & pancurses::BUTTON2_CLICKED > 0 {
                                if !button_map.contains(&2) {
                                    bterm.on_mouse_button(2, true);
                                    button_map.insert(2);
                                    buttons_this_frame.2 = true;
                                }
                            }
                            if mouse_event.bstate & pancurses::BUTTON3_CLICKED > 0 {
                                if !button_map.contains(&1) {
                                    bterm.on_mouse_button(1, true);
                                    button_map.insert(1);
                                    buttons_this_frame.1 = true;
                                }
                            }
                            bterm.on_mouse_position(
                                mouse_event.x as f64 * 8.0,
                                mouse_event.y as f64 * 8.0,
                            );
                        }
                    }
                    _ => {
                        println!("{:#?}", input);
                    }
                }
            } else {
                break;
            }
        }

        if !buttons_this_frame.0 && button_map.contains(&0) {
            button_map.remove(&0);
            bterm.on_mouse_button(0, false);
        }
        let keys_released = key_map
            .iter()
            .filter(|k| !keys_this_frame.contains(k))
            .map(|k| *k)
            .collect::<Vec<char>>();
        for key in keys_released {
            key_map.remove(&key);
            if let Some(key) = char_to_keycode(key) {
                bterm.on_key(key, virtual_key_code_to_scan(key), false);
            }
        }

        gamestate.tick(&mut bterm);

        let be = BACKEND.lock().unwrap();
        let window = be.window.as_ref().unwrap();

        window.clear();

        // Tell each console to draw itself
        for cons in &mut BACKEND_INTERNAL.lock().unwrap().consoles {
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
                        window.mvaddch(
                            st.height as i32 - (y as i32 + 1),
                            x as i32,
                            to_char(t.glyph),
                        );
                        idx += 1;
                    }
                }
            } else if let Some(st) = cons_any.downcast_ref::<SparseConsole>() {
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

fn char_to_keycode(c: char) -> Option<VirtualKeyCode> {
    match c {
        '`' => Some(VirtualKeyCode::Grave),
        '1' => Some(VirtualKeyCode::Key1),
        '2' => Some(VirtualKeyCode::Key2),
        '3' => Some(VirtualKeyCode::Key3),
        '4' => Some(VirtualKeyCode::Key4),
        '5' => Some(VirtualKeyCode::Key5),
        '6' => Some(VirtualKeyCode::Key6),
        '7' => Some(VirtualKeyCode::Key7),
        '8' => Some(VirtualKeyCode::Key8),
        '9' => Some(VirtualKeyCode::Key9),
        '0' => Some(VirtualKeyCode::Key0),
        'a' => Some(VirtualKeyCode::A),
        'b' => Some(VirtualKeyCode::B),
        'c' => Some(VirtualKeyCode::C),
        'd' => Some(VirtualKeyCode::D),
        'e' => Some(VirtualKeyCode::E),
        'f' => Some(VirtualKeyCode::F),
        'g' => Some(VirtualKeyCode::G),
        'h' => Some(VirtualKeyCode::H),
        'i' => Some(VirtualKeyCode::I),
        'j' => Some(VirtualKeyCode::J),
        'k' => Some(VirtualKeyCode::K),
        'l' => Some(VirtualKeyCode::L),
        'm' => Some(VirtualKeyCode::M),
        'n' => Some(VirtualKeyCode::N),
        'o' => Some(VirtualKeyCode::O),
        'p' => Some(VirtualKeyCode::P),
        'q' => Some(VirtualKeyCode::Q),
        'r' => Some(VirtualKeyCode::R),
        's' => Some(VirtualKeyCode::S),
        't' => Some(VirtualKeyCode::T),
        'u' => Some(VirtualKeyCode::U),
        'v' => Some(VirtualKeyCode::V),
        'w' => Some(VirtualKeyCode::W),
        'x' => Some(VirtualKeyCode::X),
        'y' => Some(VirtualKeyCode::Y),
        'z' => Some(VirtualKeyCode::Z),
        '\t' => Some(VirtualKeyCode::Tab),
        '\n' => Some(VirtualKeyCode::Return),
        ',' => Some(VirtualKeyCode::Comma),
        '.' => Some(VirtualKeyCode::Period),
        '/' => Some(VirtualKeyCode::Slash),
        '[' => Some(VirtualKeyCode::LBracket),
        ']' => Some(VirtualKeyCode::RBracket),
        '\\' => Some(VirtualKeyCode::Backslash),
        _ => None,
    }
}
