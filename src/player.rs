use bevy::prelude::*;
use crate::constants::*;
use crate::game_state::GameState;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Velocity {
    pub y: f32,
}

pub fn player_movement(
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    time: Res<Time>,
    game_state: Res<GameState>,
) {
    if *game_state != GameState::Running {
        return;
    }

    for (mut transform, mut velocity) in &mut query {
        velocity.y += GRAVITY * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();

        if transform.translation.y <= PLAYER_START_Y {
            transform.translation.y = PLAYER_START_Y;
            velocity.y = 0.0;
        }
    }
}

pub fn player_jump(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Transform, &mut Velocity), With<Player>>,
    game_state: Res<GameState>,
) {
    if *game_state != GameState::Running {
        return;
    }

    for (transform, mut velocity) in &mut query {
        let on_ground = transform.translation.y <= PLAYER_START_Y + 0.1;

        if keyboard.just_pressed(KeyCode::Space) && on_ground {
            velocity.y = JUMP_FORCE;
        }
    }
}