use crate::{BResult, gamestate::{BTerm, GameState}};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use super::BACKEND;

pub fn main_loop<GS: GameState>(mut bterm: BTerm, mut gamestate: GS) -> BResult<()> {

    // We're doing a little dance here to get around lifetime/borrow checking.
    // Removing the context data from BTerm in an atomic swap, so it isn't borrowed after move.
    let wrap = { std::mem::replace(&mut BACKEND.lock().context_wrapper, None) };
    let unwrap = wrap.unwrap();

    let el = unwrap.el;
    let window = unwrap.window;

    el.run(move |event, _, control_flow| {
        if bterm.quitting {
            *control_flow = ControlFlow::Exit;
        }

        match &event {
            _ => {}
        }
    });
}