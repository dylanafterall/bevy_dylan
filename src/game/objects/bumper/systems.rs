use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_bumper(
    mut commands: Commands,
) {
    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::ball(75.0))
        .insert(Restitution::coefficient(10.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));
}