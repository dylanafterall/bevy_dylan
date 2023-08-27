use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct SecondLevelPlugin;

impl Plugin for SecondLevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(());
    }
}