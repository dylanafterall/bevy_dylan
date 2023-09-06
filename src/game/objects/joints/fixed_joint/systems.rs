use super::super::components::*;

use bevy::prelude::*;
use bevy_rapier2d::{
    prelude::*,
    plugin::RapierContext,
};
use crate::game::collision_manager::events::DestructibleContact;

// -----------------------------------------------------------------------------
pub fn spawn_fixed_joint(
    mut commands: Commands
) {
    // adjust RADIUS and CENTER for scaling and translating --------------------
    const RADIUS :f32 = 150.0;
    const CENTER :Vec3 = Vec3::new(400.0, 400.0, 0.0);

    // consts derived from RADIUS and CENTER -----------------------------------
    const DIAMETER :f32 = 2.0 * RADIUS;
    const TOP_LEFT :Vec3 = Vec3::new(CENTER.x - RADIUS, CENTER.y + RADIUS, CENTER.z);
    const TOP_RIGHT :Vec3 = Vec3::new(CENTER.x + RADIUS, CENTER.y + RADIUS, CENTER.z);
    const BOTTOM_LEFT :Vec3 = Vec3::new(CENTER.x - RADIUS, CENTER.y - RADIUS, CENTER.z);
    const BOTTOM_RIGHT :Vec3 = Vec3::new(CENTER.x + RADIUS, CENTER.y - RADIUS, CENTER.z);

    // joint builders ----------------------------------------------------------
    // top left to top right
    let fixed_joint_top = FixedJointBuilder::new()
        .local_anchor1(Vec2::new(DIAMETER, 0.0));

    // top right to bottom right
    let fixed_joint_right = FixedJointBuilder::new()
        .local_anchor1(Vec2::new(0.0, -DIAMETER));

    // bottom right to bottom left
    let fixed_joint_bottom = FixedJointBuilder::new()
    .local_anchor1(Vec2::new(-DIAMETER, 0.0));

    // bottom left to top left
    let fixed_joint_left = FixedJointBuilder::new()
        .local_anchor1(Vec2::new(0.0, DIAMETER));

    // bodies ------------------------------------------------------------------
    let top_left_entity = commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(50.0, 50.0))
        .insert(ActiveEvents::CONTACT_FORCE_EVENTS)
        .insert(ContactForceEventThreshold(400.0))
        .insert(GravityScale(0.0))
        .insert(TransformBundle::from(Transform::from_xyz(
            TOP_LEFT.x, TOP_LEFT.y, TOP_LEFT.z
        )))
        .id();

    let top_right_entity = commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::cuboid(50.0, 50.0))
        .insert(ActiveEvents::CONTACT_FORCE_EVENTS)
        .insert(ContactForceEventThreshold(400.0))
        .insert(GravityScale(0.0))
        .insert(TransformBundle::from(Transform::from_xyz(
            TOP_RIGHT.x, TOP_RIGHT.y, TOP_RIGHT.z
        )))
        .insert(ImpulseJoint::new(top_left_entity, fixed_joint_top))
        .insert(Destructible)
        .id();

    let bottom_right_entity = commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::cuboid(50.0, 50.0))
        .insert(ActiveEvents::CONTACT_FORCE_EVENTS)
        .insert(ContactForceEventThreshold(400.0))
        .insert(GravityScale(0.0))
        .insert(TransformBundle::from(Transform::from_xyz(
            BOTTOM_RIGHT.x, BOTTOM_RIGHT.y, BOTTOM_RIGHT.z
        )))
        .insert(ImpulseJoint::new(top_right_entity, fixed_joint_right))
        .insert(Destructible)
        .id();

    let bottom_left_entity = commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::cuboid(50.0, 50.0))
        .insert(ActiveEvents::CONTACT_FORCE_EVENTS)
        .insert(ContactForceEventThreshold(400.0))
        .insert(GravityScale(0.0))
        .insert(TransformBundle::from(Transform::from_xyz(
            BOTTOM_LEFT.x, BOTTOM_LEFT.y, BOTTOM_LEFT.z
        )))
        .insert(ImpulseJoint::new(bottom_right_entity, fixed_joint_bottom))
        .insert(Destructible)
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
        rapier_context.impulse_joints.remove_joints_attached_to_rigid_body(body_handle);
    }
}