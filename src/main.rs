use bevy::{core::FixedTimestep, prelude::*};

mod ball;
mod collision;
mod components;
mod move_player;
mod player_one;
mod player_two;
mod scoreboard;
mod velocity;
mod wall;

mod prelude {
    pub use crate::ball::*;
    pub use crate::collision::*;
    pub use crate::components::*;
    pub use crate::move_player::*;
    pub use crate::player_one::*;
    pub use crate::player_two::*;
    pub use crate::scoreboard::*;
    pub use crate::velocity::*;
    pub use crate::wall::*;
    pub use bevy::prelude::*;
}

const BACKGROUND_COLOR: Color = Color::rgb(0.20, 0.20, 0.20);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(prelude::Scoreboard {
            player_one: 0,
            player_two: 0,
        })
        .add_startup_system(setup)
        .add_startup_system(scoreboard::setup)
        .add_startup_system(wall::setup)
        .add_startup_system(ball::setup)
        .add_startup_system(player_one::setup)
        .add_startup_system(player_two::setup)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(move_player::TIME_STEP as f64))
                .with_system(collision::check_for_wall_collisions)
                .with_system(collision::check_for_paddle_collisions)
                .with_system(collision::check_for_goal)
                .with_system(
                    move_player::move_player_one.before(collision::check_for_wall_collisions),
                )
                .with_system(
                    move_player::move_player_two.before(collision::check_for_wall_collisions),
                )
                .with_system(velocity::apply_velocity.before(collision::check_for_wall_collisions)),
        )
        .add_system(scoreboard::update_score)
        .run();
}

fn setup(mut commands: Commands, mut windows: ResMut<Windows>) {
    let window = windows.primary_mut();
    let height = window.height();
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // display center line
    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            scale: Vec3::new(3.0, height, 0.0),
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..default()
        },
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            ..default()
        },
        ..default()
    });

    window.set_title("Pong".to_string());
}
