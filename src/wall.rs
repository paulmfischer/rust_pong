use crate::prelude::*;
// use bevy::math::{const_vec2, const_vec3};

const WALL_THICKNESS: f32 = 10.0;
const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);

pub fn setup(mut commands: Commands, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();

    commands
        .spawn_bundle(WallBundle::new(WallLocation::Left, window))
        .insert(Goal);
    commands.spawn_bundle(WallBundle::new(WallLocation::Top, window));
    commands
        .spawn_bundle(WallBundle::new(WallLocation::Right, window))
        .insert(Goal);
    commands.spawn_bundle(WallBundle::new(WallLocation::Bottom, window));
}

enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

const WALL_OFFSET: f32 = 3.0;
impl WallLocation {
    fn position(&self, window: &Window) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new((window.width() / 2.0 + WALL_OFFSET) * -1.0, 0.),
            WallLocation::Right => Vec2::new(window.width() / 2.0 + WALL_OFFSET, 0.),
            WallLocation::Bottom => Vec2::new(0., (window.height() / 2.0 + WALL_OFFSET) * -1.0),
            WallLocation::Top => Vec2::new(0., window.height() / 2.0 + WALL_OFFSET),
        }
    }

    fn size(&self, window: &Window) -> Vec2 {
        let arena_height = window.height();
        let arena_width = window.width();
        // Make sure we haven't messed up our constants
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left => Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS),
            WallLocation::Right => Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS),
            WallLocation::Bottom => Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS),
            WallLocation::Top => Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS),
        }
    }
}

#[derive(Bundle)]
struct WallBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    wall: Wall,
}

impl WallBundle {
    // This "builder method" allows us to reuse logic across our wall entities,
    // making our code easier to read and less prone to bugs when we change the logic
    fn new(location: WallLocation, window: &Window) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
                    // This is used to determine the order of our sprites
                    translation: location.position(window).extend(0.0),
                    // The z-scale of 2D objects must always be 1.0,
                    // or their ordering will be affected in surprising ways.
                    // See https://github.com/bevyengine/bevy/issues/4149
                    scale: location.size(window).extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            wall: Wall,
        }
    }
}
