use bevy::{
    prelude::*,
};

// -----------------------------------------------------------------------------
pub struct CoreInputPlugin;

impl Plugin for CoreInputPlugin {
    fn build(&self, app: &mut App) {
        app
            // plugins
            .add_plugins(());
    }
}