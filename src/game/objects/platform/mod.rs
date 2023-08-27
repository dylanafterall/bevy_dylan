mod systems;

use crate::AppState;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct PlatformPlugin;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(())

            .add_systems(OnEnter(AppState::Game), (
                systems::spawn_platform,
            ))
            .add_systems(OnExit(AppState::Game), (
                systems::despawn_platform,
            ));
    }
}