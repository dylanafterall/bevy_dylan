use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct CreditsPlugin;

impl Plugin for CreditsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(());
    }
}