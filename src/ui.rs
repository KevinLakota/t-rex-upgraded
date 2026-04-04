use bevy::prelude::*;
use crate::game_state::GameState;
use crate::score::Score;
use crate::health::Health;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct LivesText;

#[derive(Component)]
pub struct GameOverText;

#[derive(Component)]
pub struct RestartText;

pub fn setup_ui(mut commands: Commands) {
    commands.spawn((
        Text::new("Score: 0"),
        TextFont {
            font_size: 32.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: px(15.0),
            right: px(20.0),
            ..default()
        },
        ScoreText,
    ));

    commands.spawn((
        Text::new("♥♥♥"),
        TextFont {
            font_size: 32.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.2, 0.2)),
        Node {
            position_type: PositionType::Absolute,
            top: px(15.0),
            left: px(20.0),
            ..default()
        },
        LivesText,
    ));

    commands.spawn((
        Text::new("GAME OVER"),
        TextFont {
            font_size: 56.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: percent(35.0),
            left: percent(35.0),
            display: Display::None,
            ..default()
        },
        GameOverText,
    ));

    commands.spawn((
        Text::new("Press R or Space to restart"),
        TextFont {
            font_size: 28.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: percent(50.0),
            left: percent(30.0),
            display: Display::None,
            ..default()
        },
        RestartText,
    ));
}

pub fn update_score_ui(
    score: Res<Score>,
    mut query: Query<&mut Text, With<ScoreText>>,
) {
    let Ok(mut text) = query.single_mut() else {
        return;
    };

    **text = format!("Score: {}", score.distance as i32);
}

pub fn update_game_over_ui(
    game_state: Res<GameState>,
    mut game_over_query: Query<&mut Node, (With<GameOverText>, Without<RestartText>)>,
    mut restart_query: Query<&mut Node, (With<RestartText>, Without<GameOverText>)>,
) {
    let game_over_display = if *game_state == GameState::GameOver {
        Display::Flex
    } else {
        Display::None
    };

    if let Ok(mut node) = game_over_query.single_mut() {
        node.display = game_over_display;
    }

    if let Ok(mut node) = restart_query.single_mut() {
        node.display = game_over_display;
    }
}

pub fn update_lives_ui(
    health: Res<Health>,
    mut query: Query<&mut Text, With<LivesText>>,
) {
    let Ok(mut text) = query.single_mut() else {
        return;
    };

    let hearts = "♥".repeat(health.current as usize);
    let empty = "♡".repeat((health.max - health.current) as usize);

    **text = format!("{}{}", hearts, empty);
}