use crate::game::scene_manager::SceneState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::RapierRigidBodyHandle;

// -----------------------------------------------------------------------------
#[derive(Event)]
pub struct PlayerSceneCollision {
    pub desired_scene: SceneState,
}

#[derive(Event)]
pub struct PlayerGravCollision {
    pub grav_direction: Vec2,
}

#[derive(Event)]
pub struct PlayerContact {
    pub player: Entity,
    pub partner: Entity,
    pub force_vector: Vec2,
}

#[derive(Event)]
pub struct DestructibleContact {
    pub destructible_rb_handle: RapierRigidBodyHandle,
    pub destructible_entity: Entity,
}
