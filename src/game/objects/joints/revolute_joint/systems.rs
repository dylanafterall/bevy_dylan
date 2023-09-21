use crate::style::{FRAPPE_OVERLAY0, FRAPPE_OVERLAY1, FRAPPE_OVERLAY2, FRAPPE_SUBTEXT0};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_revolute_joint(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // adjust RADIUS and CENTER for scaling and translating --------------------
    const RADIUS: f32 = 10.0;
    const CENTER: Vec3 = Vec3::new(-90.0, -40.0, 0.0);

    // consts derived from RADIUS and CENTER -----------------------------------
    const DIAMETER: f32 = 2.0 * RADIUS;
    const TOP_LEFT: Vec3 = Vec3::new(CENTER.x - RADIUS, CENTER.y + RADIUS, CENTER.z);
    const TOP_RIGHT: Vec3 = Vec3::new(CENTER.x + RADIUS, CENTER.y + RADIUS, CENTER.z);
    const BOTTOM_LEFT: Vec3 = Vec3::new(CENTER.x - RADIUS, CENTER.y - RADIUS, CENTER.z);
    const BOTTOM_RIGHT: Vec3 = Vec3::new(CENTER.x + RADIUS, CENTER.y - RADIUS, CENTER.z);

    // joints ------------------------------------------------------------------
    // top left to top right
    let revolute_joint_top = RevoluteJointBuilder::new().local_anchor1(Vec2::new(DIAMETER, 0.0));

    // top right to bottom right
    let revolute_joint_right = RevoluteJointBuilder::new().local_anchor1(Vec2::new(0.0, -DIAMETER));

    // bottom right to bottom left
    let revolute_joint_bottom =
        RevoluteJointBuilder::new().local_anchor1(Vec2::new(-DIAMETER, 0.0));

    // bottom left to top left
    let revolute_joint_left = RevoluteJointBuilder::new().local_anchor1(Vec2::new(0.0, DIAMETER));

    // bodies ------------------------------------------------------------------
    let top_left_entity = commands
        // info
        // ----
        .spawn(Name::new("RevoluteTL"))
        // physics
        // -------
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(5.0))
        .insert(GravityScale(0.0))
        .insert(Damping {
            linear_damping: 0.5,
            angular_damping: 0.5,
        })
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(5.0).into()).into(),
            material: materials.add(ColorMaterial::from(FRAPPE_SUBTEXT0)),
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
        .spawn(Name::new("RevoluteTR"))
        // physics
        // -------
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(5.0))
        .insert(GravityScale(0.0))
        .insert(Damping {
            linear_damping: 0.5,
            angular_damping: 0.5,
        })
        .insert(ImpulseJoint::new(top_left_entity, revolute_joint_top))
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(5.0).into()).into(),
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
        .spawn(Name::new("RevoluteBR"))
        // physics
        // -------
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(5.0))
        .insert(GravityScale(0.0))
        .insert(Damping {
            linear_damping: 0.5,
            angular_damping: 0.5,
        })
        .insert(ImpulseJoint::new(top_right_entity, revolute_joint_right))
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(5.0).into()).into(),
            material: materials.add(ColorMaterial::from(FRAPPE_OVERLAY0)),
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
        .spawn(Name::new("RevoluteBL"))
        // physics
        // -------
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(5.0))
        .insert(GravityScale(0.0))
        .insert(Damping {
            linear_damping: 0.5,
            angular_damping: 0.5,
        })
        .insert(ImpulseJoint::new(
            bottom_right_entity,
            revolute_joint_bottom,
        ))
        // rendering
        // ---------
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(5.0).into()).into(),
            material: materials.add(ColorMaterial::from(FRAPPE_OVERLAY1)),
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
        .insert(ImpulseJoint::new(bottom_left_entity, revolute_joint_left));
}
