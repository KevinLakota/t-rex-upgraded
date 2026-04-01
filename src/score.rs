use bevy::prelude::*;
use crate::game_state::GameState;
use crate::difficulty::Difficulty;

#[derive(Resource)]
pub struct Score {
    pub distance: f32,
}

pub fn update_score(
    mut score: ResMut<Score>,
    time: Res<Time>,
    game_state: Res<GameState>,
    difficulty: Res<Difficulty>,
) {
    if *game_state != GameState::Running {
        return;
    }

    score.distance += difficulty.obstacle_speed * time.delta_secs();
}