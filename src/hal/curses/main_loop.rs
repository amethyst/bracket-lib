use pancurses::{initscr, Window, noecho, endwin};
use super::super::super::{Console, GameState, Rltk};
use std::time::Instant;
use super::VirtualKeyCode;

pub fn main_loop<GS: GameState>(mut rltk: Rltk, mut gamestate: GS) {
    let now = Instant::now();
    let mut prev_seconds = now.elapsed().as_secs();
    let mut prev_ms = now.elapsed().as_millis();
    let mut frames = 0;

    let dummy_shader = super::shader::Shader{};

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
        let input = rltk.backend.platform.window.getch();
        if let Some(input) = input {
            //println!("{:?}", input);

            match input {
                pancurses::Input::KeyLeft => rltk.key = Some(VirtualKeyCode::Left),
                pancurses::Input::KeyRight => rltk.key = Some(VirtualKeyCode::Right),
                pancurses::Input::KeyUp => rltk.key = Some(VirtualKeyCode::Up),
                pancurses::Input::KeyDown => rltk.key = Some(VirtualKeyCode::Down),
                pancurses::Input::KeyHome => rltk.key = Some(VirtualKeyCode::Home), 
                pancurses::Input::KeyMouse => {
                    if let Ok(mouse_event) = pancurses::getmouse() {
                        if mouse_event.bstate & pancurses::BUTTON1_CLICKED > 0 {
                            rltk.left_click = true;
                        }
                        //println!("{}, {}", mouse_event.x, mouse_event.y);
                        rltk.mouse_pos = ( mouse_event.x, mouse_event.y);
                        //println!("{:?}", rltk.mouse_pos);
                        //println!("{:?}", rltk.mouse_pos());
                    }
                }               
                pancurses::Input::Character(c) => {
                    match c {
                        '`' => rltk.key = Some(VirtualKeyCode::Grave),
                        '1' => rltk.key = Some(VirtualKeyCode::Key1),
                        '2' => rltk.key = Some(VirtualKeyCode::Key2),
                        '3' => rltk.key = Some(VirtualKeyCode::Key3),
                        '4' => rltk.key = Some(VirtualKeyCode::Key4),
                        '5' => rltk.key = Some(VirtualKeyCode::Key5),
                        '6' => rltk.key = Some(VirtualKeyCode::Key6),
                        '7' => rltk.key = Some(VirtualKeyCode::Key7),
                        '8' => rltk.key = Some(VirtualKeyCode::Key8),
                        '9' => rltk.key = Some(VirtualKeyCode::Key9),
                        '0' => rltk.key = Some(VirtualKeyCode::Key0),
                        'a' => rltk.key = Some(VirtualKeyCode::A),
                        'b' => rltk.key = Some(VirtualKeyCode::B),
                        'c' => rltk.key = Some(VirtualKeyCode::C),
                        'd' => rltk.key = Some(VirtualKeyCode::D),
                        'e' => rltk.key = Some(VirtualKeyCode::E),
                        'f' => rltk.key = Some(VirtualKeyCode::F),
                        'g' => rltk.key = Some(VirtualKeyCode::G),
                        'h' => rltk.key = Some(VirtualKeyCode::H),
                        'i' => rltk.key = Some(VirtualKeyCode::I),
                        'j' => rltk.key = Some(VirtualKeyCode::J),
                        'k' => rltk.key = Some(VirtualKeyCode::K),
                        'l' => rltk.key = Some(VirtualKeyCode::L),
                        'm' => rltk.key = Some(VirtualKeyCode::M),
                        'n' => rltk.key = Some(VirtualKeyCode::N),
                        'o' => rltk.key = Some(VirtualKeyCode::O),
                        'p' => rltk.key = Some(VirtualKeyCode::P),
                        'q' => rltk.key = Some(VirtualKeyCode::Q),
                        'r' => rltk.key = Some(VirtualKeyCode::R),
                        's' => rltk.key = Some(VirtualKeyCode::S),
                        't' => rltk.key = Some(VirtualKeyCode::T),
                        'u' => rltk.key = Some(VirtualKeyCode::U),
                        'v' => rltk.key = Some(VirtualKeyCode::V),
                        'w' => rltk.key = Some(VirtualKeyCode::W),
                        'x' => rltk.key = Some(VirtualKeyCode::X),
                        'y' => rltk.key = Some(VirtualKeyCode::Y),
                        'z' => rltk.key = Some(VirtualKeyCode::Z),
                        '\t' => rltk.key = Some(VirtualKeyCode::Tab),
                        '\n' => rltk.key = Some(VirtualKeyCode::Return),
                        ',' => rltk.key = Some(VirtualKeyCode::Comma),
                        '.' => rltk.key = Some(VirtualKeyCode::Period),
                        '/' => rltk.key = Some(VirtualKeyCode::Slash),
                        '[' => rltk.key = Some(VirtualKeyCode::LBracket),
                        ']' => rltk.key = Some(VirtualKeyCode::RBracket),
                        '\\' => rltk.key = Some(VirtualKeyCode::Backslash),
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        

        gamestate.tick(&mut rltk);

        for cons in &mut rltk.consoles {
            cons.console.rebuild_if_dirty(&rltk.backend);
        }

        rltk.backend.platform.window.clear();

        // Tell each console to draw itself
        for cons in &mut rltk.consoles {
            cons.console.gl_draw(&rltk.fonts[cons.font_index], &dummy_shader, &rltk.backend);
        }

        rltk.backend.platform.window.refresh();

    }

    endwin();
}
