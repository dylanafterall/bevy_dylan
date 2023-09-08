use crate::system_params::*;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_conveyor_belt(
    mut commands: Commands,
) {
    // small rect to test conveyor belt
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::cuboid(50.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(-190.0, -400.0, 0.0)));

    // vertical box stopping small test rect from falling off
    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(25.0, 60.0))
        .insert(TransformBundle::from(Transform::from_xyz(435.0, -575.0, 0.0)));

    // collision box surrounding conveyor belt
    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(410.0, 25.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -610.0, 0.0)));

    // actual conveyor belt
    commands
        .spawn(RigidBody::KinematicPositionBased)
        .insert(ConveyorBelt)
        .insert(ActiveHooks::MODIFY_SOLVER_CONTACTS)
        .insert(Collider::cuboid(400.0, 25.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -600.0, 0.0)));
}