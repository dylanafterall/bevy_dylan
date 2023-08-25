use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct VideoSettingsPlugin;

impl Plugin for VideoSettingsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(());
    }
}