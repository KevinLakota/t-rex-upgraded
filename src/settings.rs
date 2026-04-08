use bevy::audio::{PlaybackSettings, Volume};
use bevy::prelude::*;
use bevy::window::{MonitorSelection, PrimaryWindow, WindowMode};

use crate::menu::{DisplayValueText, VolumeValueText};

type VolumeTextQuery<'w, 's> = Query<'w, 's, &'static mut Text, With<VolumeValueText>>;
type DisplayTextQuery<'w, 's> = Query<'w, 's, &'static mut Text, With<DisplayValueText>>;

#[derive(Resource)]
pub struct GameSettings {
    pub music_volume: f32,
    pub fullscreen: bool,
}

#[derive(Component)]
pub struct BackgroundMusic;

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            music_volume: 0.35,
            fullscreen: true,
        }
    }
}

impl GameSettings {
    pub fn increase_volume(&mut self) {
        self.music_volume = (self.music_volume + 0.05).clamp(0.0, 1.0);
    }

    pub fn decrease_volume(&mut self) {
        self.music_volume = (self.music_volume - 0.05).clamp(0.0, 1.0);
    }

    pub fn volume_percent(&self) -> i32 {
        (self.music_volume * 100.0).round() as i32
    }
}

pub fn start_background_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<GameSettings>,
) {
    let music = asset_server.load("music/song1.ogg");

    commands.spawn((
        AudioPlayer::new(music),
        PlaybackSettings::LOOP.with_volume(Volume::Linear(settings.music_volume)),
        BackgroundMusic,
    ));
}

pub fn apply_music_volume(
    settings: Res<GameSettings>,
    mut query: Query<&mut AudioSink, With<BackgroundMusic>>,
) {
    if !settings.is_changed() {
        return;
    }

    let Ok(mut sink) = query.single_mut() else {
        return;
    };

    sink.set_volume(Volume::Linear(settings.music_volume));
}

pub fn apply_window_mode(
    settings: Res<GameSettings>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    if !settings.is_changed() {
        return;
    }

    let Ok(mut window) = window_query.single_mut() else {
        return;
    };

    window.mode = if settings.fullscreen {
        WindowMode::BorderlessFullscreen(MonitorSelection::Primary)
    } else {
        WindowMode::Windowed
    };
}

pub fn update_options_ui(
    settings: Res<GameSettings>,
    mut queries: ParamSet<(VolumeTextQuery, DisplayTextQuery)>,
) {
    if !settings.is_changed() {
        return;
    }

    if let Ok(mut text) = queries.p0().single_mut() {
        **text = format!("Music Volume: {}%", settings.volume_percent());
    }

    if let Ok(mut text) = queries.p1().single_mut() {
        **text = format!(
            "Display Mode: {}",
            if settings.fullscreen {
                "Fullscreen"
            } else {
                "Window"
            }
        );
    }
}