use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_fan(
    mut commands: Commands
) {
    let revolute_joint = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(-200.0, 0.0))
        .motor_velocity(2.0, 1.0);

    let fan = commands
        .spawn(RigidBody::Dynamic)
        .insert(TransformBundle::from(Transform::from_xyz(
            600.0, -400.0, 0.0,
        )))
        .insert(GravityScale(0.0))
        .with_children(|children| {
            // top left
            children.spawn(Collider::ball(50.0))
                .insert(TransformBundle::from(Transform::from_xyz(-100.0, 100.0, 0.0)));
            // top right
            children.spawn(Collider::ball(50.0))
                .insert(TransformBundle::from(Transform::from_xyz(100.0, 100.0, 0.0)));
            // bottom right
            children.spawn(Collider::ball(50.0))
                .insert(TransformBundle::from(Transform::from_xyz(100.0, -100.0, 0.0)));
            // bottom left
            children.spawn(Collider::ball(50.0))
                .insert(TransformBundle::from(Transform::from_xyz(-100.0, -100.0, 0.0)));
        })
        .id();

    commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(50.0,50.0))
        .insert(GravityScale(0.0))
        .insert(TransformBundle::from(Transform::from_xyz(
            400.0, -400.0, 0.0
        )))
        .insert(ImpulseJoint::new(fan, revolute_joint));
}