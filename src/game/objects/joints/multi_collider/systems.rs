use crate::style::{FRAPPE_BLUE, FRAPPE_LAVENDER, FRAPPE_SAPPHIRE, FRAPPE_SKY};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_multi_collider(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        // info
        // ----
        .spawn(Name::new("MultiColliderCenter"))
        // physics
        // -------
        .insert(RigidBody::Dynamic)
        .insert(GravityScale(0.0))
        .insert(Damping {
            linear_damping: 0.5,
            angular_damping: 0.5,
        })
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(0.1).into()).into(),
            material: materials.add(ColorMaterial::from(Color::rgba(0.0, 0.0, 0.0, 0.0))),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(
            -40.0, -40.0, 0.0,
        )))
        // children
        // --------
        .with_children(|children| {
            children
                // info
                // ----
                .spawn(Name::new("MultiColliderTopLeft"))
                // physics
                // -------
                .insert(Collider::ball(5.0))
                // rendering
                // ---------
                .insert(MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(5.0).into()).into(),
                    material: materials.add(ColorMaterial::from(FRAPPE_SKY)),
                    ..default()
                })
                // transform
                // ---------
                .insert(TransformBundle::from(Transform::from_xyz(-15.0, 15.0, 0.0)));
            children
                // info
                // ----
                .spawn(Name::new("MultiColliderTopRight"))
                // physics
                // -------
                .insert(Collider::ball(5.0))
                // rendering
                // ---------
                .insert(MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(5.0).into()).into(),
                    material: materials.add(ColorMaterial::from(FRAPPE_SAPPHIRE)),
                    ..default()
                })
                // transform
                // ---------
                .insert(TransformBundle::from(Transform::from_xyz(15.0, 15.0, 0.0)));
            children
                // info
                // ----
                .spawn(Name::new("MultiColliderBottomRight"))
                // physics
                // -------
                .insert(Collider::ball(5.0))
                // rendering
                // ---------
                .insert(MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(5.0).into()).into(),
                    material: materials.add(ColorMaterial::from(FRAPPE_BLUE)),
                    ..default()
                })
                // transform
                // ---------
                .insert(TransformBundle::from(Transform::from_xyz(15.0, -15.0, 0.0)));
            children
                // info
                // ----
                .spawn(Name::new("MultiColliderBottomLeft"))
                // physics
                // -------
                .insert(Collider::ball(5.0))
                // rendering
                // ---------
                .insert(MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(5.0).into()).into(),
                    material: materials.add(ColorMaterial::from(FRAPPE_LAVENDER)),
                    ..default()
                })
                // transform
                // ---------
                .insert(TransformBundle::from(Transform::from_xyz(
                    -15.0, -15.0, 0.0,
                )));
        });
}
