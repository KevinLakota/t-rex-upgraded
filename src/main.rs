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

use bevy::prelude::*;
use bevy::window::WindowResolution;

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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "T-Rex Upgraded".to_string(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .init_state::<AppScreen>()
        .insert_resource(PlayerProfile::default())
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
        .add_systems(OnEnter(AppScreen::GameOver), show_game_over_ui)

        .add_systems(
            Update,
            (
                player_jump,
                player_movement,
                spawn_obstacle,
                move_obstacles,
                update_score,
                update_difficulty,
                update_invulnerability,
                blink_player,
                check_collision,
                update_score_ui,
                update_lives_ui,
            )
                .run_if(in_state(AppScreen::Running)),
        )

        .add_systems(
            Update,
            (restart_game, game_over_back_to_setup)
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
                player_name_input,
                update_player_name_text,
            )
                .run_if(in_state(AppScreen::PlayerSetup)),
        )

        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite::from_color(
            Color::srgb(0.3, 0.3, 0.3),
            Vec2::new(WORLD_WIDTH, 4.0),
        ),
        Transform::from_xyz(0.0, GROUND_Y, 0.0),
    ));

    commands.spawn((
        Sprite::from_color(
            Color::srgb(0.2, 0.8, 0.2),
            Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT),
        ),
        Transform::from_xyz(PLAYER_START_X, PLAYER_START_Y, 1.0),
        Player,
        Velocity { y: 0.0 },
    ));
}