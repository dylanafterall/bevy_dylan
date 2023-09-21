use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct ConfigInputPlugin;

impl Plugin for ConfigInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(());
    }
}
