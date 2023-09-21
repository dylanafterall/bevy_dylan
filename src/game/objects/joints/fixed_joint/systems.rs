use super::super::components::*;

use crate::game::collision_manager::events::DestructibleContact;
use crate::style::{FRAPPE_OVERLAY2, FRAPPE_SURFACE2};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::{plugin::RapierContext, prelude::*};

// -----------------------------------------------------------------------------
pub fn spawn_fixed_joint(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // adjust RADIUS and CENTER for scaling and translating --------------------
    const RADIUS: f32 = 4.5;
    const CENTER: Vec3 = Vec3::new(0.0, -30.0, 0.0);

    // consts derived from RADIUS and CENTER -----------------------------------
    const DIAMETER: f32 = 2.0 * RADIUS;
    const TOP_LEFT: Vec3 = Vec3::new(CENTER.x - RADIUS, CENTER.y + RADIUS, CENTER.z);
    const TOP_RIGHT: Vec3 = Vec3::new(CENTER.x + RADIUS, CENTER.y + RADIUS, CENTER.z);
    const BOTTOM_LEFT: Vec3 = Vec3::new(CENTER.x - RADIUS, CENTER.y - RADIUS, CENTER.z);
    const BOTTOM_RIGHT: Vec3 = Vec3::new(CENTER.x + RADIUS, CENTER.y - RADIUS, CENTER.z);

    // joint builders ----------------------------------------------------------
    // top left to top right
    let fixed_joint_top = FixedJointBuilder::new().local_anchor1(Vec2::new(DIAMETER, 0.0));

    // top right to bottom right
    let fixed_joint_right = FixedJointBuilder::new().local_anchor1(Vec2::new(0.0, -DIAMETER));

    // bottom right to bottom left
    let fixed_joint_bottom = FixedJointBuilder::new().local_anchor1(Vec2::new(-DIAMETER, 0.0));

    // bottom left to top left
    let fixed_joint_left = FixedJointBuilder::new().local_anchor1(Vec2::new(0.0, DIAMETER));

    // entities ----------------------------------------------------------------
    let top_left_entity = commands
        // info
        // ----
        .spawn(Name::new("TopLeftBrick"))
        .insert(Destructible)
        // physics
        // -------
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(4.0, 4.0))
        .insert(ActiveEvents::CONTACT_FORCE_EVENTS)
        .insert(ContactForceEventThreshold(1.5))
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(8.0, 8.0)).into())
                .into(),
            material: materials.add(ColorMaterial::from(FRAPPE_SURFACE2)),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(
            TOP_LEFT.x, TOP_LEFT.y, TOP_LEFT.z,
        )))
        .id();

    let top_right_entity = commands
        // info
        // ----
        .spawn(Name::new("TopRightBrick"))
        .insert(Destructible)
        // physics
        // -------
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(4.0, 4.0))
        .insert(ActiveEvents::CONTACT_FORCE_EVENTS)
        .insert(ContactForceEventThreshold(1.5))
        .insert(ImpulseJoint::new(top_left_entity, fixed_joint_top))
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(8.0, 8.0)).into())
                .into(),
            material: materials.add(ColorMaterial::from(FRAPPE_OVERLAY2)),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(
            TOP_RIGHT.x,
            TOP_RIGHT.y,
            TOP_RIGHT.z,
        )))
        .id();

    let bottom_right_entity = commands
        // info
        // ----
        .spawn(Name::new("BottomRightBrick"))
        .insert(Destructible)
        // physics
        // -------
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(4.0, 4.0))
        .insert(ActiveEvents::CONTACT_FORCE_EVENTS)
        .insert(ContactForceEventThreshold(1.5))
        .insert(ImpulseJoint::new(top_right_entity, fixed_joint_right))
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(8.0, 8.0)).into())
                .into(),
            material: materials.add(ColorMaterial::from(FRAPPE_OVERLAY2)),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(
            BOTTOM_RIGHT.x,
            BOTTOM_RIGHT.y,
            BOTTOM_RIGHT.z,
        )))
        .id();

    let bottom_left_entity = commands
        // info
        // ----
        .spawn(Name::new("BottomLeftBrick"))
        .insert(Destructible)
        // physics
        // -------
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(4.0, 4.0))
        .insert(ActiveEvents::CONTACT_FORCE_EVENTS)
        .insert(ContactForceEventThreshold(1.5))
        .insert(ImpulseJoint::new(bottom_right_entity, fixed_joint_bottom))
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(8.0, 8.0)).into())
                .into(),
            material: materials.add(ColorMaterial::from(FRAPPE_OVERLAY2)),
            ..default()
        })
        // transform
        // ---------
        .insert(TransformBundle::from(Transform::from_xyz(
            BOTTOM_LEFT.x,
            BOTTOM_LEFT.y,
            BOTTOM_LEFT.z,
        )))
        .id();

    commands
        .entity(top_left_entity)
        .insert(ImpulseJoint::new(bottom_left_entity, fixed_joint_left))
        .insert(Destructible);
}

pub fn handle_destructible_contact(
    mut rapier_context: ResMut<RapierContext>,
    mut destructible_listener: EventReader<DestructibleContact>,
) {
    for contact in destructible_listener.iter() {
        let rapier_body_handle = contact.destructible;
        let body_handle = rapier_body_handle.0;
        rapier_context
            .impulse_joints
            .remove_joints_attached_to_rigid_body(body_handle);
    }
}
