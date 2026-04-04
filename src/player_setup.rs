use bevy::prelude::*;

use crate::app_state::AppScreen;
use crate::player_profile::PlayerProfile;

#[derive(Component)]
pub struct PlayerSetupUI;

#[derive(Component)]
pub struct StartGameButton;

#[derive(Component)]
pub struct BackToMenuButton;

#[derive(Component)]
pub struct NameDisplayText;

pub fn spawn_player_setup(commands: &mut Commands, player_name: &str) {
    commands
        .spawn((
            Node {
                width: percent(100.0),
                height: percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.06, 0.06, 0.10)),
            PlayerSetupUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Player Setup"),
                TextFont {
                    font_size: 42.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            parent.spawn((
                Text::new("Tu bude zadanie mena"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.85, 0.85, 0.85)),
            ));

            parent.spawn((
                Text::new(format!(
                    "Name: {}",
                    if player_name.is_empty() {
                        "_"
                    } else {
                        player_name
                    }
                )),
                TextFont {
                    font_size: 30.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 0.8)),
                NameDisplayText,
            ));

            parent
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

            parent
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
    mut next_state: ResMut<NextState<AppScreen>>,
    player_profile: Res<PlayerProfile>,
) {
    for (interaction, mut color, start_game, back_button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgb(0.35, 0.35, 0.35));

                if start_game.is_some() {
                    if !player_profile.name.trim().is_empty() {
                        next_state.set(AppScreen::Running);
                    }
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

pub fn player_setup_back_to_menu(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppScreen>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(AppScreen::MainMenu);
    }
}

pub fn update_player_name_text(
    player_profile: Res<PlayerProfile>,
    mut query: Query<&mut Text, With<NameDisplayText>>,
) {
    if !player_profile.is_changed() {
        return;
    }

    let Ok(mut text) = query.single_mut() else {
        return;
    };

    let shown_name = if player_profile.name.is_empty() {
        "_".to_string()
    } else {
        player_profile.name.clone()
    };

    **text = format!("Name: {}", shown_name);
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
) {
    spawn_player_setup(&mut commands, &player_profile.name);
}

pub fn player_name_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_profile: ResMut<PlayerProfile>,
) {
    let keys = [
        (KeyCode::KeyA, 'a'),
        (KeyCode::KeyB, 'b'),
        (KeyCode::KeyC, 'c'),
        (KeyCode::KeyD, 'd'),
        (KeyCode::KeyE, 'e'),
        (KeyCode::KeyF, 'f'),
        (KeyCode::KeyG, 'g'),
        (KeyCode::KeyH, 'h'),
        (KeyCode::KeyI, 'i'),
        (KeyCode::KeyJ, 'j'),
        (KeyCode::KeyK, 'k'),
        (KeyCode::KeyL, 'l'),
        (KeyCode::KeyM, 'm'),
        (KeyCode::KeyN, 'n'),
        (KeyCode::KeyO, 'o'),
        (KeyCode::KeyP, 'p'),
        (KeyCode::KeyQ, 'q'),
        (KeyCode::KeyR, 'r'),
        (KeyCode::KeyS, 's'),
        (KeyCode::KeyT, 't'),
        (KeyCode::KeyU, 'u'),
        (KeyCode::KeyV, 'v'),
        (KeyCode::KeyW, 'w'),
        (KeyCode::KeyX, 'x'),
        (KeyCode::KeyY, 'y'),
        (KeyCode::KeyZ, 'z'),
        (KeyCode::Digit0, '0'),
        (KeyCode::Digit1, '1'),
        (KeyCode::Digit2, '2'),
        (KeyCode::Digit3, '3'),
        (KeyCode::Digit4, '4'),
        (KeyCode::Digit5, '5'),
        (KeyCode::Digit6, '6'),
        (KeyCode::Digit7, '7'),
        (KeyCode::Digit8, '8'),
        (KeyCode::Digit9, '9'),
        (KeyCode::Space, ' '),
    ];

    for (key, ch) in keys {
        if keyboard.just_pressed(key) && player_profile.name.len() < 16 {
            player_profile.name.push(ch);
        }
    }

    if keyboard.just_pressed(KeyCode::Backspace) {
        player_profile.name.pop();
    }
}