mod heightfield;
mod joints;
mod platform;

use heightfield::HeightfieldPlugin;
use joints::JointsPlugin;
use platform::PlatformPlugin;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct ObjectsPlugin;

impl Plugin for ObjectsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                HeightfieldPlugin,
                JointsPlugin,
                PlatformPlugin,
            ));
    }
}