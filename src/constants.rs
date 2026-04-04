pub const WINDOW_WIDTH: u32 = 1200;
pub const WINDOW_HEIGHT: u32 = 400;

pub const WORLD_WIDTH: f32 = 1200.0;
pub const GROUND_Y: f32 = -120.0;

pub const PLAYER_WIDTH: f32 = 40.0;
pub const PLAYER_HEIGHT: f32 = 60.0;
pub const PLAYER_START_X: f32 = -400.0;
pub const PLAYER_START_Y: f32 = GROUND_Y + 32.0;

pub const GRAVITY: f32 = -900.0;
pub const JUMP_FORCE: f32 = 400.0;
pub const OBSTACLE_START_X: f32 = 600.0;
pub const BASE_SPEED: f32 = 300.0;
pub const MAX_SPEED: f32 = 650.0;
pub const OBSTACLE_SPAWN_INTERVAL: f32 = 1.5;
pub const DESPAWN_X: f32 = -700.0;

pub const MAX_HEALTH: u32 = 3;
pub const INVULNERABILITY_DURATION: f32 = 1.0;
pub const BLINK_INTERVAL: f32 = 0.12;

pub const MAX_SCOREBOARD_ENTRIES: usize = 20;
pub const SCOREBOARD_FILE: &str = "scoreboard.txt";