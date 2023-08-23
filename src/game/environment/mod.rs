use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app
            // plugins
            .add_plugins(());
    }
}