use bevy::prelude::*;
use crate::difficulty::Difficulty;

#[derive(Resource)]
pub struct Score {
    pub distance: f32,
}

impl Score {
    pub fn reset(&mut self) {
        self.distance = 0.0;
    }
}

pub fn update_score(
    mut score: ResMut<Score>,
    time: Res<Time>,
    difficulty: Res<Difficulty>,
) {
    score.distance += difficulty.obstacle_speed * time.delta_secs();
}