use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppScreen {
    #[default]
    MainMenu,
    PlayerSetup,
    Running,
    GameOver,
    Options,
}

pub fn restart_game(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppScreen>>,
) {
    if keyboard.just_pressed(KeyCode::KeyR) {
        next_state.set(AppScreen::Running);
    }
}

pub fn game_over_back_to_setup(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppScreen>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(AppScreen::PlayerSetup);
    }
}