use bevy::prelude::*;
use rand::{RngExt};

use crate::constants::*;
use crate::difficulty::Difficulty;
use crate::game_state::GameState;
use crate::score::Score;

#[derive(Clone, Copy, Debug)]
pub enum ObstacleType {
    SmallCactus,
    LargeCactus,
}

#[derive(Clone, Copy, Debug)]
pub enum SpawnPattern {
    Close,
    Normal,
    Far,
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

impl SpawnPattern {
    pub fn interval_range(&self) -> (f32, f32) {
        match self {
            SpawnPattern::Close => (1.10, 1.35),
            SpawnPattern::Normal => (1.45, 1.85),
            SpawnPattern::Far => (1.95, 2.35),
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

fn choose_spawn_pattern(score: f32) -> SpawnPattern {
    let mut rng = rand::rng();

    if score < 2000.0 {
        if rng.random_bool(0.75) {
            SpawnPattern::Normal
        } else {
            SpawnPattern::Far
        }
    } else if score < 8000.0 {
        let roll = rng.random_range(0..100);

        if roll < 20 {
            SpawnPattern::Close
        } else if roll < 75 {
            SpawnPattern::Normal
        } else {
            SpawnPattern::Far
        }
    } else {
        let roll = rng.random_range(0..100);

        if roll < 35 {
            SpawnPattern::Close
        } else if roll < 80 {
            SpawnPattern::Normal
        } else {
            SpawnPattern::Far
        }
    }
}

fn random_spawn_interval(pattern: SpawnPattern, speed: f32) -> f32 {
    let mut rng = rand::rng();
    let (min_time, max_time) = pattern.interval_range();

    let speed_factor = (BASE_SPEED / speed).clamp(0.65, 1.0);

    rng.random_range((min_time * speed_factor)..=(max_time * speed_factor))
}

pub fn spawn_obstacle(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: ResMut<ObstacleSpawnTimer>,
    game_state: Res<GameState>,
    score: Res<Score>,
    difficulty: Res<Difficulty>,
) {
    if *game_state != GameState::Running {
        return;
    }

    spawn_timer.timer += time.delta_secs();

    if spawn_timer.timer >= spawn_timer.next_spawn_time {
        spawn_timer.timer = 0.0;

        let pattern = choose_spawn_pattern(score.distance);
        spawn_timer.next_spawn_time =
            random_spawn_interval(pattern, difficulty.obstacle_speed);

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