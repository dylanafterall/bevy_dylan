use crate::game::characters::player::components::Player;
use crate::style::{
    LATTE_BASE, LATTE_CRUST, LATTE_GREEN, LATTE_LAVENDER, LATTE_MAUVE, LATTE_PEACH, LATTE_PINK,
    LATTE_RED, LATTE_SAPPHIRE, LATTE_YELLOW,
};
use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_lines(
    mut gizmos: Gizmos,
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
) {
    // setup
    // -----
    let sin = time.elapsed_seconds().sin() * 15.0;
    let cos = time.elapsed_seconds().cos() * 15.0;

    let player_transform = player_query.single();
    let player_pos = Vec2::new(
        player_transform.translation.x,
        player_transform.translation.y,
    );

    // gizmos
    // ------
    // rotating line
    gizmos.line_2d(
        Vec2::new(-40.0, 30.0),
        Vec2::new(sin - 40.0, cos + 30.0),
        LATTE_LAVENDER,
    );

    // line pointing to player
    gizmos.line_gradient_2d(Vec2::new(0.0, -60.0), player_pos, LATTE_GREEN, LATTE_CRUST);

    // stationary polygon
    let translation = Vec2::new(-100.0, -40.0);
    gizmos.linestrip_2d(
        [
            Vec2::new(0.0, -10.0) + translation,
            Vec2::new(-3.1, -3.1) + translation,
            Vec2::new(-10.0, 0.0) + translation,
            Vec2::new(-3.1, 3.1) + translation,
            Vec2::new(0.0, 10.0) + translation,
            Vec2::new(3.1, 3.1) + translation,
            Vec2::new(10.0, 0.0) + translation,
            Vec2::new(3.1, -3.1) + translation,
            Vec2::new(0.0, -10.0) + translation,
        ],
        LATTE_BASE,
    );

    // rotating, gradient polygon
    gizmos.linestrip_gradient_2d([
        (
            Vec2::new(sin + 0.0, cos + -15.0) + translation,
            LATTE_LAVENDER,
        ),
        (
            Vec2::new(sin + -4.65, cos + -4.65) + translation,
            LATTE_SAPPHIRE,
        ),
        (Vec2::new(sin + -15.0, cos + 0.0) + translation, LATTE_GREEN),
        (
            Vec2::new(sin + -4.65, cos + 4.65) + translation,
            LATTE_YELLOW,
        ),
        (Vec2::new(sin + 0.0, cos + 15.0) + translation, LATTE_PEACH),
        (Vec2::new(sin + 4.65, cos + 4.65) + translation, LATTE_RED),
        (Vec2::new(sin + 15.0, cos + 0.0) + translation, LATTE_MAUVE),
        (Vec2::new(sin + 4.65, cos + -4.65) + translation, LATTE_PINK),
        (
            Vec2::new(sin + 0.0, cos + -15.0) + translation,
            LATTE_LAVENDER,
        ),
    ]);
}
