use bevy::prelude::*;
use crate::constants::*;
use crate::game_state::GameState;

#[derive(Resource)]
pub struct Score {
    pub distance: f32,
}

pub fn update_score(
    mut score: ResMut<Score>,
    time: Res<Time>,
    game_state: Res<GameState>,
) {
    if *game_state != GameState::Running {
        return;
    }

    score.distance += OBSTACLE_SPEED * time.delta_secs();
}