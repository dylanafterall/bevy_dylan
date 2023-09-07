mod systems;

use crate::game::scene_manager::SceneState;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct ConveyorBeltPlugin;

impl Plugin for ConveyorBeltPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(())

            .add_systems(OnEnter(SceneState::Fourth), (
                systems::spawn_conveyor_belt,
            ));
    }
}