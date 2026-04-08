use bevy::app::AppExit;
use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;

use crate::app_state::AppScreen;
use crate::settings::GameSettings;

type MenuButtonQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static Interaction,
        &'static mut BackgroundColor,
        Option<&'static PlayButton>,
        Option<&'static OptionsButton>,
        Option<&'static ExitButton>,
        Option<&'static BackToMenuButton>,
        Option<&'static MusicDownButton>,
        Option<&'static MusicUpButton>,
        Option<&'static ToggleDisplayButton>,
    ),
    (Changed<Interaction>, With<Button>),
>;

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

#[derive(Component)]
pub struct MusicDownButton;

#[derive(Component)]
pub struct MusicUpButton;

#[derive(Component)]
pub struct ToggleDisplayButton;

#[derive(Component)]
pub struct VolumeValueText;

#[derive(Component)]
pub struct DisplayValueText;

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

pub fn spawn_options(
    mut commands: Commands,
    settings: Res<GameSettings>,
) {
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
                Text::new(format!("Music Volume: {}%", settings.volume_percent())),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.85, 0.85, 0.85)),
                VolumeValueText,
            ));

            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: px(16.0),
                        ..default()
                    },
                ))
                .with_children(|row| {
                    spawn_menu_button(row, "Music -", MusicDownButton);
                    spawn_menu_button(row, "Music +", MusicUpButton);
                });

            parent.spawn((
                Text::new(format!(
                    "Display Mode: {}",
                    if settings.fullscreen {
                        "Fullscreen"
                    } else {
                        "Window"
                    }
                )),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.85, 0.85, 0.85)),
                DisplayValueText,
            ));

            spawn_menu_button(parent, "Toggle Fullscreen", ToggleDisplayButton);
            spawn_menu_button(parent, "Back to Menu", BackToMenuButton);
        });
}

pub fn menu_button_system(
    mut interaction_query: MenuButtonQuery,
    mut next_state: ResMut<NextState<AppScreen>>,
    mut exit: MessageWriter<AppExit>,
    mut settings: ResMut<GameSettings>,
) {
    for (
        interaction,
        mut color,
        play,
        options,
        exit_button,
        back_button,
        music_down,
        music_up,
        toggle_display,
    ) in &mut interaction_query
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
                } else if music_down.is_some() {
                    settings.decrease_volume();
                } else if music_up.is_some() {
                    settings.increase_volume();
                } else if toggle_display.is_some() {
                    settings.fullscreen = !settings.fullscreen;
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