mod collision;
mod constants;
mod game_state;
mod obstacle;
mod player;
mod score;
mod ui;
mod difficulty;
mod health;

use bevy::prelude::*;
use bevy::window::WindowResolution;

use collision::*;
use constants::*;
use game_state::*;
use obstacle::*;
use player::*;
use score::*;
use ui::*;
use difficulty::*;
use health::*;

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
        .insert_resource(GameState::Running)
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
                restart_game,
                check_collision.after(restart_game),
                update_score_ui,
                update_game_over_ui,
                update_lives_ui,
            ),
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