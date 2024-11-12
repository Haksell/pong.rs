use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerScore;

#[derive(Component)]
pub struct AiScore;

#[derive(Resource, Default)]
pub struct Score {
    pub player: u32,
    pub ai: u32,
}

pub enum Scorer {
    Ai,
    Player,
}

#[derive(Event)]
pub struct Scored(pub Scorer);

pub fn update_scoreboard(
    mut player_score: Query<&mut Text, With<PlayerScore>>,
    mut ai_score: Query<&mut Text, (With<AiScore>, Without<PlayerScore>)>,
    score: Res<Score>,
) {
    if score.is_changed() {
        if let Ok(mut player_score) = player_score.get_single_mut() {
            player_score.sections[0].value = score.player.to_string();
        }

        if let Ok(mut ai_score) = ai_score.get_single_mut() {
            ai_score.sections[0].value = score.ai.to_string();
        }
    }
}

pub fn spawn_scoreboard(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "0",
            TextStyle {
                font_size: 72.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            right: Val::Px(15.0),
            ..default()
        }),
        PlayerScore,
    ));

    commands.spawn((
        TextBundle::from_section(
            "0",
            TextStyle {
                font_size: 72.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(15.0),
            ..default()
        }),
        AiScore,
    ));
}
