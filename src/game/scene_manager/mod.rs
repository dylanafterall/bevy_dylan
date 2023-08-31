pub mod components;
mod systems;

use crate::AppState;

use bevy::prelude::*;

// -----------------------------------------------------------------------------
pub struct SceneManagerPlugin;

impl Plugin for SceneManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(())

            .add_state::<SceneState>()

            .add_systems(Update, (
                systems::handle_scene_transition,
            ))

            .add_systems(OnEnter(AppState::Game), (
                systems::spawn_initial_scene_sensors,
            ))
            .add_systems(OnEnter(SceneState::First), (
                systems::spawn_first_scene_sensors,
            ))
            .add_systems(OnEnter(SceneState::Second), (
                systems::spawn_second_scene_sensors,
            ))
            .add_systems(OnEnter(SceneState::Third), (
                systems::spawn_third_scene_sensors,
            ))

            .add_systems(OnExit(SceneState::First), (
                systems::despawn_scene_sensors,
            ))
            .add_systems(OnExit(SceneState::Second), (
                systems::despawn_scene_sensors,
            ))
            .add_systems(OnExit(SceneState::Third), (
                systems::despawn_scene_sensors,
            ));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SceneState {
    #[default]
    First,
    Second,
    Third,
}