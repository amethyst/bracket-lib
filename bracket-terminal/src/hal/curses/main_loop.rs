use super::char_to_keycode;
use super::*;
use crate::hal::VirtualKeyCode;
use crate::prelude::{
    to_char, BEvent, BTerm, GameState, SimpleConsole, SparseConsole, BACKEND_INTERNAL, RGBA,
};
use crate::{clear_input_state, Result};
use pancurses::endwin;
use std::collections::HashSet;
use std::convert::TryInto;
use std::time::Instant;

pub fn main_loop<GS: GameState>(mut bterm: BTerm, mut gamestate: GS) -> Result<()> {
    let now = Instant::now();
    let mut prev_seconds = now.elapsed().as_secs();
    let mut prev_ms = now.elapsed().as_millis();
    let mut frames = 0;

    let mut button_map: HashSet<usize> = HashSet::new();
    let mut key_map: HashSet<char> = HashSet::new();
    let mut keys_this_frame: HashSet<char> = HashSet::new();

    let mut output_buffer: Option<Vec<OutputBuffer>> = None;

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
            let input = BACKEND.lock().window.as_ref().unwrap().getch();
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
                            bterm.on_mouse_position(mouse_event.x as f64, mouse_event.y as f64);
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

        if output_buffer.is_none() {
            output_buffer = Some(full_redraw()?);
        } else {
            partial_redraw(output_buffer.as_mut().unwrap());
        }

        crate::hal::fps_sleep(BACKEND.lock().frame_sleep_time, &now, prev_ms);
    }

    endwin();
    Ok(())
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

// Completely redraws the back-end
fn full_redraw() -> Result<Vec<OutputBuffer>> {
    let be = BACKEND.lock();
    let window = be.window.as_ref().unwrap();

    let height = window.get_max_y() as usize;
    let width = window.get_max_x() as usize;
    let mut buffer = vec![OutputBuffer::default(); height * width];

    window.clear();

    // Tell each console to draw itself
    for cons in &mut BACKEND_INTERNAL.lock().consoles {
        let cons_any = cons.console.as_any();
        if let Some(st) = cons_any.downcast_ref::<SimpleConsole>() {
            let mut last_bg = RGBA::new();
            let mut last_fg = RGBA::new();
            let mut cp_fg = 0;
            let mut cp_bg = 0;
            let mut idx = 0;
            for y in 0..st.height {
                for x in 0..st.width {
                    let t = &st.tiles[idx];
                    if t.fg != last_fg {
                        cp_fg = find_nearest_color(t.fg, &be.color_map);
                        last_fg = t.fg;
                    }
                    if t.bg != last_bg {
                        cp_bg = find_nearest_color(t.bg, &be.color_map);
                        last_bg = t.bg;
                    }
                    let pair = (cp_bg * 16) + cp_fg;
                    window.attrset(pancurses::COLOR_PAIR(pair.try_into()?));

                    let ch = to_char(t.glyph as u8);
                    let ty = st.height as i32 - (y as i32 + 1);
                    window.mvaddch(ty, x as i32, ch);
                    let buf_idx = (ty as usize * height) + x as usize;
                    buffer[buf_idx].glyph = ch;
                    buffer[buf_idx].fg = t.fg;
                    buffer[buf_idx].bg = t.bg;
                    idx += 1;
                }
            }
        } else if let Some(st) = cons_any.downcast_ref::<SparseConsole>() {
            let mut last_bg = RGBA::new();
            let mut last_fg = RGBA::new();
            let mut cp_fg = 0;
            let mut cp_bg = 0;
            for t in st.tiles.iter() {
                let x = t.idx as u32 % st.width;
                let y = t.idx as u32 / st.width;
                if t.fg != last_fg {
                    cp_fg = find_nearest_color(t.fg, &be.color_map);
                    last_fg = t.fg;
                }
                if t.bg != last_bg {
                    cp_bg = find_nearest_color(t.bg, &be.color_map);
                    last_bg = t.bg;
                }
                let pair = (cp_bg * 16) + cp_fg;
                window.attrset(pancurses::COLOR_PAIR(pair.try_into()?));
                let ch = to_char(t.glyph as u8);
                let ty = st.height as i32 - (y as i32 + 1);
                window.mvaddch(ty, x as i32, ch);
                let buf_idx = (ty as usize * height) + x as usize;
                buffer[buf_idx].glyph = ch;
                buffer[buf_idx].fg = t.fg;
                buffer[buf_idx].bg = t.bg;
            }
        }
    }

    window.refresh();
    Ok(buffer)
}

fn partial_redraw(buf: &mut Vec<OutputBuffer>) {
    let be = BACKEND.lock();
    let window = be.window.as_ref().unwrap();

    let height = window.get_max_y() as usize;
    let width = window.get_max_x() as usize;

    let mut dirty = Vec::new();

    // Iterate all consoles, rendering to the buffer and denoting dirty
    for cons in &mut BACKEND_INTERNAL.lock().consoles {
        let cons_any = cons.console.as_any();
        if let Some(st) = cons_any.downcast_ref::<SimpleConsole>() {
            let mut idx = 0;
            for y in 0..st.height {
                for x in 0..st.width {
                    let t = &st.tiles[idx];

                    let new_output = OutputBuffer {
                        glyph: to_char(t.glyph as u8),
                        fg: t.fg,
                        bg: t.bg,
                    };
                    let ty = st.height as i32 - (y as i32 + 1);
                    let buf_idx = (ty as usize * width) + x as usize;
                    if buf[buf_idx] != new_output {
                        buf[buf_idx] = new_output;
                        dirty.push(buf_idx);
                    }
                    idx += 1;
                }
            }
        } else if let Some(st) = cons_any.downcast_ref::<SparseConsole>() {
            for t in st.tiles.iter() {
                let x = t.idx as u32 % st.width;
                let y = t.idx as u32 / st.width;
                let ty = st.height as i32 - (y as i32 + 1);
                let buf_idx = (ty as usize * width) + x as usize;
                let new_output = OutputBuffer {
                    glyph: to_char(t.glyph as u8),
                    fg: t.fg,
                    bg: t.bg,
                };
                if buf[buf_idx] != new_output {
                    buf[buf_idx] = new_output;
                    dirty.push(buf_idx);
                }
            }
        }
    }

    let mut last_bg = RGBA::new();
    let mut last_fg = RGBA::new();
    let mut cp_fg = 0;
    let mut cp_bg = 0;
    dirty.iter().for_each(|idx| {
        let x = idx % width;
        let y = idx / width;
        let t = &buf[*idx];

        if t.fg != last_fg {
            cp_fg = find_nearest_color(t.fg, &be.color_map);
            last_fg = t.fg;
        }
        if t.bg != last_bg {
            cp_bg = find_nearest_color(t.bg, &be.color_map);
            last_bg = t.bg;
        }
        let pair = (cp_bg * 16) + cp_fg;
        window.attrset(pancurses::COLOR_PAIR(pair.try_into().unwrap()));

        window.mvaddch(y as i32, x as i32, buf[*idx].glyph);
    });

    window.refresh();
}
