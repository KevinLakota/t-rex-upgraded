use bevy::prelude::*;
use crate::score::Score;
use crate::constants::{BASE_SPEED, MAX_SPEED};

#[derive(Resource)]
pub struct Difficulty {
    pub obstacle_speed: f32,
}
pub fn update_difficulty(
    score: Res<Score>,
    mut difficulty: ResMut<Difficulty>,
) {
    let new_speed = BASE_SPEED + score.distance / 40.0;

    difficulty.obstacle_speed = new_speed.min(MAX_SPEED);
}