use crate::game::scene_manager::SceneState;
use bevy::prelude::*;

// -----------------------------------------------------------------------------
#[derive(Component)]
pub enum PlayerCollisionSensor {
    SceneSensor(SceneState),
    GravSensor(Vec2),
}
