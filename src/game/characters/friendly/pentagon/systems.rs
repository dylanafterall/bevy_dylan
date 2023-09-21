use crate::game::characters::components::*;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::style::FRAPPE_TEAL;
use bevy_rapier2d::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_pentagon(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let pentagon_vertices = vec![
        Vec2::new(0.0, 5.0),
        Vec2::new(-4.76, 1.55),
        Vec2::new(-2.94, -4.05),
        Vec2::new(2.94, -4.05),
        Vec2::new(4.76, 1.55),
    ];

    let mut rounded_collider = Collider::round_convex_polyline(pentagon_vertices, 0.0)
        .expect("Collider vertices are hard-coded, no error expected.");
    rounded_collider.set_scale(Vec2::new(1.0, 1.0), 0);
    rounded_collider.promote_scaled_shape();

    commands
        // info
        // ----
        .spawn(Name::new("Pentagon"))
        .insert(NPC::Friendly)
        // physics
        // -------
        .insert(RigidBody::Dynamic)
        .insert(rounded_collider)
        .insert(Restitution::coefficient(0.7))
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::RegularPolygon::new(5.0, 5).into()).into(),
            material: materials.add(ColorMaterial::from(FRAPPE_TEAL)),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(-40.0, 40.0, 0.0)));
}
