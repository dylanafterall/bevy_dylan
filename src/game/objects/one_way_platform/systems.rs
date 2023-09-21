use crate::system_params::*;

use crate::style::FRAPPE_MANTLE;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_one_way_platform(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        // info
        // ----
        .spawn(Name::new("OneWayPlatform"))
        .insert(OneWayPlatform)
        // physics
        // -------
        .insert(Collider::cuboid(15.0, 2.5))
        .insert(ActiveHooks::MODIFY_SOLVER_CONTACTS)
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(30.0, 5.0)).into())
                .into(),
            material: materials.add(ColorMaterial::from(FRAPPE_MANTLE)),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(-80.0, 25.0, 0.0)));
}
