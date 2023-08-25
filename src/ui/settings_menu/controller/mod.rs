use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct ControllerSettingsPlugin;

impl Plugin for ControllerSettingsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(());
    }
}