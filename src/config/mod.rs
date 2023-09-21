mod resources;

mod audio;
mod input;
mod window;

use audio::ConfigAudioPlugin;
use input::ConfigInputPlugin;
use window::ConfigWindowPlugin;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ConfigAudioPlugin, ConfigInputPlugin, ConfigWindowPlugin));
    }
}
