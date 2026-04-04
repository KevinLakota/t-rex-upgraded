use bevy::prelude::*;
use crate::constants::*;

#[derive(Resource)]
pub struct Health {
    pub current: u32,
    pub max: u32,
}

#[derive(Resource)]
pub struct Invulnerability {
    pub active: bool,
    pub timer: f32,
    pub blink_timer: f32,
    pub visible: bool,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            current: MAX_HEALTH,
            max: MAX_HEALTH,
        }
    }
}

impl Default for Invulnerability {
    fn default() -> Self {
        Self {
            active: false,
            timer: 0.0,
            blink_timer: 0.0,
            visible: true,
        }
    }
}

impl Health {
    pub fn reset_to_max(&mut self) {
        self.current = self.max;
    }
}

impl Invulnerability {
    pub fn reset(&mut self) {
        self.active = false;
        self.timer = 0.0;
        self.blink_timer = 0.0;
        self.visible = true;
    }
}