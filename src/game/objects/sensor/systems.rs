use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_sensor(mut commands: Commands) {
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(Sensor)
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 200.0, 0.0)));
}