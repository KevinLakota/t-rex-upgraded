use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct PlayerProfile {
    pub name: String,
}