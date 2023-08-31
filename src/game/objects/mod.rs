mod heightfield;
mod platform;

use crate::game::objects::heightfield::HeightfieldPlugin;
use crate::game::objects::platform::PlatformPlugin;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct ObjectsPlugin;

impl Plugin for ObjectsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                HeightfieldPlugin,
                PlatformPlugin,
            ));
    }
}