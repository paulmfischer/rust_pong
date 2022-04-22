use crate::prelude::*;

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "0".to_string(),
                    style: TextStyle {
                        font_size: SCOREBOARD_FONT_SIZE,
                        color: SCORE_COLOR,
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    },
                },
                TextSection {
                    value: "0".to_string(),
                    style: TextStyle {
                        font_size: SCOREBOARD_FONT_SIZE,
                        color: SCORE_COLOR,
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    },
                },
            ],
            ..default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: SCOREBOARD_TEXT_PADDING,
                left: SCOREBOARD_TEXT_PADDING,
                ..default()
            },
            ..default()
        },
        ..default()
    });
}
