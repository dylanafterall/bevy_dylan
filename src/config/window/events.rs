use super::resources::AspectRatio;
use bevy::prelude::*;

// -----------------------------------------------------------------------------
#[derive(Event)]
pub struct ChangeResolution {
    pub resolution: Vec2,
    pub aspect_ratio: AspectRatio,
}
