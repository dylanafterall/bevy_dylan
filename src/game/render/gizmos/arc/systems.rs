use bevy::prelude::*;

use crate::style::{LATTE_BLUE, LATTE_RED, LATTE_YELLOW};
use std::f32::consts::PI;

// -----------------------------------------------------------------------------
pub fn spawn_arcs(mut gizmos: Gizmos, time: Res<Time>) {
    let sin = time.elapsed_seconds().sin() * 50.0;

    // outside arc
    gizmos.arc_2d(
        Vec2::new(-100.0, 30.0),
        &sin / 10.0,
        PI / 2.0,
        15.0,
        LATTE_RED,
    );

    // middle arc
    gizmos.arc_2d(
        Vec2::new(-100.0, 30.0),
        &sin / 7.0,
        PI / 2.0,
        12.5,
        LATTE_YELLOW,
    );

    // inside arc
    gizmos.arc_2d(
        Vec2::new(-100.0, 30.0),
        &sin / 5.0,
        PI / 2.0,
        10.0,
        LATTE_BLUE,
    );
}
