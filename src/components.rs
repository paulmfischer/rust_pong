use bevy::prelude::*;

#[derive(Component)]
pub struct Ball;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerOne;

#[derive(Component)]
pub struct PlayerTwo;

#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
pub struct Collider;

#[derive(Default)]
pub struct CollisionEvent;

#[derive(Component)]
pub struct Goal;

pub struct Scoreboard {
    player_one: usize,
    player_two: usize,
}
