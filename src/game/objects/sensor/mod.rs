mod systems;

use crate::AppState;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct SensorPlugin;

impl Plugin for SensorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(())

            .add_systems(OnEnter(AppState::Game), (
                systems::spawn_sensor,
            ));
    }
}