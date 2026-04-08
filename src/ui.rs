use bevy::prelude::*;
use crate::app_state::AppScreen;
use crate::health::Health;
use crate::score::Score;
use crate::constants::MAX_HEALTH;

type ScoreQuery<'w, 's> = Query<'w, 's, &'static mut Node, With<ScoreText>>;
type LivesQuery<'w, 's> = Query<'w, 's, &'static mut Node, With<LivesText>>;
type GameOverQuery<'w, 's> = Query<'w, 's, &'static mut Node, With<GameOverUI>>;
type GameOverButtonQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static Interaction,
        &'static mut BackgroundColor,
        Option<&'static RestartButton>,
        Option<&'static GameOverMenuButton>,
    ),
    (Changed<Interaction>, With<Button>),
>;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct LivesText;

#[derive(Component)]
pub struct GameOverUI;

#[derive(Component)]
pub struct GameOverText;

#[derive(Component)]
pub struct RestartText;

#[derive(Component)]
pub struct RestartButton;

#[derive(Component)]
pub struct GameOverMenuButton;

#[derive(Component)]
pub struct HeartUI {
    pub index: usize,
}

pub fn setup_ui(mut commands: Commands,
                asset_server: Res<AssetServer>,) {
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

    let full = asset_server.load("heart.png");

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: px(12.0),
                left: px(20.0),
                display: Display::None,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                column_gap: px(10.0),
                ..default()
            },
            LivesText,
        ))
        .with_children(|parent| {
            for i in 0..MAX_HEALTH {
                parent.spawn((
                    ImageNode::new(full.clone()),
                    Node {
                        width: px(64.0),
                        height: px(64.0),
                        ..default()
                    },
                    HeartUI { index: i as usize },
                ));
            }
        });

    commands
        .spawn((
            Node {
                width: percent(100.0),
                height: percent(100.0),
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: px(18.0),
                display: Display::None,
                ..default()
            },
            GameOverUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("GAME OVER"),
                TextFont {
                    font_size: 64.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                GameOverText,
            ));

            parent.spawn((
                Text::new("Press R or Space to restart"),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                RestartText,
            ));

            spawn_game_over_button(parent, "Restart", RestartButton);
            spawn_game_over_button(parent, "Main Menu", GameOverMenuButton);
        });
}

fn spawn_game_over_button<T: Component>(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    marker: T,
) {
    parent
        .spawn((
            Button,
            Node {
                width: px(220.0),
                height: px(60.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.18, 0.18, 0.18)),
            marker,
        ))
        .with_children(|button| {
            button.spawn((
                Text::new(label),
                TextFont {
                    font_size: 26.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
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
    asset_server: Res<AssetServer>,
    mut query: Query<(&HeartUI, &mut ImageNode)>,
) {
    let full = asset_server.load("heart.png");
    let empty = asset_server.load("empty.png");

    for (heart, mut image) in &mut query {
        if heart.index < health.current as usize {
            image.image = full.clone();
        } else {
            image.image = empty.clone();
        }
    }
}

pub fn show_running_ui(
    mut queries: ParamSet<(ScoreQuery, LivesQuery)>,
) {
    if let Ok(mut node) = queries.p0().single_mut() {
        node.display = Display::Block;
    }

    if let Ok(mut node) = queries.p1().single_mut() {
        node.display = Display::Flex;
    }
}

pub fn show_game_over_ui(
    mut query: Query<&mut Node, With<GameOverUI>>,
) {
    let Ok(mut node) = query.single_mut() else {
        return;
    };

    node.display = Display::Flex;
}

pub fn hide_all_game_ui(
    mut queries: ParamSet<(ScoreQuery, LivesQuery, GameOverQuery)>,
) {
    if let Ok(mut node) = queries.p0().single_mut() {
        node.display = Display::None;
    }

    if let Ok(mut node) = queries.p1().single_mut() {
        node.display = Display::None;
    }

    if let Ok(mut node) = queries.p2().single_mut() {
        node.display = Display::None;
    }
}
pub fn game_over_button_system(
    mut interaction_query: GameOverButtonQuery,
    mut next_state: ResMut<NextState<AppScreen>>,
) {
    for (interaction, mut color, restart, menu) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgb(0.35, 0.35, 0.35));

                if restart.is_some() {
                    next_state.set(AppScreen::Running);
                } else if menu.is_some() {
                    next_state.set(AppScreen::MainMenu);
                }
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.28, 0.28, 0.28));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgb(0.18, 0.18, 0.18));
            }
        }
    }
}