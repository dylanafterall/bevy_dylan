mod components;
mod systems;

use crate::game::scene_manager::SceneState;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct ElevatorPlugin;

impl Plugin for ElevatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(())
            .add_systems(OnEnter(SceneState::Fourth), (systems::spawn_elevator,))
            .add_systems(
                Update,
                systems::move_elevator.run_if(in_state(SceneState::Fourth)),
            );
    }
}
