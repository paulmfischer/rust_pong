use crate::prelude::*;

// Defines the amount of time that should elapse between each physics step.
pub const TIME_STEP: f32 = 1.0 / 60.0;

const PADDLE_SPEED: f32 = 500.0;

pub fn move_player_one(
    keyboard_input: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    mut query: Query<&mut Transform, With<PlayerOne>>,
) {
    let window = windows.get_primary().unwrap();
    let height = window.height();
    let mut player_transform = query.single_mut();
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::S) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::W) {
        direction += 1.0;
    }

    let new_position = player_transform.translation.y + direction * PADDLE_SPEED * TIME_STEP;

    let top_bound = height / 2.0 - player_transform.scale.y / 2.0 - 10.0;
    let bottom_bound = height / 2.0 * -1.0 + player_transform.scale.y / 2.0 + 10.0;

    player_transform.translation.y = new_position.clamp(bottom_bound, top_bound);
}

pub fn move_player_two(
    keyboard_input: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    mut query: Query<&mut Transform, With<PlayerTwo>>,
) {
    let window = windows.get_primary().unwrap();
    let height = window.height();
    let mut player_transform = query.single_mut();
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::Down) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        direction += 1.0;
    }

    let new_position = player_transform.translation.y + direction * PADDLE_SPEED * TIME_STEP;

    let top_bound = height / 2.0 - player_transform.scale.y / 2.0;
    let bottom_bound = height / 2.0 * -1.0 + player_transform.scale.y / 2.0;

    player_transform.translation.y = new_position.clamp(bottom_bound, top_bound);
}
