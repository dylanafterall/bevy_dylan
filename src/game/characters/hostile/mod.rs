use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct HostilePlugin;

impl Plugin for HostilePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(());
    }
}