use crate::game::characters::components::*;

use crate::style::FRAPPE_MAUVE;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_bloom_triangle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        // info
        // ----
        .spawn(Name::new("Bloom"))
        .insert(NPC::Neutral)
        // physics
        // -------
        .insert(RigidBody::Dynamic)
        .insert(Collider::triangle(
            Vec2::new(0.0, 10.0),
            Vec2::new(-8.66, -5.0),
            Vec2::new(8.66, -5.0),
        ))
        .insert(LockedAxes::TRANSLATION_LOCKED)
        .insert(Damping {
            linear_damping: 0.0,
            angular_damping: 0.2,
        })
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::RegularPolygon::new(10.0, 3).into())
                .into(),
            material: materials.add(ColorMaterial::from(FRAPPE_MAUVE)),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(75.0, 25.0, 0.0)));
}
