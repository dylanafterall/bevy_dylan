use bevy::{
    prelude::*,
};

// -----------------------------------------------------------------------------
pub struct CoreInputPlugin;

impl Plugin for CoreInputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(());
    }
}