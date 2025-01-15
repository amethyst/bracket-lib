use bevy::prelude::*;

pub fn player_input(keyboard: &ButtonInput<KeyCode>) -> (i32, i32) {
    if keyboard.just_pressed(KeyCode::ArrowLeft)
        || keyboard.just_pressed(KeyCode::Numpad4)
        || keyboard.just_pressed(KeyCode::KeyH)
    {
        (-1, 0)
    } else if keyboard.just_pressed(KeyCode::ArrowRight)
        || keyboard.just_pressed(KeyCode::Numpad6)
        || keyboard.just_pressed(KeyCode::KeyL)
    {
        (1, 0)
    } else if keyboard.just_pressed(KeyCode::ArrowUp)
        || keyboard.just_pressed(KeyCode::Numpad8)
        || keyboard.just_pressed(KeyCode::KeyK)
    {
        (0, -1)
    } else if keyboard.just_pressed(KeyCode::ArrowDown)
        || keyboard.just_pressed(KeyCode::Numpad2)
        || keyboard.just_pressed(KeyCode::KeyJ)
    {
        (0, 1)
    } else {
        (0, 0)
    }
}
