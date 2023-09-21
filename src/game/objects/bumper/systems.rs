use crate::style::FRAPPE_PEACH;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_bumper(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        // info
        // ----
        .spawn(Name::new("Bumper"))
        // physics
        // -------
        .insert(RigidBody::Fixed)
        .insert(Collider::ball(7.5))
        .insert(Restitution::coefficient(5.0))
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(7.5).into()).into(),
            material: materials.add(ColorMaterial::from(FRAPPE_PEACH)),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 40.0, 0.0)));
}
