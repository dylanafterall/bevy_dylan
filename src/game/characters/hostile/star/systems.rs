use crate::game::characters::components::*;

use crate::game::characters::player::carry::components::Carryable;
use crate::style::FRAPPE_RED;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_star(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // render setup ------------------------------------------------------------
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [0.00, -5.000, 0.0],
            [-1.224, -1.685, 0.0],
            [-4.755, -1.545, 0.0],
            [-1.981, 0.644, 0.0],
            [-2.939, 4.045, 0.0],
            [-0.00, 2.083, 0.0],
            [2.939, 4.045, 0.0],
            [1.981, 0.644, 0.0],
            [4.755, -1.545, 0.0],
            [1.224, -1.685, 0.0],
        ],
    );
    mesh.set_indices(Some(Indices::U32(vec![
        5, 6, 7, 5, 3, 4, 5, 7, 9, 3, 5, 1, 1, 2, 3, 7, 8, 9, 1, 5, 9, 9, 0, 1,
    ])));

    // physics setup -----------------------------------------------------------
    let compound1 = vec![
        Vec2::new(1.224, -1.685),
        Vec2::new(4.755, -1.545),
        Vec2::new(1.981, 0.644),
    ];
    let compound2 = vec![
        Vec2::new(-1.981, 0.644),
        Vec2::new(-4.755, -1.545),
        Vec2::new(-1.224, -1.685),
    ];
    let compound3 = vec![
        Vec2::new(-0.00, 2.083),
        Vec2::new(-2.939, 4.045),
        Vec2::new(-1.981, 0.644),
        Vec2::new(-1.224, -1.685),
        Vec2::new(0.00, -5.000),
    ];
    let compound4 = vec![
        Vec2::new(1.224, -1.685),
        Vec2::new(1.981, 0.644),
        Vec2::new(2.939, 4.045),
        Vec2::new(-0.00, 2.083),
        Vec2::new(0.00, -5.000),
    ];

    let compound1_collider = Collider::convex_polyline(compound1)
        .expect("Collider vertices are hard-coded, no error expected.");
    let compound2_collider = Collider::convex_polyline(compound2)
        .expect("Collider vertices are hard-coded, no error expected.");
    let compound3_collider = Collider::convex_polyline(compound3)
        .expect("Collider vertices are hard-coded, no error expected.");
    let compound4_collider = Collider::convex_polyline(compound4)
        .expect("Collider vertices are hard-coded, no error expected.");

    // spawn -------------------------------------------------------------------
    commands
        // info
        // ----
        .spawn(Name::new("Star"))
        .insert(NPC::Hostile)
        .insert(Carryable)
        // physics
        // -------
        .insert(RigidBody::Dynamic)
        .insert(Collider::compound(vec![
            (Vec2::new(0.0, 0.0), 0.0, compound1_collider),
            (Vec2::new(0.0, 0.0), 0.0, compound2_collider),
            (Vec2::new(0.0, 0.0), 0.0, compound3_collider),
            (Vec2::new(0.0, 0.0), 0.0, compound4_collider),
        ]))
        .insert(Restitution::coefficient(0.5))
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(mesh)).into(),
            material: materials.add(ColorMaterial::from(FRAPPE_RED)),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(20.0, 40.0, 0.0)));
}
