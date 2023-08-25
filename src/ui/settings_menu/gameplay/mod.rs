use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct GameplaySettingsPlugin;

impl Plugin for GameplaySettingsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(());
    }
}