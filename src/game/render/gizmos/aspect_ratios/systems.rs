use bevy::prelude::*;

use crate::style::{LATTE_BLUE, LATTE_GREEN, LATTE_RED};

// -----------------------------------------------------------------------------
pub fn spawn_aspect_ratio_gizmos(mut gizmos: Gizmos) {
    // 4:3 --- 256:192
    gizmos.rect_2d(Vec2::ZERO, 0.0, Vec2::new(100.0, 40.0), LATTE_RED);

    // 5:4 --- 240:192
    gizmos.rect_2d(Vec2::ZERO, 0.0, Vec2::new(200.0, 80.0), LATTE_BLUE);

    // 8:5 --- 256:160
    gizmos.rect_2d(Vec2::ZERO, 0.0, Vec2::new(300.0, 120.0), LATTE_GREEN);

    // 16:9 --- 256:144
    // gizmos.rect_2d(Vec2::ZERO, 0.0, Vec2::new(256.0, 144.0), LATTE_YELLOW);

    // 21:9 --- 336:144
    // gizmos.rect_2d(Vec2::ZERO, 0.0, Vec2::new(336.0, 144.0), LATTE_RED);

    // 32:9 --- 512:144
    // gizmos.rect_2d(Vec2::ZERO, 0.0, Vec2::new(512.0, 144.0), LATTE_RED);
}
