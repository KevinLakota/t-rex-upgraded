use bevy::prelude::*;

use crate::health::{Health, Invulnerability};
use crate::obstacle::{Obstacle, ObstacleSpawnTimer};
use crate::player::{Player, Velocity};
use crate::score::Score;
use crate::constants::{PLAYER_START_X, PLAYER_START_Y};

fn reset_player_state(
    player_query: &mut Query<(&mut Transform, &mut Velocity, &mut Visibility), With<Player>>,
) {
    let Ok((mut transform, mut velocity, mut visibility)) = player_query.single_mut() else {
        return;
    };

    transform.translation.x = PLAYER_START_X;
    transform.translation.y = PLAYER_START_Y;
    velocity.y = 0.0;
    *visibility = Visibility::Visible;
}

fn clear_all_obstacles(
    commands: &mut Commands,
    obstacles: &Query<Entity, With<Obstacle>>,
) {
    for entity in obstacles {
        commands.entity(entity).despawn();
    }
}

pub fn reset_game(
    mut commands: Commands,
    obstacles: Query<Entity, With<Obstacle>>,
    mut player_query: Query<(&mut Transform, &mut Velocity, &mut Visibility), With<Player>>,
    mut score: ResMut<Score>,
    mut spawn_timer: ResMut<ObstacleSpawnTimer>,
    mut health: ResMut<Health>,
    mut invulnerability: ResMut<Invulnerability>,
) {
    score.reset();
    spawn_timer.reset();
    health.reset_to_max();
    invulnerability.reset();

    clear_all_obstacles(&mut commands, &obstacles);
    reset_player_state(&mut player_query);
}