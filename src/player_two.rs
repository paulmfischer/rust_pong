use crate::prelude::*;
use bevy::math::const_vec3;

// These constants are defined in `Transform` units.
// Using the default 2D camera they correspond 1:1 with screen pixels.
// The `const_vec3!` macros are needed as functions that operate on floats cannot be constant in Rust.
const PADDLE_SIZE: Vec3 = const_vec3!([20.0, 120.0, 0.0]);
const GAP_BETWEEN_PADDLE_AND_WALL: f32 = 60.0;
// const PADDLE_SPEED: f32 = 500.0;
// How close can the paddle get to the wall
// const PADDLE_PADDING: f32 = 10.0;
// const PADDLE_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);
const PADDLE_COLOR: Color = Color::rgb(0.0, 1.0, 0.0);

pub fn setup(mut commands: Commands, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    println!(
        "window size player two: {} {}",
        window.width(),
        window.height()
    );
    let paddle_x = window.width() / 2.0 - GAP_BETWEEN_PADDLE_AND_WALL;
    commands
        .spawn()
        .insert(Paddle)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(paddle_x, 0.0, 1.0),
                scale: PADDLE_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: PADDLE_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(PlayerTwo)
        .insert(Collider);
}
