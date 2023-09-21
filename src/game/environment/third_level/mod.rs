use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct ThirdLevelPlugin;

impl Plugin for ThirdLevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(());
    }
}
