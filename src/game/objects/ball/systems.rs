use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_ball(mut commands: Commands) {
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(50.0))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));
}