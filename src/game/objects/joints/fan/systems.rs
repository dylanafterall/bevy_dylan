use crate::style::{
    FRAPPE_FLAMINGO, FRAPPE_MAROON, FRAPPE_PINK, FRAPPE_ROSEWATER, FRAPPE_SURFACE2,
};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_fan(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let revolute_joint = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(-20.0, 0.0))
        .motor_velocity(2.0, 1.0);

    let fan = commands
        // info
        // ----
        .spawn(Name::new("FanCenter"))
        // physics
        // -------
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(0.1))
        .insert(Sensor)
        .insert(GravityScale(0.0))
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(0.1).into()).into(),
            material: materials.add(ColorMaterial::from(Color::rgba(0.0, 0.0, 0.0, 0.0))),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(60.0, -40.0, 0.0)))
        // children
        // --------
        .with_children(|children| {
            children
                // info
                // ----
                .spawn(Name::new("FanTopLeft"))
                // physics
                // -------
                .insert(Collider::ball(5.0))
                // rendering
                // ---------
                .insert(MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(5.0).into()).into(),
                    material: materials.add(ColorMaterial::from(FRAPPE_ROSEWATER)),
                    ..default()
                })
                // transform
                // ---------
                .insert(TransformBundle::from(Transform::from_xyz(-12.5, 12.5, 0.0)));
            children
                // info
                // ----
                .spawn(Name::new("FanTopRight"))
                // physics
                // -------
                .insert(Collider::ball(5.0))
                // rendering
                // ---------
                .insert(MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(5.0).into()).into(),
                    material: materials.add(ColorMaterial::from(FRAPPE_FLAMINGO)),
                    ..default()
                })
                // transform
                // ---------
                .insert(TransformBundle::from(Transform::from_xyz(12.5, 12.5, 0.0)));
            children
                // info
                // ----
                .spawn(Name::new("FanBottomRight"))
                // physics
                // -------
                .insert(Collider::ball(5.0))
                // rendering
                // ---------
                .insert(MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(5.0).into()).into(),
                    material: materials.add(ColorMaterial::from(FRAPPE_PINK)),
                    ..default()
                })
                // transform
                // ---------
                .insert(TransformBundle::from(Transform::from_xyz(12.5, -12.5, 0.0)));
            children
                // info
                // ----
                .spawn(Name::new("FanBottomLeft"))
                // physics
                // -------
                .insert(Collider::ball(5.0))
                // rendering
                // ---------
                .insert(MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(5.0).into()).into(),
                    material: materials.add(ColorMaterial::from(FRAPPE_MAROON)),
                    ..default()
                })
                // transform
                // ---------
                .insert(TransformBundle::from(Transform::from_xyz(
                    -12.5, -12.5, 0.0,
                )));
        })
        .id();

    commands
        // info
        // ----
        .spawn(Name::new("FanHandle"))
        // physics
        // -------
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(2.5, 2.5))
        .insert(GravityScale(0.0))
        .insert(ImpulseJoint::new(fan, revolute_joint))
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(5.0, 5.0)).into())
                .into(),
            material: materials.add(ColorMaterial::from(FRAPPE_SURFACE2)),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(40.0, -40.0, 0.0)));
}
