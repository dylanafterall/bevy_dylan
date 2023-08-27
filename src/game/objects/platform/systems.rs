use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_platform(mut commands: Commands) {
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));
}

pub fn despawn_platform(
    mut commands: Commands,
    platform_query: Query<Entity, (With<Collider>, Without<RigidBody>)>
) {
    for platform in platform_query.iter() {
        commands.entity(platform).despawn();
    }
}