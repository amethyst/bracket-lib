use crate::prelude::INPUT;
use amethyst::{
    input::{Button, InputHandler, StringBindings},
    winit::MouseButton,
    winit::VirtualKeyCode,
};
use std::collections::HashSet;

#[inline(always)]
pub(crate) fn button_state_map(inputs: &InputHandler<StringBindings>) -> (Vec<usize>, Vec<usize>) {
    let binput = INPUT.lock().unwrap();
    let bterm_pressed_buttons = binput.mouse_button_pressed_set();

    let amethyst_button_state = inputs
        .buttons_that_are_down()
        .map(|b| match b {
            Button::Mouse(MouseButton::Left) => 0,
            Button::Mouse(MouseButton::Right) => 1,
            Button::Mouse(MouseButton::Middle) => 2,
            Button::Mouse(MouseButton::Other(num)) => 3 + num as usize,
            _ => 65536,
        })
        .collect::<HashSet<usize>>();

    let newly_released_buttons = bterm_pressed_buttons
        .iter()
        .filter(|b| !amethyst_button_state.contains(b))
        .map(|b| *b)
        .collect::<Vec<usize>>();

    let newly_pressed_buttons = inputs
        .buttons_that_are_down()
        .map(|b| match b {
            Button::Mouse(MouseButton::Left) => 0,
            Button::Mouse(MouseButton::Right) => 1,
            Button::Mouse(MouseButton::Middle) => 2,
            Button::Mouse(MouseButton::Other(num)) => 3 + num as usize,
            _ => 65536,
        })
        .filter(|b| *b != 65536 && !bterm_pressed_buttons.contains(b))
        .collect::<Vec<usize>>();

    (newly_pressed_buttons, newly_released_buttons)
}

#[inline(always)]
pub(crate) fn key_state_map(
    inputs: &InputHandler<StringBindings>,
    keys_down: &HashSet<(VirtualKeyCode, u32)>,
) -> (Vec<(VirtualKeyCode, u32)>, Vec<(VirtualKeyCode, u32)>) {
    let binput = INPUT.lock().unwrap();
    let binput_pressed_scan_codes = binput.scan_code_pressed_set();

    let amethyst_keys_down = inputs
        .keys_that_are_down()
        .zip(inputs.scan_codes_that_are_down())
        .collect::<HashSet<(VirtualKeyCode, u32)>>();
    let newly_pressed_keys = amethyst_keys_down
        .iter()
        .filter(|k| !binput_pressed_scan_codes.contains(&k.1))
        .map(|k| *k)
        .collect::<Vec<(VirtualKeyCode, u32)>>();
    let newly_released_keys = keys_down
        .iter()
        .filter(|k| !amethyst_keys_down.contains(&k))
        .map(|k| *k)
        .collect::<Vec<(VirtualKeyCode, u32)>>();

    (newly_pressed_keys, newly_released_keys)
}
