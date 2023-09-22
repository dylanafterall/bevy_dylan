use crate::game::characters::player::components::Player;
use crate::style::{LATTE_CRUST, LATTE_GREEN, LATTE_PEACH, LATTE_TEAL};
use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_rays(
    mut gizmos: Gizmos,
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
) {
    // setup
    // -----
    let sin = time.elapsed_seconds().sin();

    let player_transform = player_query.single();
    let player_pos = Vec2::new(
        player_transform.translation.x,
        player_transform.translation.y,
    );

    // gizmos
    // ------
    // expanding/contracting cross
    gizmos.ray_2d(
        Vec2::new(-40.0, 30.0),
        Vec2::new(15.0, 0.0) * sin,
        LATTE_PEACH,
    );
    gizmos.ray_2d(
        Vec2::new(-40.0, 30.0),
        -Vec2::new(15.0, 0.0) * sin,
        LATTE_TEAL,
    );
    gizmos.ray_2d(
        Vec2::new(-40.0, 30.0),
        Vec2::new(0.0, 15.0) * sin,
        LATTE_PEACH,
    );
    gizmos.ray_2d(
        Vec2::new(-40.0, 30.0),
        -Vec2::new(0.0, 15.0) * sin,
        LATTE_TEAL,
    );

    // gradient
    gizmos.ray_gradient_2d(
        Vec2::new(0.0, 40.0),
        (player_pos - Vec2::new(0.0, 50.0)) * 0.25,
        LATTE_GREEN,
        LATTE_CRUST,
    );
}
