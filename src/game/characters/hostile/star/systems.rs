use crate::game::characters::components::*;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_star(mut commands: Commands) {
    let shape1 = Collider::convex_polyline(vec![
        Vec2::new(12.24, -16.85),
        Vec2::new(47.55, -15.45),
        Vec2::new(19.81, 6.44),
    ]);
    let shape2 = Collider::convex_polyline(vec![
        Vec2::new(-19.81, 6.44),
        Vec2::new(-47.55, -15.45),
        Vec2::new(-12.24, -16.85),
    ]);
    let shape3 = Collider::convex_polyline(vec![
        Vec2::new(-0.00, 20.83),
        Vec2::new(-29.39, 40.45),
        Vec2::new(-19.81, 6.44),
        Vec2::new(-12.24, -16.85),
        Vec2::new(0.00, -50.00),
    ]);
    let shape4 = Collider::convex_polyline(vec![
        Vec2::new(12.24, -16.85),
        Vec2::new(19.81, 6.44),
        Vec2::new(29.39, 40.45),
        Vec2::new(-0.00, 20.83),
        Vec2::new(0.00, -50.00),
    ]);
    commands
        .spawn(HostileCharacter)
        .insert(RigidBody::Dynamic)
        .insert(Collider::compound(vec![
            (Vec2::new(0.0, 0.0), 0.0, shape1.unwrap()),
            (Vec2::new(0.0, 0.0), 0.0, shape2.unwrap()),
            (Vec2::new(0.0, 0.0), 0.0, shape3.unwrap()),
            (Vec2::new(0.0, 0.0), 0.0, shape4.unwrap()),
        ]))
        .insert(Restitution::coefficient(0.5))
        .insert(TransformBundle::from(Transform::from_xyz(200.0, 400.0, 0.0)));
}