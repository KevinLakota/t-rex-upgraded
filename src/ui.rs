use bevy::prelude::*;
use crate::health::Health;
use crate::score::Score;

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
            display: Display::None,
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
            display: Display::None,
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

pub fn show_running_ui(
    mut query: Query<
        &mut Node,
        Or<(With<ScoreText>, With<LivesText>)>,
    >,
) {
    for mut node in &mut query {
        node.display = Display::Block;
    }
}

pub fn show_game_over_ui(
    mut query: Query<
        &mut Node,
        Or<(With<GameOverText>, With<RestartText>)>,
    >,
) {
    for mut node in &mut query {
        node.display = Display::Block;
    }
}

pub fn hide_all_game_ui(
    mut query: Query<
        &mut Node,
        Or<(With<ScoreText>, With<LivesText>, With<GameOverText>, With<RestartText>)>,
    >,
) {
    for mut node in &mut query {
        node.display = Display::None;
    }
}