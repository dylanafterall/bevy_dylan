use bevy::{
    prelude::*,
};

pub struct CoreAudioPlugin;

impl Plugin for CoreAudioPlugin {
    fn build(&self, app: &mut App) {
        app
            // plugins
            .add_plugins(());
    }
}