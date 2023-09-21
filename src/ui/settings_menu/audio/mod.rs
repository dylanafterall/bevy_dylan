use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct AudioSettingsPlugin;

impl Plugin for AudioSettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(());
    }
}
