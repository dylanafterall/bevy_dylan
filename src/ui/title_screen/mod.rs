use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct TitlePlugin;

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(());
    }
}
