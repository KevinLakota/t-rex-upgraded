use bevy::prelude::*;
use crate::constants::*;
use crate::game_state::GameState;

#[derive(Component)]
pub struct Obstacle;

#[derive(Resource)]
pub struct ObstacleSpawnTimer {
    pub timer: f32,
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

    if spawn_timer.timer >= OBSTACLE_SPAWN_INTERVAL {
        spawn_timer.timer = 0.0;

        commands.spawn((
            Sprite::from_color(
                Color::srgb(0.8, 0.2, 0.2),
                Vec2::new(OBSTACLE_WIDTH, OBSTACLE_HEIGHT),
            ),
            Transform::from_xyz(
                OBSTACLE_START_X,
                GROUND_Y + OBSTACLE_HEIGHT / 2.0,
                1.0,
            ),
            Obstacle,
        ));
    }
}

pub fn move_obstacles(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform), With<Obstacle>>,
    time: Res<Time>,
    game_state: Res<GameState>,
) {
    if *game_state != GameState::Running {
        return;
    }

    for (entity, mut transform) in &mut query {
        transform.translation.x -= OBSTACLE_SPEED * time.delta_secs();

        if transform.translation.x < DESPAWN_X {
            commands.entity(entity).despawn();
        }
    }
}