use crate::game::characters::player::components::Player;
use crate::style::{LATTE_FLAMINGO, LATTE_MAUVE, LATTE_PINK, LATTE_ROSEWATER};
use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_circles(mut gizmos: Gizmos, player_query: Query<&Transform, With<Player>>) {
    let player_transform = player_query.single();
    let player_pos = Vec2::new(
        player_transform.translation.x,
        player_transform.translation.y,
    );

    gizmos.circle_2d(player_pos, 9.0, LATTE_MAUVE);
    gizmos.circle_2d(player_pos, 10.0, LATTE_PINK);
    gizmos
        .circle_2d(player_pos, 11.0, LATTE_FLAMINGO)
        .segments(64);
    gizmos
        .circle_2d(player_pos, 12.0, LATTE_ROSEWATER)
        .segments(64);
}
