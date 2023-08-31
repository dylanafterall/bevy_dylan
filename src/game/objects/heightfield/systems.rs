use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_heightfield(mut commands: Commands) {
    commands
        .spawn(Collider::heightfield(
            vec![100.0, -200.0, 0.0, -50.0, 300.0],
            Vec2::new(500.0, 1.0),
        ))
        .insert(TransformBundle::from(Transform::from_xyz(-800.0, -400.0, 0.0)));
}