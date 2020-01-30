use super::super::super::{GameState, Rltk};
use super::VirtualKeyCode;
use std::time::Instant;
use std::io::{stdout, Write};
use crossterm::terminal::{SetSize};
use crossterm::{execute, queue};

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

        gamestate.tick(&mut rltk);

        for cons in &mut rltk.consoles {
            cons.console.rebuild_if_dirty(&rltk.backend);
        }

        queue!(stdout(), crossterm::cursor::Hide).expect("Command fail");

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
}
