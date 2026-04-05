use bevy::prelude::*;

use crate::health::{Health, Invulnerability};
use crate::obstacle::{Obstacle, ObstacleSpawnTimer, clear_obstacles};
use crate::player::{Player, Velocity, PlayerAnimation, reset_player};
use crate::score::Score;

pub fn reset_game(
    mut commands: Commands,
    obstacles: Query<Entity, With<Obstacle>>,
    mut player_query: Query<
        (&mut Transform, &mut Velocity, &mut Visibility, &mut Sprite, &mut PlayerAnimation),
        With<Player>,
    >,
    mut score: ResMut<Score>,
    mut spawn_timer: ResMut<ObstacleSpawnTimer>,
    mut health: ResMut<Health>,
    mut invulnerability: ResMut<Invulnerability>,
) {
    score.reset();
    spawn_timer.reset();
    health.reset_to_max();
    invulnerability.reset();

    clear_obstacles(&mut commands, &obstacles);
    reset_player(&mut player_query);
}