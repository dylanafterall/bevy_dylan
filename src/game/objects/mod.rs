mod ball;
mod platform;
mod sensor;
mod star;

use crate::game::objects::ball::BallPlugin;
use crate::game::objects::platform::PlatformPlugin;
use crate::game::objects::sensor::SensorPlugin;
use crate::game::objects::star::StarPlugin;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct ObjectsPlugin;

impl Plugin for ObjectsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                BallPlugin,
                PlatformPlugin,
                SensorPlugin,
                StarPlugin,
            ));
    }
}