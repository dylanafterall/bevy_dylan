use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app
            // plugins
            .add_plugins(());
    }
}