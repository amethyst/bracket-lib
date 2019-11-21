use pancurses::{initscr, Window, noecho, endwin};
use super::super::super::{Console, GameState, Rltk};
use std::time::Instant;

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
        rltk.backend.platform.window.getch();
    }

    endwin();
}
