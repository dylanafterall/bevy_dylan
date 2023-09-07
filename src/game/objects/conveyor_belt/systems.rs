use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_conveyor_belt(
    mut commands: Commands,
) {
    commands
        .spawn(RigidBody::KinematicPositionBased)
        .insert(ActiveHooks::FILTER_CONTACT_PAIRS | ActiveHooks::MODIFY_SOLVER_CONTACTS)
        .insert(Collider::cuboid(400.0, 25.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -600.0, 0.0)));
}