use crate::style::FRAPPE_CRUST;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_bucket(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        // info
        // ----
        .spawn(Name::new("Bucket"))
        // physics
        // -------
        .insert(Collider::segment(
            Vec2::new(-100.0, -60.0),
            Vec2::new(100.0, -60.0),
        ))
        // children
        // --------
        .with_children(|children| {
            // left wall
            children.spawn(Collider::segment(
                Vec2::new(-100.0, -60.0),
                Vec2::new(-100.0, 15.0),
            ));
            // right wall
            children.spawn(Collider::segment(
                Vec2::new(100.0, -60.0),
                Vec2::new(100.0, 15.0),
            ));
        });

    commands
        // info
        // ----
        .spawn(Name::new("BucketBottom"))
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(204.0, 2.0)).into())
                .into(),
            transform: Transform::from_translation(Vec3::new(0.0, -61.0, 0.0)),
            material: materials.add(ColorMaterial::from(FRAPPE_CRUST)),
            ..default()
        });

    commands
        // info
        // ----
        .spawn(Name::new("BucketLeft"))
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(2.0, 75.0)).into())
                .into(),
            transform: Transform::from_translation(Vec3::new(-101.0, -22.5, 0.0)),
            material: materials.add(ColorMaterial::from(FRAPPE_CRUST)),
            ..default()
        });

    commands
        // info
        // ----
        .spawn(Name::new("BucketRight"))
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(2.0, 75.0)).into())
                .into(),
            transform: Transform::from_translation(Vec3::new(101.0, -22.5, 0.0)),
            material: materials.add(ColorMaterial::from(FRAPPE_CRUST)),
            ..default()
        });
}
