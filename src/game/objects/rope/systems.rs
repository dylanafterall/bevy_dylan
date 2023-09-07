use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_rope(
    mut commands: Commands,
) {
    const RAD: f32 = 10.0;
    const ROPE_LENGTH: f32 = RAD * 3.0;

    let joint = RopeJointBuilder::new()
        .limits([0.0, ROPE_LENGTH]);

    let one = commands
        .spawn((
            TransformBundle::from(Transform::from_xyz(-400.0, -400.0, 0.0)),
            RigidBody::Fixed,
            LockedAxes::ROTATION_LOCKED,
            ColliderMassProperties::Density(10.0),
            Damping {
                linear_damping: 2.0,
                angular_damping: 1.0,
            },
            GravityScale(-8.0),
            Collider::cuboid(RAD * 2.0, RAD * 2.0),
        ))
        .id();

    let two = commands
        .spawn((
            TransformBundle::from(Transform::from_xyz(-400.0, -370.0, 0.0)),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            ColliderMassProperties::Density(10.0),
            Damping {
                linear_damping: 2.0,
                angular_damping: 1.0,
            },
            GravityScale(-8.0),
            Collider::ball(RAD),
        ))
        .insert(ImpulseJoint::new(one, joint))
        .id();

    let three = commands
        .spawn((
            TransformBundle::from(Transform::from_xyz(-400.0, -340.0, 0.0)),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            ColliderMassProperties::Density(10.0),
            Damping {
                linear_damping: 2.0,
                angular_damping: 1.0,
            },
            GravityScale(-8.0),
            Collider::ball(RAD),
        ))
        .insert(ImpulseJoint::new(two, joint))
        .id();

    let four = commands
        .spawn((
            TransformBundle::from(Transform::from_xyz(-400.0, -310.0, 0.0)),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            ColliderMassProperties::Density(10.0),
            Damping {
                linear_damping: 2.0,
                angular_damping: 1.0,
            },
            GravityScale(-8.0),
            Collider::ball(RAD),
        ))
        .insert(ImpulseJoint::new(three, joint))
        .id();

    let five = commands
        .spawn((
            TransformBundle::from(Transform::from_xyz(-400.0, -280.0, 0.0)),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            ColliderMassProperties::Density(10.0),
            Damping {
                linear_damping: 2.0,
                angular_damping: 1.0,
            },
            GravityScale(-8.0),
            Collider::ball(RAD),
        ))
        .insert(ImpulseJoint::new(four, joint))
        .id();

    let six = commands
        .spawn((
            TransformBundle::from(Transform::from_xyz(-400.0, -250.0, 0.0)),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            ColliderMassProperties::Density(10.0),
            Damping {
                linear_damping: 2.0,
                angular_damping: 1.0,
            },
            GravityScale(-8.0),
            Collider::ball(RAD),
        ))
        .insert(ImpulseJoint::new(five, joint))
        .id();

    let seven = commands
        .spawn((
            TransformBundle::from(Transform::from_xyz(-400.0, -220.0, 0.0)),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            ColliderMassProperties::Density(10.0),
            Damping {
                linear_damping: 2.0,
                angular_damping: 1.0,
            },
            GravityScale(-8.0),
            Collider::ball(RAD),
        ))
        .insert(ImpulseJoint::new(six, joint))
        .id();

    let eight = commands
        .spawn((
            TransformBundle::from(Transform::from_xyz(-400.0, -190.0, 0.0)),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            ColliderMassProperties::Density(10.0),
            Damping {
                linear_damping: 2.0,
                angular_damping: 1.0,
            },
            GravityScale(-8.0),
            Collider::ball(RAD),
        ))
        .insert(ImpulseJoint::new(seven, joint))
        .id();

    let nine = commands
        .spawn((
            TransformBundle::from(Transform::from_xyz(-400.0, -160.0, 0.0)),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            ColliderMassProperties::Density(10.0),
            Damping {
                linear_damping: 2.0,
                angular_damping: 1.0,
            },
            GravityScale(-8.0),
            Collider::ball(RAD),
        ))
        .insert(ImpulseJoint::new(eight, joint))
        .id();

    let ten = commands
        .spawn((
            TransformBundle::from(Transform::from_xyz(-400.0, -130.0, 0.0)),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            ColliderMassProperties::Density(10.0),
            Damping {
                linear_damping: 2.0,
                angular_damping: 1.0,
            },
            GravityScale(-8.0),
            Collider::ball(RAD),
        ))
        .insert(ImpulseJoint::new(nine, joint))
        .id();

    let eleven = commands
        .spawn((
            TransformBundle::from(Transform::from_xyz(-400.0, -100.0, 0.0)),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            ColliderMassProperties::Density(10.0),
            Damping {
                linear_damping: 2.0,
                angular_damping: 1.0,
            },
            GravityScale(-8.0),
            Collider::ball(RAD),
        ))
        .insert(ImpulseJoint::new(ten, joint))
        .id();

    let twelve = commands
        .spawn((
            TransformBundle::from(Transform::from_xyz(-400.0, -70.0, 0.0)),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            ColliderMassProperties::Density(10.0),
            Damping {
                linear_damping: 2.0,
                angular_damping: 1.0,
            },
            GravityScale(-8.0),
            Collider::ball(RAD),
        ))
        .insert(ImpulseJoint::new(eleven, joint))
        .id();

    let thirteen = commands
        .spawn((
            TransformBundle::from(Transform::from_xyz(-400.0, -40.0, 0.0)),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            ColliderMassProperties::Density(10.0),
            Damping {
                linear_damping: 2.0,
                angular_damping: 1.0,
            },
            GravityScale(-8.0),
            Collider::ball(RAD),
        ))
        .insert(ImpulseJoint::new(twelve, joint))
        .id();

    let fourteen = commands
        .spawn((
            TransformBundle::from(Transform::from_xyz(-400.0, -10.0, 0.0)),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            ColliderMassProperties::Density(10.0),
            Damping {
                linear_damping: 2.0,
                angular_damping: 1.0,
            },
            GravityScale(-8.0),
            Collider::ball(RAD),
        ))
        .insert(ImpulseJoint::new(thirteen, joint))
        .id();

    let fifteen = commands
        .spawn((
            TransformBundle::from(Transform::from_xyz(-400.0, 20.0, 0.0)),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            ColliderMassProperties::Density(10.0),
            Damping {
                linear_damping: 2.0,
                angular_damping: 1.0,
            },
            GravityScale(-8.0),
            Collider::ball(RAD),
        ))
        .insert(ImpulseJoint::new(fourteen, joint))
        .id();

    commands
        .spawn((
            TransformBundle::from(Transform::from_xyz(-400.0, 50.0, 0.0)),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            ColliderMassProperties::Density(10.0),
            Damping {
                linear_damping: 2.0,
                angular_damping: 1.0,
            },
            GravityScale(-8.0),
            Collider::ball(RAD),
        ))
        .insert(ImpulseJoint::new(fifteen, joint));
}