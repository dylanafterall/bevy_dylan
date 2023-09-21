use crate::style::{
    FRAPPE_BLUE, FRAPPE_FLAMINGO, FRAPPE_GREEN, FRAPPE_LAVENDER, FRAPPE_MAROON, FRAPPE_MAUVE,
    FRAPPE_PEACH, FRAPPE_PINK, FRAPPE_RED, FRAPPE_ROSEWATER, FRAPPE_SAPPHIRE, FRAPPE_SKY,
    FRAPPE_SURFACE2, FRAPPE_TEAL, FRAPPE_TEXT, FRAPPE_YELLOW,
};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_rope(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    const RAD: f32 = 1.0;
    const ROPE_LENGTH: f32 = RAD * 3.0;

    let joint = RopeJointBuilder::new().limits([0.0, ROPE_LENGTH]);

    let one = commands
        // info
        // ----
        .spawn(Name::new("RopeAnchor"))
        // physics
        // -------
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(RAD * 2.0, RAD * 2.0))
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(RAD * 4.0, RAD * 4.0)).into())
                .into(),
            material: materials.add(ColorMaterial::from(FRAPPE_SURFACE2)),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(
            -40.0, -40.0, 0.0,
        )))
        .id();

    let two = commands
        // info
        // ----
        .spawn(Name::new("RopeTwo"))
        // physics
        // -------
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(RAD))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(ColliderMassProperties::Density(5.0))
        .insert(Damping {
            linear_damping: 2.0,
            angular_damping: 1.0,
        })
        .insert(GravityScale(-4.0))
        .insert(ImpulseJoint::new(one, joint))
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(RAD).into()).into(),
            material: materials.add(ColorMaterial::from(FRAPPE_TEXT)),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(
            -40.0, -37.0, 0.0,
        )))
        .id();

    let three = commands
        // info
        // ----
        .spawn(Name::new("RopeThree"))
        // physics
        // -------
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(RAD))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(ColliderMassProperties::Density(5.0))
        .insert(Damping {
            linear_damping: 2.0,
            angular_damping: 1.0,
        })
        .insert(GravityScale(-4.0))
        .insert(ImpulseJoint::new(two, joint))
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(RAD).into()).into(),
            material: materials.add(ColorMaterial::from(FRAPPE_LAVENDER)),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(
            -40.0, -34.0, 0.0,
        )))
        .id();

    let four = commands
        // info
        // ----
        .spawn(Name::new("RopeFour"))
        // physics
        // -------
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(RAD))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(ColliderMassProperties::Density(5.0))
        .insert(Damping {
            linear_damping: 2.0,
            angular_damping: 1.0,
        })
        .insert(GravityScale(-4.0))
        .insert(ImpulseJoint::new(three, joint))
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(RAD).into()).into(),
            material: materials.add(ColorMaterial::from(FRAPPE_BLUE)),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(
            -40.0, -31.0, 0.0,
        )))
        .id();

    let five = commands
        // info
        // ----
        .spawn(Name::new("RopeFive"))
        // physics
        // -------
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(RAD))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(ColliderMassProperties::Density(5.0))
        .insert(Damping {
            linear_damping: 2.0,
            angular_damping: 1.0,
        })
        .insert(GravityScale(-4.0))
        .insert(ImpulseJoint::new(four, joint))
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(RAD).into()).into(),
            material: materials.add(ColorMaterial::from(FRAPPE_SAPPHIRE)),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(
            -40.0, -28.0, 0.0,
        )))
        .id();

    let six = commands
        // info
        // ----
        .spawn(Name::new("RopeSix"))
        // physics
        // -------
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(RAD))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(ColliderMassProperties::Density(5.0))
        .insert(Damping {
            linear_damping: 2.0,
            angular_damping: 1.0,
        })
        .insert(GravityScale(-4.0))
        .insert(ImpulseJoint::new(five, joint))
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(RAD).into()).into(),
            material: materials.add(ColorMaterial::from(FRAPPE_SKY)),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(
            -40.0, -25.0, 0.0,
        )))
        .id();

    let seven = commands
        // info
        // ----
        .spawn(Name::new("RopeSeven"))
        // physics
        // -------
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(RAD))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(ColliderMassProperties::Density(5.0))
        .insert(Damping {
            linear_damping: 2.0,
            angular_damping: 1.0,
        })
        .insert(GravityScale(-4.0))
        .insert(ImpulseJoint::new(six, joint))
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(RAD).into()).into(),
            material: materials.add(ColorMaterial::from(FRAPPE_TEAL)),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(
            -40.0, -22.0, 0.0,
        )))
        .id();

    let eight = commands
        // info
        // ----
        .spawn(Name::new("RopeEight"))
        // physics
        // -------
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(RAD))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(ColliderMassProperties::Density(5.0))
        .insert(Damping {
            linear_damping: 2.0,
            angular_damping: 1.0,
        })
        .insert(GravityScale(-4.0))
        .insert(ImpulseJoint::new(seven, joint))
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(RAD).into()).into(),
            material: materials.add(ColorMaterial::from(FRAPPE_GREEN)),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(
            -40.0, -19.0, 0.0,
        )))
        .id();

    let nine = commands
        // info
        // ----
        .spawn(Name::new("RopeNine"))
        // physics
        // -------
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(RAD))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(ColliderMassProperties::Density(5.0))
        .insert(Damping {
            linear_damping: 2.0,
            angular_damping: 1.0,
        })
        .insert(GravityScale(-4.0))
        .insert(ImpulseJoint::new(eight, joint))
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(RAD).into()).into(),
            material: materials.add(ColorMaterial::from(FRAPPE_YELLOW)),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(
            -40.0, -16.0, 0.0,
        )))
        .id();

    let ten = commands
        // info
        // ----
        .spawn(Name::new("RopeTen"))
        // physics
        // -------
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(RAD))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(ColliderMassProperties::Density(5.0))
        .insert(Damping {
            linear_damping: 2.0,
            angular_damping: 1.0,
        })
        .insert(GravityScale(-4.0))
        .insert(ImpulseJoint::new(nine, joint))
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(RAD).into()).into(),
            material: materials.add(ColorMaterial::from(FRAPPE_PEACH)),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(
            -40.0, -13.0, 0.0,
        )))
        .id();

    let eleven = commands
        // info
        // ----
        .spawn(Name::new("RopeEleven"))
        // physics
        // -------
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(RAD))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(ColliderMassProperties::Density(5.0))
        .insert(Damping {
            linear_damping: 2.0,
            angular_damping: 1.0,
        })
        .insert(GravityScale(-4.0))
        .insert(ImpulseJoint::new(ten, joint))
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(RAD).into()).into(),
            material: materials.add(ColorMaterial::from(FRAPPE_MAROON)),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(
            -40.0, -10.0, 0.0,
        )))
        .id();

    let twelve = commands
        // info
        // ----
        .spawn(Name::new("RopeTwelve"))
        // physics
        // -------
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(RAD))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(ColliderMassProperties::Density(5.0))
        .insert(Damping {
            linear_damping: 2.0,
            angular_damping: 1.0,
        })
        .insert(GravityScale(-4.0))
        .insert(ImpulseJoint::new(eleven, joint))
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(RAD).into()).into(),
            material: materials.add(ColorMaterial::from(FRAPPE_RED)),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(-40.0, -7.0, 0.0)))
        .id();

    let thirteen = commands
        // info
        // ----
        .spawn(Name::new("RopeThirteen"))
        // physics
        // -------
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(RAD))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(ColliderMassProperties::Density(5.0))
        .insert(Damping {
            linear_damping: 2.0,
            angular_damping: 1.0,
        })
        .insert(GravityScale(-4.0))
        .insert(ImpulseJoint::new(twelve, joint))
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(RAD).into()).into(),
            material: materials.add(ColorMaterial::from(FRAPPE_MAUVE)),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(-40.0, -4.0, 0.0)))
        .id();

    let fourteen = commands
        // info
        // ----
        .spawn(Name::new("RopeFourteen"))
        // physics
        // -------
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(RAD))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(ColliderMassProperties::Density(5.0))
        .insert(Damping {
            linear_damping: 2.0,
            angular_damping: 1.0,
        })
        .insert(GravityScale(-4.0))
        .insert(ImpulseJoint::new(thirteen, joint))
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(RAD).into()).into(),
            material: materials.add(ColorMaterial::from(FRAPPE_PINK)),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(-40.0, -1.0, 0.0)))
        .id();

    let fifteen = commands
        // info
        // ----
        .spawn(Name::new("RopeFifteen"))
        // physics
        // -------
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(RAD))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(ColliderMassProperties::Density(5.0))
        .insert(Damping {
            linear_damping: 2.0,
            angular_damping: 1.0,
        })
        .insert(GravityScale(-4.0))
        .insert(ImpulseJoint::new(fourteen, joint))
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(RAD).into()).into(),
            material: materials.add(ColorMaterial::from(FRAPPE_FLAMINGO)),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(-40.0, 2.0, 0.0)))
        .id();

    commands
        // info
        // ----
        .spawn(Name::new("RopeSixteen"))
        // physics
        // -------
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(RAD))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(ColliderMassProperties::Density(5.0))
        .insert(Damping {
            linear_damping: 2.0,
            angular_damping: 1.0,
        })
        .insert(GravityScale(-4.0))
        .insert(ImpulseJoint::new(fifteen, joint))
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(RAD).into()).into(),
            material: materials.add(ColorMaterial::from(FRAPPE_ROSEWATER)),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(-40.0, 5.0, 0.0)));
}
