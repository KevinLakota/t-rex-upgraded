use bevy::prelude::*;
use std::error::Error;
use std::fmt;
use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, BufReader};
use std::path::Path;

use crate::constants::{MAX_SCOREBOARD_ENTRIES, SCOREBOARD_FILE};
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

#[derive(Debug)]
pub enum ScoreboardError {
    FileOpenError(io::Error),
    FileReadError(io::Error),
    FileWriteError(io::Error),
    InvalidLineFormat(String),
    InvalidScore(String),
}

impl fmt::Display for ScoreboardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScoreboardError::FileOpenError(err) => {
                write!(f, "Failed to open scoreboard file: {}", err)
            }
            ScoreboardError::FileReadError(err) => {
                write!(f, "Failed to read scoreboard file: {}", err)
            }
            ScoreboardError::FileWriteError(err) => {
                write!(f, "Failed to write scoreboard file: {}", err)
            }
            ScoreboardError::InvalidLineFormat(line) => {
                write!(f, "Invalid scoreboard line format: {}", line)
            }
            ScoreboardError::InvalidScore(score) => {
                write!(f, "Invalid score value: {}", score)
            }
        }
    }
}

impl Error for ScoreboardError {}

impl Scoreboard {
    pub fn load_from_file() -> Self {
        match Self::try_load_from_file() {
            Ok(scoreboard) => scoreboard,
            Err(error) => {
                warn!("Scoreboard could not be loaded: {}", error);
                Self::default()
            }
        }
    }

    pub fn try_load_from_file() -> Result<Self, ScoreboardError> {
        let path = Path::new(SCOREBOARD_FILE);

        if !path.exists() {
            return Ok(Self::default());
        }

        let file = OpenOptions::new()
            .read(true)
            .open(path)
            .map_err(ScoreboardError::FileOpenError)?;

        let reader = BufReader::new(file);
        let mut scoreboard = Self::default();

        for line_result in reader.lines() {
            let line = line_result.map_err(ScoreboardError::FileReadError)?;

            let parts: Vec<&str> = line.split('|').collect();

            if parts.len() != 2 {
                return Err(ScoreboardError::InvalidLineFormat(line));
            }

            let name = parts[0].trim().to_string();

            if name.is_empty() {
                return Err(ScoreboardError::InvalidLineFormat(line));
            }

            let score_text = parts[1].trim();

            let score = score_text
                .parse::<i32>()
                .map_err(|_| ScoreboardError::InvalidScore(score_text.to_string()))?;

            scoreboard.add_entry(name, score);
        }

        Ok(scoreboard)
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

    pub fn try_save_to_file(&self) -> Result<(), ScoreboardError> {
        let mut content = String::new();

        for entry in &self.entries {
            content.push_str(&format!("{}|{}\n", entry.name, entry.score));
        }

        fs::write(SCOREBOARD_FILE, content)
            .map_err(ScoreboardError::FileWriteError)?;

        Ok(())
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

    if let Err(error) = scoreboard.try_save_to_file() {
        error!("Failed to save score on game over: {}", error);
    }
}