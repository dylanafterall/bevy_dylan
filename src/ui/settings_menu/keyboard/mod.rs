use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct KeyboardSettingsPlugin;

impl Plugin for KeyboardSettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(());
    }
}
