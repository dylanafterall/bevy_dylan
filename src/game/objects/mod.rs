pub mod components;

mod ball;
mod heightfield;
mod platform;
mod round_hull;
mod star;

use crate::game::objects::ball::BallPlugin;
use crate::game::objects::heightfield::HeightfieldPlugin;
use crate::game::objects::platform::PlatformPlugin;
use crate::game::objects::round_hull::RoundHullPlugin;
use crate::game::objects::star::StarPlugin;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct ObjectsPlugin;

impl Plugin for ObjectsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                BallPlugin,
                HeightfieldPlugin,
                PlatformPlugin,
                RoundHullPlugin,
                StarPlugin,
            ));
    }
}