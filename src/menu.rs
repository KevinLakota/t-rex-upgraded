use bevy::app::AppExit;
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;

use crate::app_state::AppScreen;

#[derive(Component)]
pub struct MainMenuUI;

#[derive(Component)]
pub struct OptionsUI;

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct OptionsButton;

#[derive(Component)]
pub struct ExitButton;

#[derive(Component)]
pub struct BackToMenuButton;

fn spawn_menu_button<T: Component>(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    label: &str,
    marker: T,
) {
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
            marker,
        ))
        .with_children(|button| {
            button.spawn((
                Text::new(label),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

pub fn spawn_main_menu(mut commands: Commands) {
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
            BackgroundColor(Color::srgb(0.08, 0.08, 0.08)),
            MainMenuUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("T-Rex Upgraded"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            spawn_menu_button(parent, "Play", PlayButton);
            spawn_menu_button(parent, "Options", OptionsButton);
            spawn_menu_button(parent, "Exit", ExitButton);
        });
}

pub fn spawn_options(mut commands: Commands) {
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
            BackgroundColor(Color::srgb(0.10, 0.06, 0.06)),
            OptionsUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Options"),
                TextFont {
                    font_size: 42.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            parent.spawn((
                Text::new("Sem neskor pojde hudba a SFX."),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.85, 0.85, 0.85)),
            ));

            spawn_menu_button(parent, "Back to Menu", BackToMenuButton);
        });
}

pub fn menu_button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&PlayButton>,
            Option<&OptionsButton>,
            Option<&ExitButton>,
            Option<&BackToMenuButton>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<AppScreen>>,
    mut exit: MessageWriter<AppExit>,
) {
    for (interaction, mut color, play, options, exit_button, back_button) in &mut interaction_query
    {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgb(0.35, 0.35, 0.35));

                if play.is_some() {
                    next_state.set(AppScreen::PlayerSetup);
                } else if options.is_some() {
                    next_state.set(AppScreen::Options);
                } else if exit_button.is_some() {
                    exit.write(AppExit::Success);
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

pub fn cleanup_main_menu(
    mut commands: Commands,
    query: Query<Entity, With<MainMenuUI>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub fn cleanup_options(
    mut commands: Commands,
    query: Query<Entity, With<OptionsUI>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}