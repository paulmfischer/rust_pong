use crate::prelude::*;
use bevy::math::{const_vec2, const_vec3};

// We set the z-value of the ball to 1 so it renders on top in the case of overlapping sprites.
// const BALL_STARTING_POSITION: Vec3 = const_vec3!([-15.0, 15.0, 1.0]);
const BALL_SIZE: Vec3 = const_vec3!([30.0, 30.0, 0.0]);
const BALL_SPEED: f32 = 400.0;
const INITIAL_BALL_DIRECTION: Vec2 = const_vec2!([0.5, -0.5]);
const BALL_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);

pub fn setup(mut commands: Commands, windows: Res<Windows>) {
    // let window = windows.get_primary().unwrap();
    let ball_size = Vec3::new(30.0, 30.0, 0.0);
    // let x_position = ball_size.to_array()[0] / 2.0 * -1.0;
    let y_position = ball_size.to_array()[1] / 2.0;
    let start_position = Vec3::new(0.0, y_position, 1.0);
    println!("ball start {}", start_position);
    commands
        .spawn()
        .insert(Ball)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                scale: BALL_SIZE,
                translation: start_position, //BALL_STARTING_POSITION,
                ..default()
            },
            sprite: Sprite {
                color: BALL_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Velocity(INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED));
}
