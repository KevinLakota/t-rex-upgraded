use bevy::prelude::*;
use rand::{RngExt};
use crate::difficulty::Difficulty;

use crate::constants::*;
use crate::game_state::GameState;

#[derive(Clone, Copy, Debug)]
pub enum ObstacleType {
    SmallCactus,
    LargeCactus,
}

#[derive(Component)]
pub struct Obstacle {
    pub obstacle_type: ObstacleType,
}

#[derive(Resource)]
pub struct ObstacleSpawnTimer {
    pub timer: f32,
    pub next_spawn_time: f32,
}

impl ObstacleType {
    pub fn size(&self) -> Vec2 {
        match self {
            ObstacleType::SmallCactus => Vec2::new(30.0, 50.0),
            ObstacleType::LargeCactus => Vec2::new(45.0, 70.0),
        }
    }

    pub fn color(&self) -> Color {
        match self {
            ObstacleType::SmallCactus => Color::srgb(0.8, 0.2, 0.2),
            ObstacleType::LargeCactus => Color::srgb(0.9, 0.4, 0.2),
        }
    }
}

fn random_obstacle_type() -> ObstacleType {
    let mut rng = rand::rng();

    if rng.random_bool(0.7) {
        ObstacleType::SmallCactus
    } else {
        ObstacleType::LargeCactus
    }
}

fn random_spawn_interval() -> f32 {
    let mut rng = rand::rng();
    rng.random_range(1.2..=2.0)
}

pub fn spawn_obstacle(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: ResMut<ObstacleSpawnTimer>,
    game_state: Res<GameState>,
) {
    if *game_state != GameState::Running {
        return;
    }

    spawn_timer.timer += time.delta_secs();

    if spawn_timer.timer >= spawn_timer.next_spawn_time {
        spawn_timer.timer = 0.0;
        spawn_timer.next_spawn_time = random_spawn_interval();

        let obstacle_type = random_obstacle_type();
        let size = obstacle_type.size();

        commands.spawn((
            Sprite::from_color(obstacle_type.color(), size),
            Transform::from_xyz(
                OBSTACLE_START_X,
                GROUND_Y + size.y / 2.0,
                1.0,
            ),
            Obstacle { obstacle_type },
        ));
    }
}

pub fn move_obstacles(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform), With<Obstacle>>,
    time: Res<Time>,
    game_state: Res<GameState>,
    difficulty: Res<Difficulty>,
) {
    if *game_state != GameState::Running {
        return;
    }

    for (entity, mut transform) in &mut query {
        transform.translation.x -= difficulty.obstacle_speed * time.delta_secs();

        if transform.translation.x < DESPAWN_X {
            commands.entity(entity).despawn();
        }
    }
}