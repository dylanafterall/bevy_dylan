mod systems;

use crate::AppState;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(())

            .add_systems(OnEnter(AppState::Game), (
                systems::spawn_ball,
            ))
            .add_systems(OnExit(AppState::Game), (
                systems::despawn_ball,
            ));
    }
}