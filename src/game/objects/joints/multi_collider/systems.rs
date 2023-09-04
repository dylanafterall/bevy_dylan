use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_multi_collider(
    mut commands: Commands
) {
    commands
        .spawn(RigidBody::Dynamic)
        .insert(TransformBundle::from(Transform::from_xyz(
            -400.0, -400.0, 0.0,
        )))
        .insert(GravityScale(0.0))
        .with_children(|children| {
            // top left
            children.spawn(Collider::ball(50.0))
                .insert(TransformBundle::from(Transform::from_xyz(-150.0, 150.0, 0.0)));
            // top right
            children.spawn(Collider::ball(50.0))
                .insert(TransformBundle::from(Transform::from_xyz(150.0, 150.0, 0.0)));
            // bottom right
            children.spawn(Collider::ball(50.0))
                .insert(TransformBundle::from(Transform::from_xyz(150.0, -150.0, 0.0)));
            // bottom left
            children.spawn(Collider::ball(50.0))
                .insert(TransformBundle::from(Transform::from_xyz(-150.0, -150.0, 0.0)));
        });
}