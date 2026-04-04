use bevy::prelude::*;
use bevy_simple_text_input::{
    TextInput,
    TextInputSettings,
    TextInputSubmitMessage,
    TextInputTextColor,
    TextInputTextFont,
    TextInputValue,
};

use crate::app_state::AppScreen;
use crate::player_profile::PlayerProfile;
use crate::scoreboard::Scoreboard;

#[derive(Component)]
pub struct PlayerSetupUI;

#[derive(Component)]
pub struct StartGameButton;

#[derive(Component)]
pub struct BackToMenuButton;

#[derive(Component)]
pub struct NameInput;

#[derive(Component)]
pub struct ScoreboardText;

pub fn spawn_player_setup(
    commands: &mut Commands,
    player_name: &str,
    scoreboard: &Scoreboard,
) {
    let top_scores = if scoreboard.entries.is_empty() {
        "No scores yet.".to_string()
    } else {
        scoreboard
            .top_entries(5)
            .iter()
            .enumerate()
            .map(|(i, entry)| format!("{}. {} - {}", i + 1, entry.name, entry.score))
            .collect::<Vec<_>>()
            .join("\n")
    };

    commands
        .spawn((
            Node {
                width: percent(100.0),
                height: percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: px(24.0),
                padding: UiRect::all(px(24.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.06, 0.06, 0.10)),
            PlayerSetupUI,
        ))
        .with_children(|root| {
            root.spawn((
                Text::new("Player Setup"),
                TextFont {
                    font_size: 42.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            root.spawn((
                Text::new("Enter your name and press Enter or Play"),
                TextFont {
                    font_size: 22.0,
                    ..default()
                },
                TextColor(Color::srgb(0.85, 0.85, 0.85)),
            ));

            root.spawn((
                Node {
                    width: px(520.0),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: px(12.0),
                    ..default()
                },
            ))
                .with_children(|row| {
                    row.spawn((
                        Text::new("Name:"),
                        TextFont {
                            font_size: 28.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    row.spawn((
                        Node {
                            width: px(340.0),
                            min_height: px(52.0),
                            border: UiRect::all(px(2.0)),
                            padding: UiRect::axes(px(10.0), px(8.0)),
                            ..default()
                        },
                        BorderColor::all(Color::srgb(0.65, 0.65, 0.90)),
                        BackgroundColor(Color::srgb(0.12, 0.12, 0.16)),
                        TextInput,
                        NameInput,
                        TextInputValue(player_name.to_string()),
                        TextInputTextFont(TextFont {
                            font_size: 28.0,
                            ..default()
                        }),
                        TextInputTextColor(TextColor(Color::WHITE)),
                        TextInputSettings {
                            retain_on_submit: true,
                            ..default()
                        },
                    ));
                });

            root.spawn((
                Node {
                    width: px(760.0),
                    flex_direction: FlexDirection::Row,
                    column_gap: px(20.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
            ))
                .with_children(|row| {
                    row.spawn((
                        Node {
                            width: px(280.0),
                            height: px(220.0),
                            border: UiRect::all(px(2.0)),
                            padding: UiRect::all(px(16.0)),
                            flex_direction: FlexDirection::Column,
                            row_gap: px(10.0),
                            ..default()
                        },
                        BorderColor::all(Color::srgb(0.35, 0.35, 0.45)),
                        BackgroundColor(Color::srgb(0.10, 0.10, 0.14)),
                    ))
                        .with_children(|panel| {
                            panel.spawn((
                                Text::new("Character Preview"),
                                TextFont {
                                    font_size: 24.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));

                            panel.spawn((
                                Text::new("Character preview will be added here later."),
                                TextFont {
                                    font_size: 18.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.82, 0.82, 0.82)),
                            ));
                        });

                    row.spawn((
                        Node {
                            width: px(280.0),
                            height: px(220.0),
                            border: UiRect::all(px(2.0)),
                            padding: UiRect::all(px(16.0)),
                            flex_direction: FlexDirection::Column,
                            row_gap: px(10.0),
                            ..default()
                        },
                        BorderColor::all(Color::srgb(0.35, 0.35, 0.45)),
                        BackgroundColor(Color::srgb(0.10, 0.10, 0.14)),
                    ))
                        .with_children(|panel| {
                            panel.spawn((
                                Text::new("Scoreboard"),
                                TextFont {
                                    font_size: 24.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));

                            panel.spawn((
                                Text::new(top_scores),
                                TextFont {
                                    font_size: 18.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.82, 0.82, 0.82)),
                                ScoreboardText,
                            ));
                        });
                });

            root.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: px(16.0),
                    ..default()
                },
            ))
                .with_children(|buttons| {
                    buttons
                        .spawn((
                            Button,
                            Node {
                                width: px(220.0),
                                height: px(65.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.18, 0.18, 0.18)),
                            StartGameButton,
                        ))
                        .with_children(|button| {
                            button.spawn((
                                Text::new("Play"),
                                TextFont {
                                    font_size: 28.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });

                    buttons
                        .spawn((
                            Button,
                            Node {
                                width: px(220.0),
                                height: px(65.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.18, 0.18, 0.18)),
                            BackToMenuButton,
                        ))
                        .with_children(|button| {
                            button.spawn((
                                Text::new("Back to Menu"),
                                TextFont {
                                    font_size: 28.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });
                });
        });
}

fn try_start_game(
    text_input_query: &Query<&TextInputValue, With<NameInput>>,
    player_profile: &mut ResMut<PlayerProfile>,
    next_state: &mut ResMut<NextState<AppScreen>>,
) {
    let Ok(text_input) = text_input_query.single() else {
        return;
    };

    let trimmed = text_input.0.trim();

    if trimmed.is_empty() {
        return;
    }

    player_profile.name = trimmed.to_string();
    next_state.set(AppScreen::Running);
}

pub fn player_setup_button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&StartGameButton>,
            Option<&BackToMenuButton>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    text_input_query: Query<&TextInputValue, With<NameInput>>,
    mut next_state: ResMut<NextState<AppScreen>>,
    mut player_profile: ResMut<PlayerProfile>,
) {
    for (interaction, mut color, start_game, back_button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgb(0.35, 0.35, 0.35));

                if start_game.is_some() {
                    try_start_game(&text_input_query, &mut player_profile, &mut next_state);
                } else if back_button.is_some() {
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

pub fn player_setup_submit_system(
    mut submit_events: MessageReader<TextInputSubmitMessage>,
    text_input_query: Query<&TextInputValue, With<NameInput>>,
    mut next_state: ResMut<NextState<AppScreen>>,
    mut player_profile: ResMut<PlayerProfile>,
) {
    for _event in submit_events.read() {
        try_start_game(&text_input_query, &mut player_profile, &mut next_state);
    }
}

pub fn player_setup_back_to_menu(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppScreen>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(AppScreen::MainMenu);
    }
}

pub fn cleanup_player_setup(
    mut commands: Commands,
    query: Query<Entity, With<PlayerSetupUI>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub fn spawn_player_setup_screen(
    mut commands: Commands,
    player_profile: Res<PlayerProfile>,
    scoreboard: Res<Scoreboard>,
) {
    spawn_player_setup(&mut commands, &player_profile.name, &scoreboard);
}