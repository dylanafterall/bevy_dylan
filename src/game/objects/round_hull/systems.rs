use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_round_hull(mut commands: Commands) {
    let sharp_collider_shape = Collider::convex_polyline(vec![
        Vec2::new(-60.0, -50.0),
        Vec2::new(60.0, -50.0),
        Vec2::new(40.0, 50.0),
        Vec2::new(-40.0, 50.0),
    ]);

    let rounded_collider_shape = Collider::round_convex_polyline(
        vec![
            Vec2::new(-60.0, -50.0),
            Vec2::new(60.0, -50.0),
            Vec2::new(40.0, 50.0),
            Vec2::new(-40.0, 50.0),
        ],
        0.1,
        );

    commands
        .spawn(RigidBody::Dynamic)
        .insert(sharp_collider_shape.unwrap())
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(400.0, 400.0, 0.0)));

    commands
        .spawn(RigidBody::Dynamic)
        .insert(rounded_collider_shape.unwrap())
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(-400.0, 400.0, 0.0)));
}