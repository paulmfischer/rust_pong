use crate::prelude::*;

#[derive(Component)]
pub struct PlayerOneScore;

#[derive(Component)]
pub struct PlayerTwoScore;

pub struct Scoreboard {
    pub player_one: usize,
    pub player_two: usize,
}


const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());

    // player one score
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(15.0),
                    left: Val::Px(15.0),
                    ..default()
                },
                ..default()
            },
            text: Text {
                sections: vec![TextSection {
                    value: "0".to_string(),
                    style: TextStyle {
                        font_size: SCOREBOARD_FONT_SIZE,
                        color: SCORE_COLOR,
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    },
                }],
                ..default()
            },
            ..default()
        })
        .insert(PlayerOneScore);

    // plyer two score
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(15.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..default()
            },
            text: Text {
                sections: vec![TextSection {
                    value: "0".to_string(),
                    style: TextStyle {
                        font_size: SCOREBOARD_FONT_SIZE,
                        color: SCORE_COLOR,
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    },
                }],
                ..default()
            },
            ..default()
        })
        .insert(PlayerTwoScore);
}

pub fn update_score(
    scoreboard: Res<Scoreboard>,
    mut playerone_query: Query<&mut Text, (With<PlayerOneScore>, Without<PlayerTwoScore>)>,
    mut playertwo_query: Query<&mut Text, (With<PlayerTwoScore>, Without<PlayerOneScore>)>,
) {
    let mut playerone_text = playerone_query.single_mut();
    let mut playertwo_text = playertwo_query.single_mut();

    playerone_text.sections[0].value = format!("{}", scoreboard.player_one);
    playertwo_text.sections[0].value = format!("{}", scoreboard.player_two);
}
