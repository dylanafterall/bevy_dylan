use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct ConfigAudioPlugin;

impl Plugin for ConfigAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(());
    }
}
