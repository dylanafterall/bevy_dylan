use crate::style::{LATTE_OVERLAY0, LATTE_SUBTEXT0, LATTE_SURFACE0, LATTE_TEXT};
use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub fn spawn_rects(mut gizmos: Gizmos, time: Res<Time>) {
    gizmos.rect_2d(
        Vec2::new(-55.0, -50.0),
        -time.elapsed_seconds() / 3.0,
        Vec2::splat(12.5),
        LATTE_TEXT,
    );

    gizmos.rect_2d(
        Vec2::new(-35.0, -30.0),
        -time.elapsed_seconds() / 0.25,
        Vec2::splat(10.0),
        LATTE_SUBTEXT0,
    );

    gizmos.rect_2d(
        Vec2::new(-55.0, -30.0),
        time.elapsed_seconds(),
        Vec2::splat(7.5),
        LATTE_OVERLAY0,
    );

    gizmos.rect_2d(
        Vec2::new(-35.0, -50.0),
        time.elapsed_seconds() / 0.5,
        Vec2::splat(5.0),
        LATTE_SURFACE0,
    );
}
