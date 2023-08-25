use bevy::{
    prelude::*,
};

// -----------------------------------------------------------------------------
pub struct CoreAudioPlugin;

impl Plugin for CoreAudioPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(());
    }
}