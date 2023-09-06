use bevy::prelude::*;
use bevy_rapier2d::prelude::RapierRigidBodyHandle;
use bevy_rapier2d::rapier::prelude::CollisionEventFlags;

// -----------------------------------------------------------------------------
#[derive(Event)]
pub struct PlayerSceneCollision {
    pub scene_sensor: Entity,
}

#[derive(Event)]
pub struct PlayerOtherCollision {
    pub player: Entity,
    pub partner: Entity,
    pub flag: CollisionEventFlags,
}

#[derive(Event)]
pub struct PlayerContact {
    pub player: Entity,
    pub partner: Entity,
    pub force_vector: Vec2,
}

#[derive(Event)]
pub struct DestructibleContact {
    pub destructible: RapierRigidBodyHandle,
}

