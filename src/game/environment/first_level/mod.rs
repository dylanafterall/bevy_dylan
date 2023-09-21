use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct FirstLevelPlugin;

impl Plugin for FirstLevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(());
    }
}
