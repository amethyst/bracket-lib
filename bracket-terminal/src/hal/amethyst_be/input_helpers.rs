use std::collections::HashSet;
use crate::prelude::INPUT;
use amethyst::{
    input::{Bindings, Button, InputBundle, InputHandler, StringBindings},
    winit::MouseButton,
};

#[inline(always)]
pub fn button_state_map(inputs: &InputHandler<StringBindings>) -> (Vec<usize>, Vec<usize>) {
    let mut binput = INPUT.lock().unwrap();
    let bterm_pressed_buttons = binput.mouse_button_pressed_set();

    let amethyst_button_state = inputs
        .buttons_that_are_down()
        .map(|b| {
            match b {
                Button::Mouse(MouseButton::Left) => 0,
                Button::Mouse(MouseButton::Right) => 1,
                Button::Mouse(MouseButton::Middle) => 2,
                Button::Mouse(MouseButton::Other(num)) => 3 + num as usize,
                _ => 65536
            }
        })
        .collect::<HashSet<usize>>();

    let newly_released_buttons = bterm_pressed_buttons
        .iter()
        .filter(|b| !amethyst_button_state.contains(b))
        .map(|b| *b)
        .collect::<Vec<usize>>();

    let newly_pressed_buttons = inputs
        .buttons_that_are_down()
        .map(|b| {
            match b {
                Button::Mouse(MouseButton::Left) => 0,
                Button::Mouse(MouseButton::Right) => 1,
                Button::Mouse(MouseButton::Middle) => 2,
                Button::Mouse(MouseButton::Other(num)) => 3 + num as usize,
                _ => 65536
            }
        })
        .filter(|b| *b != 65536 && !bterm_pressed_buttons.contains(b))
        .collect::<Vec<usize>>();

        (newly_pressed_buttons, newly_released_buttons)
}