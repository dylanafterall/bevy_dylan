use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(());
    }
}