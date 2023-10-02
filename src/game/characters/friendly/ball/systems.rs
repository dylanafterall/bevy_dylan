use crate::game::characters::components::*;

use crate::game::characters::player::carry::components::Carryable;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let texture_handle = asset_server.load("images/bevy_icon.png");

    commands
        // info
        // ----
        .spawn(Name::new("Ball"))
        .insert(NPC::Friendly)
        .insert(Carryable)
        // physics
        // -------
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(5.0))
        .insert(Restitution::coefficient(0.7))
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(5.0).into()).into(),
            material: materials.add(ColorMaterial::from(texture_handle)),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(-20.0, 40.0, 0.0)));
}
