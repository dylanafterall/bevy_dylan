use crate::style::FRAPPE_SURFACE2;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_platform(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        // info
        // ----
        .spawn(Name::new("Platform"))
        // physics
        // -------
        .insert(Collider::cuboid(50.0, 5.0))
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(100.0, 10.0)).into())
                .into(),
            material: materials.add(ColorMaterial::from(FRAPPE_SURFACE2)),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -15.0, 0.0)));
}
