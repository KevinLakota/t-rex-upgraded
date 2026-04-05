use bevy::prelude::*;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader};
use std::path::Path;
use crate::constants::{SCOREBOARD_FILE, MAX_SCOREBOARD_ENTRIES};
use crate::player_profile::PlayerProfile;
use crate::score::Score;

#[derive(Clone, Debug)]
pub struct ScoreEntry {
    pub name: String,
    pub score: i32,
}

#[derive(Resource, Default)]
pub struct Scoreboard {
    pub entries: Vec<ScoreEntry>,
}

impl Scoreboard {
    pub fn load_from_file() -> Self {
        let path = Path::new(SCOREBOARD_FILE);

        if !path.exists() {
            return Self::default();
        }

        let file = match OpenOptions::new().read(true).open(path) {
            Ok(file) => file,
            Err(_) => return Self::default(),
        };

        let reader = BufReader::new(file);
        let mut scoreboard = Self::default();

        for line in reader.lines() {
            let Ok(line) = line else {
                continue;
            };

            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() != 2 {
                continue;
            }

            let name = parts[0].trim().to_string();
            let Ok(score) = parts[1].trim().parse::<i32>() else {
                continue;
            };

            if name.is_empty() {
                continue;
            }

            scoreboard.add_entry(name, score);
        }

        scoreboard
    }

    pub fn add_entry(&mut self, name: String, score: i32) {
        let normalized_name = name.trim();

        if normalized_name.is_empty() {
            return;
        }

        if let Some(existing) = self
            .entries
            .iter_mut()
            .find(|entry| entry.name.eq_ignore_ascii_case(normalized_name))
        {
            if score > existing.score {
                existing.name = normalized_name.to_string();
                existing.score = score;
            }
        } else {
            self.entries.push(ScoreEntry {
                name: normalized_name.to_string(),
                score,
            });
        }

        self.entries.sort_by(|a, b| b.score.cmp(&a.score));
        self.entries.truncate(MAX_SCOREBOARD_ENTRIES);
    }

    pub fn save_to_file(&self) {
        let mut content = String::new();

        for entry in &self.entries {
            content.push_str(&format!("{}|{}\n", entry.name, entry.score));
        }

        let _ = fs::write(SCOREBOARD_FILE, content);
    }

    pub fn top_entries(&self, count: usize) -> Vec<ScoreEntry> {
        self.entries.iter().take(count).cloned().collect()
    }
}

pub fn save_score_on_game_over(
    player_profile: Res<PlayerProfile>,
    score: Res<Score>,
    mut scoreboard: ResMut<Scoreboard>,
) {
    let name = player_profile.name.trim();

    if name.is_empty() {
        return;
    }

    scoreboard.add_entry(name.to_string(), score.distance as i32);
    scoreboard.save_to_file();
}