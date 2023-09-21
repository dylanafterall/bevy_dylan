use crate::system_params::*;

use crate::style::{FRAPPE_CRUST, FRAPPE_SURFACE2, FRAPPE_YELLOW};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_conveyor_belt(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        // info
        // ----
        .spawn(Name::new("BeltTestCube"))
        // physics
        // -------
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(5.0, 5.0))
        .insert(Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0,
        })
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(10.0, 10.0)).into())
                .into(),
            material: materials.add(ColorMaterial::from(FRAPPE_YELLOW)),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(
            -19.0, -40.0, 0.0,
        )));

    commands
        // info
        // ----
        .spawn(Name::new("BeltRetainerWall"))
        // physics
        // -------
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(2.5, 6.0))
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(5.0, 12.0)).into())
                .into(),
            material: materials.add(ColorMaterial::from(FRAPPE_SURFACE2)),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(43.5, -56.5, 0.0)));

    commands
        // info
        // ----
        .spawn(Name::new("BeltContainer"))
        // physics
        // -------
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(41.0, 2.5))
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(82.0, 5.0)).into())
                .into(),
            material: materials.add(ColorMaterial::from(FRAPPE_SURFACE2)),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -60.0, 0.0)));

    commands
        // info
        // ----
        .spawn(Name::new("Belt"))
        .insert(ConveyorBelt)
        // physics
        // -------
        .insert(RigidBody::KinematicPositionBased)
        .insert(ActiveHooks::MODIFY_SOLVER_CONTACTS)
        .insert(Collider::cuboid(40.0, 0.5))
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(80.0, 1.0)).into())
                .into(),
            material: materials.add(ColorMaterial::from(FRAPPE_CRUST)),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -57.0, 0.0)));
}
