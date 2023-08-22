use bevy::prelude::*;

// store window resolutions to choose between
#[derive(Resource)]
pub struct ResolutionSettings {
    pub _large: Vec2,
    pub _medium: Vec2,
    pub _small: Vec2,
}