mod systems;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(());
    }
}