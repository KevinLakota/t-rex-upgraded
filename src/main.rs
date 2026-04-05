mod collision;
mod constants;
mod obstacle;
mod player;
mod score;
mod ui;
mod difficulty;
mod health;
mod app_state;
mod menu;
mod player_profile;
mod scoreboard;
mod player_setup;
mod game_reset;
mod background;
mod settings;

use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowMode};
use bevy_simple_text_input::{TextInputPlugin, TextInputSystem};

use app_state::*;
use collision::*;
use constants::*;
use difficulty::*;
use health::*;
use menu::*;
use obstacle::*;
use player::*;
use player_profile::*;
use player_setup::*;
use score::*;
use ui::*;
use game_reset::*;
use scoreboard::*;
use background::*;
use settings::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "T-Rex Upgraded".to_string(),
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(TextInputPlugin)
        .init_state::<AppScreen>()
        .insert_resource(GameSettings::default())
        .insert_resource(PlayerProfile::default())
        .insert_resource(Scoreboard::load_from_file())
        .insert_resource(Score { distance: 0.0 })
        .insert_resource(ObstacleSpawnTimer {
            timer: 0.0,
            next_spawn_time: OBSTACLE_SPAWN_INTERVAL,
        })
        .insert_resource(Difficulty {
            obstacle_speed: BASE_SPEED,
        })
        .insert_resource(Health::default())
        .insert_resource(Invulnerability::default())
        .add_systems(Startup, (setup, setup_ui))
        .add_systems(OnEnter(AppScreen::MainMenu), (spawn_main_menu, hide_all_game_ui))
        .add_systems(OnExit(AppScreen::MainMenu), cleanup_main_menu)
        .add_systems(OnEnter(AppScreen::Options), (spawn_options, hide_all_game_ui))
        .add_systems(Startup, start_background_music)
        .add_systems(Update, (apply_music_volume, apply_window_mode, update_options_ui))
        .add_systems(OnExit(AppScreen::Options), cleanup_options)
        .add_systems(
            OnEnter(AppScreen::PlayerSetup),
            (spawn_player_setup_screen, hide_all_game_ui),
        )
        .add_systems(OnExit(AppScreen::PlayerSetup), cleanup_player_setup)
        .add_systems(
            OnEnter(AppScreen::Running),
            (reset_game, hide_all_game_ui, show_running_ui),
        )
        .add_systems(
            OnEnter(AppScreen::GameOver),
            (show_game_over_ui, save_score_on_game_over),
        )
        .add_systems(
            Update,
            (
                player_jump,
                player_movement,
                spawn_obstacle,
                move_obstacles,
                animate_player,
                update_score,
                update_difficulty,
                update_invulnerability,
                blink_player,
                check_collision,
                update_score_ui,
                update_lives_ui,
                move_background,
                loop_background,
            )
                .run_if(in_state(AppScreen::Running)),
        )
        .add_systems(
            Update,
            (
                restart_game,
                game_over_back_to_setup,
                game_over_button_system,
            )
                .run_if(in_state(AppScreen::GameOver)),
        )
        .add_systems(
            Update,
            menu_button_system.run_if(in_state(AppScreen::MainMenu)),
        )
        .add_systems(
            Update,
            menu_button_system.run_if(in_state(AppScreen::Options)),
        )
        .add_systems(
            Update,
            (
                player_setup_button_system,
                player_setup_back_to_menu,
                player_setup_submit_system.after(TextInputSystem),
            )
                .run_if(in_state(AppScreen::PlayerSetup)),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    commands.spawn(Camera2d);

    let visible_width = window.width();

    spawn_background(&mut commands, &asset_server, visible_width);
    spawn_ground_visual(&mut commands, visible_width);

    let idle = asset_server.load("idle.png");
    let run1 = asset_server.load("run1.png");
    let run2 = asset_server.load("run2.png");

    commands.spawn((
        Sprite::from_image(idle.clone()),
        Transform::from_xyz(PLAYER_START_X, PLAYER_START_Y, 1.0)
            .with_scale(Vec3::splat(PLAYER_SPRITE_SCALE)),
        Player,
        Velocity { y: 0.0 },
        PlayerAnimation {
            frames: vec![idle.clone(), run1, idle.clone(), run2],
            current_frame: 0,
            timer: Timer::from_seconds(0.12, TimerMode::Repeating),
        },
    ));
}