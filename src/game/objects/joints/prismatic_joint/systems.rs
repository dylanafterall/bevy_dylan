use crate::style::{FRAPPE_GREEN, FRAPPE_SURFACE2};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_prismatic_joint(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let joint = PrismaticJointBuilder::new(Vec2::Y)
        .local_anchor1(Vec2::new(30.0, 0.0))
        .limits([-10.0, 10.0])
        .motor_position(-10.0, 50.0, 0.05);

    let parent_entity = commands
        // info
        // ----
        .spawn(Name::new("PrismaticHandle"))
        // physics
        // -------
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(5.0, 5.0))
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(10.0, 10.0)).into())
                .into(),
            material: materials.add(ColorMaterial::from(FRAPPE_SURFACE2)),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(
            -100.0, 25.0, 0.0,
        )))
        .id();

    commands
        // info
        // ----
        .spawn(Name::new("PrismaticSpring"))
        // physics
        // -------
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(5.0))
        .insert(ImpulseJoint::new(parent_entity, joint))
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(5.0).into()).into(),
            material: materials.add(ColorMaterial::from(FRAPPE_GREEN)),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(-70.0, 25.0, 0.0)));
}
