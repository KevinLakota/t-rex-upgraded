use bevy::prelude::*;
use crate::constants::*;
use crate::health::Invulnerability;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Velocity {
    pub y: f32,
}

pub fn reset_player(
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

pub fn player_movement(
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    time: Res<Time>,
) {
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
) {
    for (transform, mut velocity) in &mut query {
        let on_ground = transform.translation.y <= PLAYER_START_Y + 0.1;

        if keyboard.just_pressed(KeyCode::Space) && on_ground {
            velocity.y = JUMP_FORCE;
        }
    }
}

pub fn update_invulnerability(
    mut invulnerability: ResMut<Invulnerability>,
    time: Res<Time>,
) {
    if !invulnerability.active {
        return;
    }

    invulnerability.timer -= time.delta_secs();
    invulnerability.blink_timer += time.delta_secs();

    if invulnerability.timer <= 0.0 {
        invulnerability.active = false;
        invulnerability.timer = 0.0;
        invulnerability.blink_timer = 0.0;
        invulnerability.visible = true;
    }
}

pub fn blink_player(
    invulnerability: Res<Invulnerability>,
    mut query: Query<&mut Visibility, With<Player>>,
) {
    let Ok(mut visibility) = query.single_mut() else {
        return;
    };

    if !invulnerability.active {
        *visibility = Visibility::Visible;
        return;
    }

    let blink_phase = (invulnerability.blink_timer / BLINK_INTERVAL) as i32 % 2 == 0;

    *visibility = if blink_phase {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };
}