use crate::game::characters::components::*;

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_rainbow_square(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut mesh = Mesh::from(shape::Quad::new(Vec2::new(10.0, 10.0)));

    let vertex_colors: Vec<[f32; 4]> = vec![
        Color::WHITE.as_rgba_f32(),
        Color::BLUE.as_rgba_f32(),
        Color::GREEN.as_rgba_f32(),
        Color::RED.as_rgba_f32(),
    ];

    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, vertex_colors);

    let mesh_handle: Mesh2dHandle = meshes.add(mesh).into();

    commands
        // info
        // ----
        .spawn(Name::new("RainbowSquare"))
        .insert(NPC::Hostile)
        // physics
        // -------
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(5.0, 5.0))
        .insert(Restitution::coefficient(0.7))
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: mesh_handle,
            material: materials.add(ColorMaterial::default()),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(40.0, 40.0, 0.0)));
}
