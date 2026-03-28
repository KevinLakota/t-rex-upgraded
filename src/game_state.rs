use bevy::prelude::*;
use crate::constants::*;
use crate::obstacle::{Obstacle, ObstacleSpawnTimer};
use crate::player::{Player, Velocity};
use crate::score::Score;

#[derive(Resource, PartialEq, Eq, Clone, Copy)]
pub enum GameState {
    Running,
    GameOver,
}

pub fn restart_game(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut commands: Commands,
    obstacles: Query<Entity, With<Obstacle>>,
    mut player_query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    mut score: ResMut<Score>,
    mut spawn_timer: ResMut<ObstacleSpawnTimer>,
) {
    if *game_state != GameState::GameOver {
        return;
    }

    if keyboard.just_pressed(KeyCode::KeyR) || keyboard.just_pressed(KeyCode::Space) {
        *game_state = GameState::Running;
        score.distance = 0.0;
        spawn_timer.timer = 0.0;
        spawn_timer.next_spawn_time = OBSTACLE_SPAWN_INTERVAL;

        for entity in &obstacles {
            commands.entity(entity).despawn();
        }

        let Ok((mut transform, mut velocity)) = player_query.single_mut() else {
            return;
        };

        transform.translation.x = PLAYER_START_X;
        transform.translation.y = PLAYER_START_Y;
        velocity.y = 0.0;

        println!("RESTART");
    }
}