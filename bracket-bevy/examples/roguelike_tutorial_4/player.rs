use bevy::prelude::*;

pub fn player_input(keyboard: &Input<KeyCode>) -> (i32, i32) {
    if keyboard.just_pressed(KeyCode::Left)
        || keyboard.just_pressed(KeyCode::Numpad4)
        || keyboard.just_pressed(KeyCode::H)
    {
        (-1, 0)
    } else if keyboard.just_pressed(KeyCode::Right)
        || keyboard.just_pressed(KeyCode::Numpad6)
        || keyboard.just_pressed(KeyCode::L)
    {
        (1, 0)
    } else if keyboard.just_pressed(KeyCode::Up)
        || keyboard.just_pressed(KeyCode::Numpad8)
        || keyboard.just_pressed(KeyCode::K)
    {
        (0, -1)
    } else if keyboard.just_pressed(KeyCode::Down)
        || keyboard.just_pressed(KeyCode::Numpad2)
        || keyboard.just_pressed(KeyCode::J)
    {
        (0, 1)
    } else {
        (0, 0)
    }
}
