use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_prismatic_joint(
    mut commands: Commands,
) {
    let joint = PrismaticJointBuilder::new(Vec2::Y)
        .local_anchor1(Vec2::new(300.0, 0.0))
        .limits([-100.0, 100.0])
        .motor_position(100.0, 10.0, 0.1);

    let parent_entity = commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::ball(50.0))
        .insert(GravityScale(0.0))
        .insert(TransformBundle::from(Transform::from_xyz(
            -800.0, -400.0, 0.0
        )))
        .id();

    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(50.0))
        .insert(GravityScale(0.0))
        .insert(TransformBundle::from(Transform::from_xyz(
            -500.0, -400.0, 0.0
        )))
        .insert(ImpulseJoint::new(parent_entity, joint));
}