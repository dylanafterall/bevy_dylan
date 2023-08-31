use super::SceneState;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
#[derive(Component)]
pub struct SceneSensor {
    pub desired_scene: SceneState,
}