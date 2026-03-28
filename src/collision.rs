use bevy::prelude::*;

use crate::constants::*;
use crate::game_state::GameState;
use crate::obstacle::Obstacle;
use crate::player::Player;

pub fn check_collision(
    mut game_state: ResMut<GameState>,
    player_query: Query<&Transform, With<Player>>,
    obstacle_query: Query<(&Transform, &Obstacle)>,
) {
    if *game_state != GameState::Running {
        return;
    }

    let Ok(player_transform) = player_query.single() else {
        return;
    };

    for (obstacle_transform, obstacle) in &obstacle_query {
        let collision = collide(
            player_transform.translation,
            Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT),
            obstacle_transform.translation,
            obstacle.obstacle_type.size(),
        );

        if collision {
            *game_state = GameState::GameOver;
            println!("GAME OVER");
        }
    }
}

fn collide(pos_a: Vec3, size_a: Vec2, pos_b: Vec3, size_b: Vec2) -> bool {
    let a_half = size_a / 2.0;
    let b_half = size_b / 2.0;

    let a_left = pos_a.x - a_half.x;
    let a_right = pos_a.x + a_half.x;
    let a_bottom = pos_a.y - a_half.y;
    let a_top = pos_a.y + a_half.y;

    let b_left = pos_b.x - b_half.x;
    let b_right = pos_b.x + b_half.x;
    let b_bottom = pos_b.y - b_half.y;
    let b_top = pos_b.y + b_half.y;

    a_left < b_right && a_right > b_left && a_bottom < b_top && a_top > b_bottom
}