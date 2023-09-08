use crate::system_params::*;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_one_way_platform(mut commands: Commands) {
    commands
        .spawn(Collider::cuboid(150.0, 25.0))
        .insert(ActiveHooks::MODIFY_SOLVER_CONTACTS)
        .insert(OneWayPlatform)
        .insert(TransformBundle::from(Transform::from_xyz(-800.0, 250.0, 0.0)));
}