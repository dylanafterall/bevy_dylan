use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // plugins
            .add_plugins(());
    }
}